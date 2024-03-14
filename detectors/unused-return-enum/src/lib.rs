#![feature(rustc_private)]

extern crate rustc_hir;
extern crate rustc_span;

use if_chain::if_chain;
use rustc_hir::intravisit::{walk_expr, FnKind, Visitor};
use rustc_hir::{Body, Expr, ExprKind, FnDecl, FnRetTy, MatchSource, QPath, TyKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{def_id::LocalDefId, sym, Span};
use scout_audit_internal::Detector;

dylint_linting::declare_late_lint! {
    pub UNUSED_RETURN_ENUM,
    Warn,
    Detector::UnusedReturnEnum.get_lint_message()
}

#[derive(Debug)]
struct CounterVisitor {
    count_err: u32,
    count_ok: u32,
    found_try: bool,
    found_return: bool,
    span: Vec<Span>,
}

impl<'tcx> Visitor<'tcx> for CounterVisitor {
    fn visit_expr(&mut self, expr: &'tcx Expr) {
        match expr.kind {
            ExprKind::Call(function, _) => {
                if_chain! {
                    if let ExprKind::Path(QPath::Resolved(_, path)) = &function.kind;
                    if let Some(last_segment) = path.segments.last();
                    then {
                        if last_segment.ident.name == sym::Ok {
                            self.count_ok += 1;
                            self.span.push(expr.span);
                        } else if last_segment.ident.name == sym::Err {
                            self.count_err += 1;
                            self.span.push(expr.span);
                        }
                    }
                }
            }
            ExprKind::Ret(Some(return_value)) => {
                if_chain! {
                    if let ExprKind::Call(function, _) = &return_value.kind;
                    if let ExprKind::Path(QPath::Resolved(_, path)) = &function.kind;
                    if let Some(last_segment) = path.segments.last();
                    then {
                        if last_segment.ident.name != sym::Ok && last_segment.ident.name != sym::Err {
                            self.found_return = true;
                        }
                    }
                }
            }
            ExprKind::Match(_, _, MatchSource::TryDesugar(_)) => {
                self.found_try = true;
            }
            _ => {}
        }
        walk_expr(self, expr);
    }
}
impl<'tcx> LateLintPass<'tcx> for UnusedReturnEnum {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        fnkind: FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        _: Span,
        _: LocalDefId,
    ) {
        // If the function is not a method, or it comes from a macro expansion, we ignore it
        if !matches!(fnkind, FnKind::Method(_, fnsig) if !fnsig.span.from_expansion()) {
            return;
        }

        // If the function returns a type different from Result, we ignore it
        if_chain! {
            if let FnRetTy::Return(return_type) = &decl.output;
            if let TyKind::Path(qpath) = &return_type.kind;
            if let QPath::Resolved(_ty, path) = qpath;
            if let Some(path_segment) = path.segments.last();
            if path_segment.ident.name != sym::Result;
            then {
                return;
            }
        }

        let mut visitor = CounterVisitor {
            count_ok: 0,
            count_err: 0,
            found_try: false,
            found_return: false,
            span: Vec::new(),
        };

        walk_expr(&mut visitor, body.value);

        if !visitor.found_return && (visitor.count_err == 0 || visitor.count_ok == 0) {
            visitor.span.iter().for_each(|span| {
                Detector::UnusedReturnEnum.span_lint_and_help(
                    cx,
                    UNUSED_RETURN_ENUM,
                    *span,
                    "If any of the variants (Ok/Err) is not used, the code could be simplified or it could imply a bug",
                );
            });
        }
    }
}
