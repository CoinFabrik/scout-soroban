#![feature(rustc_private)]

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint_and_help;

use rustc_hir::{intravisit::Visitor, ImplItem, ItemKind, Node};
use rustc_lint::{LateContext, LateLintPass};

use rustc_errors::MultiSpan;
use rustc_span::Span;

use std::collections::HashSet;
use std::vec;
use std::{
    collections::HashMap,
    ops::{Div, Mul},
};
use utils::FunctionCallVisitor;

use rustc_span::def_id::DefId;

const LINT_MESSAGE: &str =
    "This contract seems like a Token, consider implementing the Token Interface trait";
const CANONICAL_FUNCTIONS_AMOUNT: u16 = 10;
const INCLUDED_FUNCTIONS_THRESHOLD: u16 = 50;

dylint_linting::impl_late_lint! {
    pub TOKEN_INTERFACE_INFERENCE,
    Warn,
    "",
    TokenInterfaceInference::default(),
    {
        name: "The contract appears to be a Token but does not implement the Token Interface trait.",
        long_message: "Implementing the trait can improve the compliance of the token interface.",
        severity: "",
        help: "",
        vulnerability_class: "",
    }
}

#[derive(Default)]
struct TokenInterfaceInference {
    function_call_graph: HashMap<DefId, HashSet<DefId>>,
    checked_functions: HashSet<String>,
    canonical_funcs_def_id: HashSet<DefId>,
    impl_token_interface_trait: bool,
    detected_canonical_functions_count: u16,
    funcs_spans: Vec<Span>,
}

impl<'tcx> LateLintPass<'tcx> for TokenInterfaceInference {
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
        // Verify if the contract implements the token interface.
        if self.impl_token_interface_trait {
            return;
        }

        if self
            .detected_canonical_functions_count
            .mul(100)
            .div(CANONICAL_FUNCTIONS_AMOUNT)
            >= INCLUDED_FUNCTIONS_THRESHOLD
        {
            span_lint_and_help(
                cx,
                TOKEN_INTERFACE_INFERENCE,
                MultiSpan::from_spans(self.funcs_spans.clone()),
                LINT_MESSAGE,
                None,
                "",
            );
        }
    }

    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _fn_decl: &'tcx rustc_hir::FnDecl<'tcx>,
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

        let fn_name_span = if let Some(node) = cx.tcx.hir().get_if_local(def_id) {
            match node {
                Node::Item(item) => Some(item.ident.span),
                Node::ImplItem(impl_item) => Some(impl_item.ident.span),
                _ => None,
            }
        } else {
            None
        };

        let mut function_call_visitor =
            FunctionCallVisitor::new(cx, def_id, &mut self.function_call_graph);
        function_call_visitor.visit_body(body);

        // If the function is part of the token interface, I store its defid.
        if verify_token_interface_function_similarity(fn_name.clone()) {
            self.detected_canonical_functions_count += 1;
            self.canonical_funcs_def_id.insert(def_id);
            if let Some(span) = fn_name_span {
                self.funcs_spans.push(span);
            }
        }
    }
}

fn verify_token_interface_function_similarity(fn_name: String) -> bool {
    let canonical_functions_formatted = [
        String::from("allowance"),
        String::from("approve"),
        String::from("balance"),
        String::from("transfer"),
        String::from("transferfrom"),
        String::from("burn"),
        String::from("burnfrom"),
        String::from("decimals"),
        String::from("name"),
        String::from("symbol"),
    ];
    let function_name = String::from(fn_name.split("::").last().unwrap());
    let formatted_name: String = function_name
        .to_lowercase()
        .replace("_", "")
        .split_whitespace()
        .collect();

    canonical_functions_formatted.contains(&formatted_name)
}
