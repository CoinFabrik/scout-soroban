#![feature(rustc_private)]

extern crate rustc_hir;
extern crate rustc_span;

mod conditional_checker;

use clippy_utils::higher::If;
use clippy_wrappers::span_lint;
use conditional_checker::{is_panic_inducing_call, ConditionalChecker};
use rustc_hir::{
    def::Res::Local,
    intravisit::{walk_expr, FnKind, Visitor},
    Body, Expr, ExprKind, FnDecl, HirId, Path, QPath,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{
    def_id::{DefId, LocalDefId},
    Span,
};
use std::{
    collections::{HashMap, HashSet},
    vec,
};
use utils::FunctionCallVisitor;

const LINT_MESSAGE: &str =
    "The transferred amount should be checked against a minimum to prevent front-running";

dylint_linting::impl_late_lint! {
    pub FRONT_RUNNING,
    Warn,
    LINT_MESSAGE,
    FrontRunning::default(),
    {
        name: "Front Running message",
        long_message: "Front Running message",
        severity: "",
        help: "",
        vulnerability_class: "",
    }
}

#[derive(Default)]
struct FrontRunning {
    function_call_graph: HashMap<DefId, HashSet<DefId>>,
    checked_functions: HashSet<String>,
}

impl<'tcx> LateLintPass<'tcx> for FrontRunning {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _fn_decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        local_def_id: LocalDefId,
    ) {
        let def_id = local_def_id.to_def_id();
        self.checked_functions.insert(cx.tcx.def_path_str(def_id));

        if span.from_expansion() {
            return;
        }

        let mut function_call_visitor =
            FunctionCallVisitor::new(cx, def_id, &mut self.function_call_graph);
        function_call_visitor.visit_body(body);

        let mut front_running_visitor = FrontRunningVisitor {
            cx,
            local_variables: Vec::new(),
            transfer_amount_id: Vec::new(),
            conditional_checker: Vec::new(),
            checked_hir_ids: Vec::new(),
        };
        front_running_visitor.visit_body(body);

        for (transfer_amount_id, span) in front_running_visitor.transfer_amount_id.iter() {
            if !front_running_visitor
                .checked_hir_ids
                .contains(transfer_amount_id)
                && front_running_visitor
                    .local_variables
                    .contains(transfer_amount_id)
            {
                span_lint(cx, FRONT_RUNNING, *span, LINT_MESSAGE);
            }
        }
    }
}

struct FrontRunningVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    local_variables: Vec<HirId>,
    transfer_amount_id: Vec<(HirId, Span)>,
    conditional_checker: Vec<ConditionalChecker>,
    checked_hir_ids: Vec<HirId>,
}

impl<'a, 'tcx> Visitor<'tcx> for FrontRunningVisitor<'a, 'tcx> {
    fn visit_local(&mut self, local: &'tcx rustc_hir::LetStmt<'tcx>) {
        self.local_variables.push(local.pat.hir_id);
    }

    fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
        // Check if the expression is a transfer method call, then store the HirId of the amount parameter
        if let ExprKind::MethodCall(path_segment, _receiver, methodargs, ..) = expr.kind {
            if path_segment.ident.name.to_string() == "transfer"
                && self
                    .cx
                    .typeck_results()
                    .node_type(_receiver.hir_id)
                    .to_string()
                    == "soroban_sdk::token::TokenClient<'_>"
            {
                let amount_param = methodargs[2];
                if let ExprKind::AddrOf(_, _, new_exp, ..) = amount_param.kind {
                    if let ExprKind::Path(QPath::Resolved(_, Path { segments, .. }), ..) =
                        new_exp.kind
                    {
                        let transfer_amount_param = segments.first();
                        if transfer_amount_param.is_some() {
                            if let Local(id) = transfer_amount_param.unwrap().res {
                                self.transfer_amount_id.push((id, expr.span));
                            }
                        }
                    }
                }
            }
        }

        // If we are inside an 'if' statement, check if the current expression is a return or a panic inducing call
        if !self.conditional_checker.is_empty() {
            let last_conditional_checker = self.conditional_checker.iter().last().unwrap();
            match &expr.kind {
                ExprKind::Ret(..) => {
                    let hir_id = last_conditional_checker.greater_expr;
                    if let Some(hir_id) = hir_id {
                        self.checked_hir_ids.push(hir_id);
                    }
                }
                ExprKind::Call(func, _) if is_panic_inducing_call(func) => {
                    let hir_id = last_conditional_checker.greater_expr;
                    if let Some(hir_id) = hir_id {
                        self.checked_hir_ids.push(hir_id);
                    }
                }
                _ => {}
            }
        }

        // Check if the expression has an 'if' and if it does, check if it meets our condition
        if let Some(If {
            cond,
            then: if_expr,
            r#else: _,
        }) = If::hir(expr)
        {
            let mut conditional_checker = ConditionalChecker {
                greater_expr: None,
                lesser_expr: None,
            };
            if conditional_checker.handle_condition(cond) {
                self.conditional_checker.push(conditional_checker);
                walk_expr(self, if_expr);
            }
            self.conditional_checker.pop();
            return;
        }

        walk_expr(self, expr);
    }
}
