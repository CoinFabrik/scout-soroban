#![feature(rustc_private)]

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;

use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    Expr, ExprKind,
};
use rustc_lint::{LateContext, LateLintPass};

use rustc_span::Span;

use std::collections::HashMap;
use std::collections::HashSet;
use std::vec;
use utils::{is_soroban_function, verify_token_interface_function, FunctionCallVisitor};

use rustc_span::def_id::DefId;

const LINT_MESSAGE: &str =
    "Storage change that involves token manipulation is performed without emiting an event";
const CANONICAL_FUNCTIONS_AMOUNT: usize = 10;

dylint_linting::impl_late_lint! {
    pub VERIFY_TRANSFER,
    Warn,
    "",
    TokenInterfaceEvents::default(),
    {
        name: "Storage Changed without Emiting an Event in Token Interface implementations",
        long_message: " It can originate a problem when a canonical function does not emit an event expected by the contract's clients.",
        severity: "",
        help: "",
        vulnerability_class: "",
    }
}

#[derive(Default)]
struct TokenInterfaceEvents {
    function_call_graph: HashMap<DefId, HashSet<DefId>>,
    checked_functions: HashSet<String>,
    eventless_storage_changers: HashSet<DefId>,
    defids_with_events: HashSet<DefId>,
    canonical_funcs_def_id: HashSet<DefId>,
    impl_token_interface_trait: bool,
}

/// Used to add to a HashSet all the DefIds of the functions that are called starting from a specific parent in the function call graph.
/// # Params:
///     - fcg: function call graph that is used as reference.
///     - parent: the item from which the analysis starts.
///     - defids: the HashSet where all the defids found in the tree are collected.

fn check_defids(fcg: &HashMap<DefId, HashSet<DefId>>, parent: &DefId, defids: &mut HashSet<DefId>) {
    let children = fcg.get(parent);
    if children.is_some() {
        for c in children.unwrap() {
            if !defids.contains(c) {
                defids.insert(*c);
                check_defids(fcg, c, defids);
            }
        }
    }
}

/// Used to verify if, starting from a specific parent in the call graph, an event is emitted at any point of the flow.
/// # Params:
///     - fcg: function call graph
///     - parent: the item from which the analysis starts.
///     - check_against: a HashSet that is used to compare the defids. This HashSet is supposed to contain all the defids of the functions that emit events (collected by the `visit_expr` and `check_func` functions).
fn check_events_children(
    fcg: &HashMap<DefId, HashSet<DefId>>,
    parent: &DefId,
    check_against: &HashSet<DefId>,
) -> bool {
    if check_against.contains(parent) {
        return true;
    }
    let children = fcg.get(parent);
    if children.is_some() {
        for c in children.unwrap() {
            if check_against.contains(c) || check_events_children(fcg, c, check_against) {
                return true;
            }
        }
    }
    false
}

/// Used to verify if, starting from a specific parent in the call graph, a function that sets storage in a considered "unsafe" way is called in any part of its flow.
/// # Params:
///     - fcg: function call graph
///     - func: the defid from which the analysis starts.
///     - unsafe_set_storage: a HashSet that is used to compare the defids. This HashSet is supposed to contain all the defids of the functions that are considered "unsafe storage setters".
fn check_storage_setters_calls(
    fcg: &HashMap<DefId, HashSet<DefId>>,
    func: &DefId,
    unsafe_set_storage: &HashSet<DefId>,
) -> bool {
    if unsafe_set_storage.contains(func) {
        return true;
    }
    let children = fcg.get(func);
    if children.is_some() {
        for c in children.unwrap() {
            if unsafe_set_storage.contains(c)
                || check_storage_setters_calls(fcg, c, unsafe_set_storage)
            {
                return true;
            }
        }
    }
    false
}

