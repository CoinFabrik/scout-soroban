#![feature(rustc_private)]
#![recursion_limit = "256"]
extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use if_chain::if_chain;
use rustc_hir::StmtKind;
use rustc_hir::{
    def::Res,
    intravisit::{walk_body, walk_expr, walk_local, FnKind, Visitor},
    Block, Body, Expr, ExprKind, FnDecl, HirId, LangItem, Local, MatchSource, QPath,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{def_id::LocalDefId, Span};
use scout_audit_internal::Detector;

dylint_linting::impl_late_lint!(
    pub DOS_UNBOUNDED_OPERATION,
    Warn,
    "This loop seems to do not have a fixed number of iterations",
    DosUnboundedOperation::new()
);

struct SharedState {
    range_variables: Vec<HirId>,
    constants: Vec<HirId>,
}

struct DosUnboundedOperation {
    shared_state: SharedState,
}

impl DosUnboundedOperation {
    pub fn new() -> Self {
        Self {
            shared_state: SharedState {
                constants: Vec::new(),
                range_variables: Vec::new(),
            },
        }
    }
}

struct ForLoopVisitor<'a> {
    shared_state: &'a mut SharedState,
    span_constant: Vec<Span>,
}

impl<'tcx> Visitor<'tcx> for ForLoopVisitor<'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Block(a, _) = expr.kind {
            a.stmts.iter().for_each(|func| {
                if let StmtKind::Local(sd) = func.kind {
                    if sd.init.is_some() {
                        self.shared_state.constants.push(sd.pat.hir_id);
                    }
                }
            })
        }

        if_chain! {
            // Check if the expression is a for loop
            if let ExprKind::Match(match_expr, _, match_source) = expr.kind;
            if match_source == MatchSource::ForLoopDesugar;
            if let ExprKind::Call(call_func, call_args) = match_expr.kind;
            // Check the function call
            if let ExprKind::Path(qpath) = &call_func.kind;
            if let QPath::LangItem(item, _, _) = qpath;
            if item == &LangItem::IntoIterIntoIter;
            // Check if a Range is used
            if let ExprKind::Struct(struct_lang_item, struct_expr, _) = call_args.first().unwrap().kind;
            if let QPath::LangItem(range_lang_item, _, _) = struct_lang_item;
            if range_lang_item == &LangItem::Range || range_lang_item == &LangItem::RangeInclusiveStruct || range_lang_item == &LangItem::RangeInclusiveNew;
            // Get the start and end of the range
            if let Some(start_expr) = struct_expr.first();
            if let Some(end_expr) = struct_expr.last();
            then {
                // TODO we do not need to save the range variables, we can just check if the range variables are in the constants and if not add the span
                if let ExprKind::Path(QPath::Resolved(_, path)) = &end_expr.expr.kind {
                    if let Res::Local(local_def_id) = path.res {
                        self.shared_state.range_variables.push(local_def_id);
                    }
                }
                if let ExprKind::Path(QPath::Resolved(_, path)) = &start_expr.expr.kind {
                    if let Res::Local(local_def_id) = path.res {
                        self.shared_state.range_variables.push(local_def_id);
                    }
                }
                println!("range {:?}", self.shared_state.range_variables);
            }
        }
        walk_expr(self, expr);
    }
}

impl<'tcx> LateLintPass<'tcx> for DosUnboundedOperation {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        _: Span,
        _: LocalDefId,
    ) {
        if let FnKind::Method(a, _) = kind {
            // TODO: remove this
            if a.name.as_str() != "asd" {
                return;
            }
            let mut visitor = ForLoopVisitor {
                span_constant: Vec::new(),
                shared_state: &mut self.shared_state,
            };

            walk_body(&mut visitor, body);

            // If the range any of the items of the range variable is not in the constants, then we have a problem, span lint
            for range_variable in &visitor.shared_state.range_variables {
                if !visitor.shared_state.constants.contains(range_variable) {
                    // We have a problem
                    println!("we have a problem");
                }
            }

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

    // fn check_local(&mut self, _: &LateContext<'tcx>, local: &'tcx Local<'tcx>) {
    //     if let Some(init) = &local.init {
    //         match &init.kind {
    //             ExprKind::Lit(lit) => {
    //                 if let LitKind::Int(_, _) = lit.node {
    //                     // It is a constant, add it to dictionary
    //                     println!("local {:?}", local.pat.hir_id);
    //                     self.shared_state.constants.push(local.pat.hir_id);
    //                 }
    //             }
    //             ExprKind::Path(QPath::Resolved(_, path)) => {
    //                 // We search the path, if it is previously defined then we good
    //                 if let Res::Local(_local_def_id) = path.res {
    //                     // If self.constants[local_def_id] is not true, then the variable is not a constant
    //                     self.shared_state.constants.push(local.pat.hir_id);
    //                 }
    //             }
    //             ExprKind::Binary(_, left, right) => {
    //                 // If left or right is not a constant, then we need to check if it is previously defined as a constant
    //                 // If they were not previously defined, then we dont add it to the dictionary
    //                 let mut left_is_constant = false;
    //                 let mut right_is_constant = false;
    //                 match left.kind {
    //                     ExprKind::Lit(lit) => {
    //                         if let LitKind::Int(_, _) = lit.node {
    //                             left_is_constant = true;
    //                         }
    //                     }
    //                     ExprKind::Path(QPath::Resolved(_, path)) => {
    //                         // We search the path, if it is previously defined then we good
    //                         if let Res::Local(_) = path.res {
    //                             left_is_constant = true;
    //                         }
    //                     }
    //                     _ => {}
    //                 }

    //                 match right.kind {
    //                     ExprKind::Lit(lit) => {
    //                         if let LitKind::Int(_, _) = lit.node {
    //                             right_is_constant = true;
    //                         }
    //                     }
    //                     ExprKind::Path(QPath::Resolved(_, path)) => {
    //                         // We search the path, if it is previously defined then we good
    //                         if let Res::Local(_) = path.res {
    //                             right_is_constant = true;
    //                         }
    //                     }
    //                     _ => {}
    //                 }

    //                 if left_is_constant && right_is_constant {
    //                     // It is a constant, add it to dictionary
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    // }
}
