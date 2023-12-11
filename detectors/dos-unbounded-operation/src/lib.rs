#![feature(rustc_private)]
#![recursion_limit = "256"]
extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use if_chain::if_chain;
use rustc_hir::def::DefKind;
use rustc_hir::StmtKind;
use rustc_hir::{
    def::Res,
    intravisit::{walk_body, walk_expr, FnKind, Visitor},
    Body, Expr, ExprKind, FnDecl, HirId, LangItem, MatchSource, QPath,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{def_id::LocalDefId, Span};
use scout_audit_internal::Detector;

dylint_linting::declare_late_lint!(
    pub DOS_UNBOUNDED_OPERATION,
    Warn,
    "This loop seems to do not have a fixed number of iterations"
);

struct ForLoopVisitor {
    constants: Vec<HirId>,
    span_constant: Vec<Span>,
}

impl ForLoopVisitor {
    fn is_qpath_constant(&self, path: &QPath) -> bool {
        if let QPath::Resolved(_, path) = path {
            // We search the path, if it has been previously defined or is a constant then we are good
            match path.res {
                Res::Local(local_def_id) => self.constants.contains(&local_def_id),
                Res::Def(def_kind, _) => matches!(
                    def_kind,
                    DefKind::Const
                        | DefKind::AnonConst
                        | DefKind::InlineConst
                        | DefKind::AssocConst
                ),
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_expr_constant(&self, expr: &Expr) -> bool {
        match expr.kind {
            ExprKind::Lit(_) => true,
            ExprKind::Array(array) => array.iter().all(|expr| self.is_expr_constant(expr)),
            ExprKind::Field(expr, _) => self.is_expr_constant(expr),
            ExprKind::Path(path) => self.is_qpath_constant(&path),
            ExprKind::MethodCall(_, expr, _, _) => self.is_expr_constant(expr),
            ExprKind::Binary(_, left_expr, right_expr) => {
                self.is_expr_constant(left_expr) && self.is_expr_constant(right_expr)
            }
            ExprKind::Struct(_, fields, _) => {
                fields.iter().all(|field| self.is_expr_constant(field.expr))
            }
            ExprKind::Repeat(expr, _) => self.is_expr_constant(expr),
            ExprKind::Cast(expr, _) => self.is_expr_constant(expr),
            ExprKind::Index(array, value, _) => {
                self.is_expr_constant(array) && self.is_expr_constant(value)
            }
            _ => false,
        }
    }
}

impl<'tcx> Visitor<'tcx> for ForLoopVisitor {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        // Constant detection
        if let ExprKind::Block(a, _) = expr.kind {
            a.stmts.iter().for_each(|func| {
                if let StmtKind::Local(sd) = func.kind {
                    if sd.init.is_some() && self.is_expr_constant(sd.init.as_ref().unwrap()) {
                        self.constants.push(sd.pat.hir_id);
                    }
                }
            })
        }

        // For loop detection
        if_chain! {
            // Check if the expression is a for loop
            if let ExprKind::Match(match_expr, _, MatchSource::ForLoopDesugar) = expr.kind;
            if let ExprKind::Call(call_func, call_args) = match_expr.kind;
            // Check the function call
            if let ExprKind::Path(qpath) = &call_func.kind;
            if let QPath::LangItem(LangItem::IntoIterIntoIter, _, _) = qpath;
            // Check if a Range is used
            if let ExprKind::Struct(struct_lang_item, struct_expr, _) = call_args.first().unwrap().kind;
            if let QPath::LangItem(
                LangItem::Range | LangItem::RangeInclusiveStruct | LangItem::RangeInclusiveNew,
                _,
                _,
            ) = struct_lang_item;
            // Get the start and end of the range
            if let Some(start_expr) = struct_expr.first();
            if let Some(end_expr) = struct_expr.last();
            then {
                if !self.is_expr_constant(start_expr.expr) || !self.is_expr_constant(end_expr.expr) {
                    self.span_constant.push(expr.span);
                }
            }
        }
        walk_expr(self, expr);
    }
}

impl<'tcx> LateLintPass<'tcx> for DosUnboundedOperation {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        _: Span,
        _: LocalDefId,
    ) {
        let mut visitor = ForLoopVisitor {
            span_constant: Vec::new(),
            constants: Vec::new(),
        };

        walk_body(&mut visitor, body);

        for span in visitor.span_constant {
            Detector::DosUnboundedOperation.span_lint_and_help(
                cx,
                DOS_UNBOUNDED_OPERATION,
                span,
                "This loop seems to do not have a fixed number of iterations",
            );
        }
    }
}
