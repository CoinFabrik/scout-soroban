#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::{
    intravisit::{walk_expr, Visitor},
    Expr, ExprKind,
};
use rustc_lint::LateLintPass;
use rustc_span::{Span, Symbol};
use scout_audit_internal::Detector;

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
    pub UNSAFE_BLOCK,
    Warn,
    Detector::UnsafeUnwrap.get_lint_message()
}

impl<'tcx> LateLintPass<'tcx> for UnsafeUnwrap {
    fn check_fn(
        &mut self,
        cx: &rustc_lint::LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: rustc_span::Span,
        _: rustc_hir::def_id::LocalDefId,
    ) {
        struct UnsafeBlockVisitor {
            unsafe_blocks: Vec<Option<Span>>,
        }

        impl<'tcx> Visitor<'tcx> for UnsafeBlockVisitor {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                match expr.kind {
                    ExprKind::Block(ref block, _) => {
                        if block.rules == rustc_hir::BlockCheckMode::UnsafeBlock(
                            rustc_hir::UnsafeSource::UserProvided,
                        ) {
                            self.unsafe_blocks.push(Some(expr.span));
                        } else {
                            self.unsafe_blocks.push(None);
                        }
                    }
                    _ => {}
                }
                walk_expr(self, expr);
            }
        }

        let mut visitor = UnsafeBlockVisitor {
            unsafe_blocks: Vec::new(),
        };

        walk_expr(&mut visitor, body.value);

        visitor.unsafe_blocks.iter().for_each(|span| {
            if let Some(span) = span {
                Detector::UnsafeUnwrap.span_lint(
                    cx,
                    UNSAFE_BLOCK,
                    *span
                );
            }
        });
        
    }
}
