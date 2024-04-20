#![feature(rustc_private)]

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_span;

use if_chain::if_chain;
use rustc_errors::Applicability;
use rustc_hir::{
    intravisit::{walk_expr, FnKind, Visitor},
    Body, Expr, ExprKind, FnDecl,
};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::{def_id::LocalDefId, Span};
use scout_audit_clippy_utils::diagnostics::span_lint_and_sugg;
use utils::{get_receiver_ident_name, is_soroban_map};

const LINT_MESSAGE: &str = "Unsafe access on Map, method could panic.";
const UNSAFE_GET_METHODS: [&str; 3] = ["get", "get_unchecked", "try_get_unchecked"];

dylint_linting::declare_late_lint! {
    pub UNSAFE_MAP_GET,
    Warn,
    LINT_MESSAGE,
    {
        name: "Unsafe Map Get",
        long_message: "This vulnerability class pertains to the inappropriate usage of the get method for Map in soroban",
        severity: "Medium",
        help: "https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unsafe-map-get",
        vulnerability_class: "Validations and error handling",
    }
}

struct UnsafeMapGetVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
}

impl UnsafeMapGetVisitor<'_, '_> {
    fn get_first_arg_str(&self, arg: Option<&Expr<'_>>) -> String {
        arg.and_then(|arg| self.cx.sess().source_map().span_to_snippet(arg.span).ok())
            .unwrap_or_default()
    }
}

impl<'a, 'tcx> Visitor<'tcx> for UnsafeMapGetVisitor<'a, 'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
        if_chain! {
            if let ExprKind::MethodCall(path_segment, receiver, args, _) = &expr.kind;
            if UNSAFE_GET_METHODS.contains(&path_segment.ident.as_str());
            if is_soroban_map(self.cx, self.cx.typeck_results().node_type(receiver.hir_id));
            then {
                let receiver_ident_name = get_receiver_ident_name(receiver);
                let first_arg_str = self.get_first_arg_str(args.first());
                span_lint_and_sugg(
                    self.cx,
                    UNSAFE_MAP_GET,
                    expr.span,
                    LINT_MESSAGE,
                    &format!("Using `{}` on a Map is unsafe as it could panic, please use", path_segment.ident),
                    format!("{}.try_get({})", receiver_ident_name, first_arg_str),
                    Applicability::MaybeIncorrect,
                );
            }
        }
        walk_expr(self, expr);
    }
}

impl<'tcx> LateLintPass<'tcx> for UnsafeMapGet {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        _: LocalDefId,
    ) {
        // If the function comes from a macro expansion, we don't want to analyze it.
        if span.from_expansion() {
            return;
        }

        let mut visitor = UnsafeMapGetVisitor { cx };

        walk_expr(&mut visitor, body.value);
    }
}