impl<'tcx> LateLintPass<'tcx> for TokenInterfaceEvents {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx rustc_hir::Item<'tcx>) {
        if let rustc_hir::ItemKind::Impl(impl_block) = item.kind {
            if let Some(trait_ref) = impl_block.of_trait {
                let trait_def_id = trait_ref.path.res.def_id();
                let trait_name = cx.tcx.def_path_str(trait_def_id);

                if trait_name == "soroban_sdk::token::TokenInterface" {
                    self.impl_token_interface_trait = true;
                }
            }
        }
    }

    fn check_crate_post(&mut self, cx: &LateContext<'tcx>) {
        // Verify if the contract implements the token interface. If all of the canonical functions have not been detected, we assume it is not.
        if self.canonical_funcs_def_id.len() != CANONICAL_FUNCTIONS_AMOUNT
            && !self.impl_token_interface_trait
        {
            return;
        }

        // Get the defids of every function that is called either directly or indirectly by all of the functions that are a part of the token interface.
        let mut called_by_canonical_functions: HashSet<DefId> = HashSet::new();
        for cf in &self.canonical_funcs_def_id {
            check_defids(
                &self.function_call_graph,
                cf,
                &mut called_by_canonical_functions,
            )
        }

        let can_funcs: HashSet<DefId> = called_by_canonical_functions
            .union(&self.canonical_funcs_def_id)
            .cloned()
            .collect();

        // Get the functions that are called directly or indirectly by the token interface functions and, at the same time, are storage setters that do not emit an event.
        let unsafe_set_storage: HashSet<DefId> = can_funcs
            .intersection(&self.eventless_storage_changers)
            .cloned()
            .collect();

        // Emit the alerts for the considered "unsafe" functions.
        for func in self.function_call_graph.keys() {
            // Only take into account those functions that are public and exposed in a soroban contract (entrypoints that can be called externally). We do not advise on functions that are used auxiliarily.
            if is_soroban_function(cx, &self.checked_functions, func)
                || (self.impl_token_interface_trait && self.canonical_funcs_def_id.contains(func))
            {
                // Verify if the function itself or the ones it calls (directly or indirectly) emit an event at any point of the flow.
                let emits_event_in_flow = check_events_children(
                    &self.function_call_graph,
                    func,
                    &self.defids_with_events,
                );

                // Verify if the function itself or the ones it calls (directly or indirectly) call an unsafe storage setter at any point of the flow.
                let calls_unsafe_storage_setter = check_storage_setters_calls(
                    &self.function_call_graph,
                    func,
                    &unsafe_set_storage,
                );

                // If both conditions are met, emit an warning.
                if !emits_event_in_flow && calls_unsafe_storage_setter {
                    span_lint_and_help(
                        cx,
                        VERIFY_TRANSFER,
                        cx.tcx.hir().span_if_local(*func).unwrap(),
                        LINT_MESSAGE,
                        /* cx.tcx.hir().span_if_local(r) */ None,
                        "",
                    );
                }
            }
        }
    }

    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        fn_decl: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        span: Span,
        local_def_id: rustc_span::def_id::LocalDefId,
    ) {
        let def_id = local_def_id.to_def_id();
        self.checked_functions.insert(cx.tcx.def_path_str(def_id));

        if span.from_expansion() {
            return;
        }

        let fn_name = cx.tcx.def_path_str(def_id);

        let mut function_call_visitor =
            FunctionCallVisitor::new(cx, def_id, &mut self.function_call_graph);
        function_call_visitor.visit_body(body);

        // If the function is part of the token interface, I store its defid.
        if verify_token_interface_function(fn_name.clone(), fn_decl.inputs, fn_decl.output) {
            self.canonical_funcs_def_id.insert(def_id);
        }
        let mut verify_transfer_visitor = TokenInterfaceEventsVisitor {
            cx,
            is_storage_changer: false,
            emits_event: false,
        };

        verify_transfer_visitor.visit_body(body);

        // If the function modifies the storage and does not emit event, we keep record of its defid as an eventless storage changer.
        if verify_transfer_visitor.is_storage_changer && !verify_transfer_visitor.emits_event {
            self.eventless_storage_changers.insert(def_id);
        }

        // If the function emits an event, we storage its defid.
        if verify_transfer_visitor.emits_event {
            self.defids_with_events.insert(def_id);
        }
    }
}

struct TokenInterfaceEventsVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    is_storage_changer: bool,
    emits_event: bool,
}

impl<'a, 'tcx> Visitor<'tcx> for TokenInterfaceEventsVisitor<'a, 'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
        if let ExprKind::MethodCall(path, receiver, _, _) = expr.kind {
            let name = path.ident.name.as_str();

            let receiver_type = self.cx.typeck_results().node_type(receiver.hir_id);

            // verify if it is an event emission
            if name == "events" {
                self.emits_event = true;
            }

            // verify if it is a storage change
            if (name == "set" || name == "update" || name == "remove")
                && (receiver_type.to_string() == "soroban_sdk::storage::Instance"
                    || receiver_type.to_string() == "soroban_sdk::storage::Persistent"
                    || receiver_type.to_string() == "soroban_sdk::storage::Temporary")
            {
                self.is_storage_changer = true;
            }
        }
        walk_expr(self, expr);
    }
}
