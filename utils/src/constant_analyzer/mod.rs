use std::collections::HashSet;

extern crate rustc_hir;
extern crate rustc_lint;

use clippy_utils::consts::constant;
use rustc_hir::{
    def::{DefKind, Res},
    intravisit::{walk_expr, Visitor},
    Expr, ExprKind, HirId, QPath, StmtKind,
};
use rustc_lint::LateContext;

pub struct ConstantAnalyzer<'a, 'tcx> {
    pub cx: &'a LateContext<'tcx>,
    pub constants: HashSet<HirId>,
}

impl<'a, 'tcx> ConstantAnalyzer<'a, 'tcx> {
    fn is_qpath_constant(&self, path: &QPath) -> bool {
        if let QPath::Resolved(_, path) = path {
            match path.res {
                Res::Def(def_kind, _) => matches!(
                    def_kind,
                    DefKind::AnonConst
                        | DefKind::AssocConst
                        | DefKind::Const
                        | DefKind::InlineConst
                ),
                Res::Local(hir_id) => self.constants.contains(&hir_id),
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_expr_constant(&self, expr: &Expr<'tcx>) -> bool {
        // Evaluate the expression as a compile-time constant
        if constant(self.cx, self.cx.typeck_results(), expr).is_some() {
            return true;
        }

        // If the expression is not a constant, verify if it is known at compile time
        match expr.kind {
            ExprKind::Array(expr_array) => expr_array
                .iter()
                .all(|expr_in_array| self.is_expr_constant(expr_in_array)),
            ExprKind::Binary(_, left_expr, right_expr) => {
                self.is_expr_constant(left_expr) && self.is_expr_constant(right_expr)
            }
            ExprKind::Cast(cast_expr, _) => self.is_expr_constant(cast_expr),
            ExprKind::Field(field_expr, _) => self.is_expr_constant(field_expr),
            // TODO: array with just index checking
            ExprKind::Index(array_expr, index_expr, _) => {
                self.is_expr_constant(array_expr) && self.is_expr_constant(index_expr)
            }
            ExprKind::Lit(_) => true,
            ExprKind::Path(qpath_expr) => self.is_qpath_constant(&qpath_expr),
            ExprKind::Repeat(repeat_expr, _) => self.is_expr_constant(repeat_expr),
            ExprKind::Struct(_, expr_fields, _) => expr_fields
                .iter()
                .all(|field_expr| self.is_expr_constant(field_expr.expr)),
            _ => false,
        }
    }

    pub fn is_compile_time_known(&self, expr: &Expr<'tcx>) -> bool {
        self.is_expr_constant(expr)
    }
}

impl<'a, 'tcx> Visitor<'tcx> for ConstantAnalyzer<'a, 'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Block(block, _) = expr.kind {
            for stmt in block.stmts {
                if let StmtKind::Let(local) = stmt.kind {
                    if let Some(init) = local.init {
                        if self.is_expr_constant(init) {
                            self.constants.insert(local.pat.hir_id);
                        }
                    }
                }
            }
        }
        walk_expr(self, expr);
    }
}
