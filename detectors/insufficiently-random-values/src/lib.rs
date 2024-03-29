#![feature(rustc_private)]

extern crate rustc_hir;

use if_chain::if_chain;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use scout_audit_internal::{DetectorImpl, SorobanDetector as Detector};

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// This detector prevents the usage of timestamp/sequence number and modulo operator as a random number source.
    ///
    /// ### Why is this bad?
    /// The value of the block timestamp and block sequence can be manipulated by validators, which means they're not a secure source of randomness. Therefore, they shouldn't be used for generating random numbers, especially in the context of a betting contract where the outcomes of bets could be manipulated.
    ///
    /// ### Example
    /// ```rust
    ///  let pseudo_random = env.ledger().timestamp() % max_val;
    /// ```
    ///
    pub INSUFFICIENTLY_RANDOM_VALUES,
    Warn,
    ""
}

impl<'tcx> LateLintPass<'tcx> for InsufficientlyRandomValues {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if_chain! {
            if let ExprKind::Binary(op, lexp, _rexp) = expr.kind;
            if op.node == BinOpKind::Rem;
            if let ExprKind::MethodCall(path, _, _, _) = lexp.kind;
            if path.ident.as_str() == "timestamp" ||
                path.ident.as_str() == "sequence";
            then {
                Detector::InsufficientlyRandomValues.span_lint_and_help(
                    cx,
                    INSUFFICIENTLY_RANDOM_VALUES,
                    expr.span,
                    &format!("This expression seems to use ledger().{}() as a pseudo random number",path.ident.as_str()),
                );
            }
        }
    }
}
