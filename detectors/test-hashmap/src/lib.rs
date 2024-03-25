#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_data_structures;
extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

use std::collections::HashMap;

use rustc_ast::{
    visit::walk_expr as walk_ast_expr, visit::walk_item, visit::Visitor as AstVisitor, Item,
    ItemKind,
};
use rustc_hir::{
    def_id::LocalDefId,
    intravisit::{walk_body, walk_expr, FnKind, Visitor},
    Body, Expr, ExprKind, FnDecl, ImplItemKind, QPath,
};
use rustc_lint::{EarlyLintPass, LateContext, LateLintPass};
use rustc_session::{declare_lint, impl_lint_pass};
use rustc_span::Span;

dylint_linting::dylint_library!();

declare_lint! {
    pub TEST_HASMAP,
    Warn,
    "Warns about the use of the `hasmap` function"
}

pub struct TestHasmap;

pub struct TestHasmapEarly;

impl_lint_pass!(TestHasmap => [TEST_HASMAP]);
impl_lint_pass!(TestHasmapEarly => [TEST_HASMAP]);

pub struct TestHasmapFinder<'tcx, 'tcx_ref> {
    // A map to track which functions call which other functions
    cx: &'tcx_ref LateContext<'tcx>,
    caller: String,
    call_graph: HashMap<String, Vec<String>>,
}

#[allow(clippy::no_mangle_with_rust_abi)]
#[no_mangle]
pub fn register_lints(sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    // smoelius: Please keep the following `register_lints` calls sorted by crate name.
    lint_store.register_late_pass(|_| Box::new(TestHasmap));
    lint_store.register_early_pass(|| Box::new(TestHasmapEarly));
}

impl<'tcx> Visitor<'tcx> for TestHasmapFinder<'tcx, '_> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Call(mexpr, _) = &expr.kind {
            if let ExprKind::Path(path) = &mexpr.kind {
                if let QPath::TypeRelative(_, _) = &path {
                    let def = self.cx.qpath_res(path, mexpr.hir_id);
                    if let Some(def_id) = def.opt_def_id() {
                        let func_name = self.cx.tcx.def_path_str(def_id);
                        self.add_call_if_not_contains(self.caller.to_string(), func_name);
                    }
                }
            }
        }

        walk_expr(self, expr);
    }
}

impl<'tcx> LateLintPass<'tcx> for TestHasmap {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        fn_kind: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        _: Span,
        def_id: LocalDefId,
    ) {
        // Return if the function is a method or comes from a macro
        if !matches!(fn_kind, FnKind::Method(_, fnsig) if !fnsig.span.from_expansion()) {
            return;
        }

        let fn_name = cx.tcx.def_path_str(def_id);
        let mut visitor = TestHasmapFinder {
            cx,
            caller: fn_name.clone(),
            call_graph: HashMap::new(),
        };

        // Initialize the function entry in the call graph
        visitor.call_graph.entry(fn_name.clone()).or_default();

        walk_body(&mut visitor, body);

        println!("Function: {:?}", visitor.call_graph);
    }
}

impl<'tcx> TestHasmapFinder<'tcx, '_> {
    fn add_call_if_not_contains(&mut self, caller: String, callee: String) {
        if let Some(calls) = self.call_graph.get_mut(&caller) {
            if !calls.contains(&callee.to_string()) {
                calls.push(callee.to_string());
            }
        }
    }
}

impl EarlyLintPass for TestHasmapEarly {
    fn check_expr(&mut self, _: &rustc_lint::EarlyContext<'_>, ex: &rustc_ast::Expr) {
        walk_ast_expr(self, ex);
    }
}

impl<'tcx> AstVisitor<'tcx> for TestHasmapEarly {
    fn visit_expr(&mut self, ex: &'tcx rustc_ast::Expr) {
        println!("Expr: {:?}", ex);
        walk_ast_expr(self, ex);
    }
}
