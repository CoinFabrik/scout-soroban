#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]

extern crate rustc_hir;

extern crate rustc_span;

use scout_audit_clippy_utils::diagnostics::span_lint_and_help;

use std::collections::HashMap;
use std::collections::HashSet;
use std::vec;
use utils::{is_soroban_function, FunctionCallVisitor};

use rustc_hir::intravisit::walk_expr;
use rustc_hir::intravisit::Visitor;
use rustc_hir::Ty;
use rustc_hir::TyKind;
use rustc_hir::{Expr, ExprKind, QPath};
use rustc_lint::{LateContext, LateLintPass};
/* use rustc_middle::ty::{Ty, TyCtxt}; */
use rustc_span::def_id::DefId;
use rustc_span::Span;
//use scout_audit_clippy_utils::diagnostics::span_lint;

const LINT_MESSAGE: &str = "A function that is invoked changes storage without emiting an event";
const CANONICAL_FUNCTIONS_AMOUNT: usize = 10;

dylint_linting::impl_late_lint! {
    pub VERIFY_TRANSFER,
    Warn,
    "",
    VerifyTransfer::default(),
    {
        name: "Storage Changed without Emiting an Event in Token Interface implementations",
        long_message: " It can originate a problem when a canonical function does not emit an event expected by the contract's clients.",
        severity: "",
        help: "",
        vulnerability_class: "",
    }
}

#[derive(Default)]
struct VerifyTransfer {
    function_call_graph: HashMap<DefId, HashSet<DefId>>,
    checked_functions: HashSet<String>,
    eventless_storage_changers: HashSet<DefId>,
    defids_with_events: HashSet<DefId>,
    canonical_funcs_def_id: HashSet<DefId>,
}

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

impl<'tcx> LateLintPass<'tcx> for VerifyTransfer {
    fn check_crate_post(&mut self, cx: &LateContext<'tcx>) {
        if self.canonical_funcs_def_id.len() != CANONICAL_FUNCTIONS_AMOUNT {
            return;
        }

        let mut called_by_canonical_functions: HashSet<DefId> = HashSet::new();
        for cf in &self.canonical_funcs_def_id {
            check_defids(
                &self.function_call_graph,
                cf,
                &mut called_by_canonical_functions,
            )
        }

        let unsafe_set_storage: HashSet<DefId> = called_by_canonical_functions
            .intersection(&self.eventless_storage_changers)
            .cloned()
            .collect();

        for func in self.function_call_graph.keys() {
            if is_soroban_function(cx, &self.checked_functions, func) {
                let emits_event_in_flow = check_events_children(
                    &self.function_call_graph,
                    func,
                    &self.defids_with_events,
                );

                let calls_unsafe_storage_setter = check_storage_setters_calls(
                    &self.function_call_graph,
                    func,
                    &unsafe_set_storage,
                );

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

        if verify_token_interface_function(fn_name.clone(), fn_decl.inputs) {
            self.canonical_funcs_def_id.insert(def_id);
        }
        let mut verify_transfer_visitor = VerifyTransferVisitor {
            cx,
            is_storage_changer: false,
            emits_event: false,
        };

        verify_transfer_visitor.visit_body(body);

        if verify_transfer_visitor.is_storage_changer && !verify_transfer_visitor.emits_event {
            self.eventless_storage_changers.insert(def_id);
        }

        if verify_transfer_visitor.emits_event {
            self.defids_with_events.insert(def_id);
        }
    }
}

struct VerifyTransferVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    is_storage_changer: bool,
    emits_event: bool,
}

impl<'a, 'tcx> Visitor<'tcx> for VerifyTransferVisitor<'a, 'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
        if let ExprKind::MethodCall(path, receiver, _, _) = expr.kind {
            let name = path.ident.name.as_str();

            let receiver_type = self.cx.typeck_results().node_type(receiver.hir_id);
            if name == "events" {
                self.emits_event = true;
            }
            if name == "set" && receiver_type.to_string() == "soroban_sdk::storage::Persistent" {
                self.is_storage_changer = true;
            }
        }
        walk_expr(self, expr);
    }
}

fn check_params(fn_params: &[Ty], expected_types: Vec<String>) -> bool {
    let mut param_types: Vec<String> = Vec::new();
    for i in fn_params {
        if let TyKind::Path(QPath::Resolved(_, path)) = i.kind {
            let param_type = path.segments[0].ident.to_string();
            param_types.push(param_type.clone());
        }
    }
    param_types == expected_types
}

fn verify_token_interface_function(fn_name: String, fn_params: &[Ty]) -> bool {
    let function = fn_name.split("::").last().unwrap();
    let types: Vec<String> = match function {
        "allowance" => ["Env", "Address", "Address"]
            .iter()
            .map(|&s| s.to_string())
            .collect(),
        "approve" => ["Env", "Address", "Address", "i128", "u32"]
            .iter()
            .map(|&s| s.to_string())
            .collect(),
        "balance" => ["Env", "Address"].iter().map(|&s| s.to_string()).collect(),
        "transfer" => ["Env", "Address", "Address", "i128"]
            .iter()
            .map(|&s| s.to_string())
            .collect(),
        "transfer_from" => ["Env", "Address", "Address", "Address", "i128"]
            .iter()
            .map(|&s| s.to_string())
            .collect(),
        "burn" => ["Env", "Address", "i128"]
            .iter()
            .map(|&s| s.to_string())
            .collect(),
        "burn_from" => ["Env", "Address", "Address", "i128"]
            .iter()
            .map(|&s| s.to_string())
            .collect(),
        "decimals" => ["Env"].iter().map(|&s| s.to_string()).collect(),
        "name" => ["Env"].iter().map(|&s| s.to_string()).collect(),
        "symbol" => ["Env"].iter().map(|&s| s.to_string()).collect(),
        _ => return false,
    };
    check_params(fn_params, types)
}
