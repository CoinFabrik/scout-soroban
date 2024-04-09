#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::{
    def_id::LocalDefId,
    intravisit::{walk_expr, FnKind, Visitor},
    Body, Expr, ExprKind, FnDecl, HirId,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{sym, Span, Symbol};
use scout_audit_internal::{DetectorImpl, SorobanDetector as Detector};

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// Checks for usage of `unwrap`
    ///
    /// ### Why is this bad?
    /// `unwrap` might panic if the result value is an error or `None`.
    ///
    /// ### Example
    /// ```rust
    /// // example code where a warning is issued
    /// fn main() {
    ///    let result = result_fn().unwrap("error");
    /// }
    ///
    /// fn result_fn() -> Result<u8, Error> {
    ///     Err(Error::new(ErrorKind::Other, "error"))
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code that does not raise a warning
    /// fn main() {
    ///    let result = if let Ok(result) = result_fn() {
    ///       result
    ///   }
    /// }
    ///
    /// fn result_fn() -> Result<u8, Error> {
    ///     Err(Error::new(ErrorKind::Other, "error"))
    /// }
    /// ```
    pub UNSAFE_UNWRAP,
    Warn,
    ""
}

struct UnsafeUnwrapVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    checked_exprs: Vec<HirId>,
    unwrap_spans: Vec<Span>,
}

impl<'tcx> LateLintPass<'tcx> for UnsafeUnwrap {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        _: LocalDefId,
    ) {
        if span.from_expansion() {
            return;
        }

        let asd = cx.tcx.all_diagnostic_items(());
        println!("diag items {:?}", asd);

        impl<'a, 'tcx> Visitor<'tcx> for UnsafeUnwrapVisitor<'a, 'tcx> {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if let ExprKind::If(condition, block, _) = &expr.kind {
                    if let ExprKind::MethodCall(path_segment, args, _, _) =
                        &condition.peel_drop_temps().kind
                    {
                        if path_segment.ident.name == Symbol::intern("is_err") {
                            if let ExprKind::Block(block, _) = block.kind {

                            }
                        }
                    }
                }
                if let ExprKind::MethodCall(path_segment, receiver, _, _) = &expr.kind {
                    if path_segment.ident.name == sym::unwrap
                        && !self.checked_exprs.contains(&receiver.hir_id)
                    {
                        self.unwrap_spans.push(expr.span);
                    }
                }
                walk_expr(self, expr);
            }
        }

        let mut visitor = UnsafeUnwrapVisitor {
            cx,
            unwrap_spans: Vec::new(),
            checked_exprs: Vec::new(),
        };

        walk_expr(&mut visitor, body.value);

        visitor.unwrap_spans.iter().for_each(|span| {
            Detector::UnsafeUnwrap.span_lint_and_help(
                cx,
                UNSAFE_UNWRAP,
                *span,
                "Please, use a custom error instead of `unwrap`",
            );
        });
    }
}
