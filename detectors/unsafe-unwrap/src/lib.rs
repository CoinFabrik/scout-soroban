#![feature(rustc_private)]

extern crate rustc_hir;
extern crate rustc_span;

use if_chain::if_chain;
use rustc_hir::{
    def::Res,
    def_id::LocalDefId,
    intravisit::{walk_expr, FnKind, Visitor},
    Block, Body, Expr, ExprKind, FnDecl, HirId, QPath, Stmt, StmtKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{sym, Span, Symbol};
use scout_audit_clippy_utils::diagnostics::span_lint_and_help;

const LINT_MESSAGE: &str = "Unsafe usage of `unwrap`";

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
    LINT_MESSAGE,
    {
        name: "Unsafe Unwrap",
        long_message: "This vulnerability class pertains to the inappropriate usage of the unwrap method in Rust, which is commonly employed for error handling. The unwrap method retrieves the inner value of an Option or Result, but if an error or None occurs, it triggers a panic and crashes the program.    ",
        severity: "Medium",
        help: "https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unsafe-unwrap",
        vulnerability_class: "Validations and error handling",
    }
}

struct UnsafeUnwrapVisitor {
    checked_exprs: Vec<HirId>,
    unwrap_spans: Vec<Span>,
}

impl UnsafeUnwrapVisitor {
    pub fn analyze_block(
        &mut self,
        block: &Block<'_>,
        expression_hir_id: &HirId,
        should_return: bool,
    ) -> bool {
        for stmt in block.stmts {
            // Analyze the statement to know wether it returns or not.
            if self.analyze_statement(stmt, expression_hir_id, should_return) {
                return true;
            }
        }

        false
    }

    pub fn analyze_statement(
        &mut self,
        stmt: &Stmt<'_>,
        expression_hir_id: &HirId,
        should_return: bool,
    ) -> bool {
        match &stmt.kind {
            StmtKind::Expr(expr) | StmtKind::Semi(expr) => {
                // Analyze each expression to know if it returns or not.
                self.analyze_expression(expr, expression_hir_id, should_return)
            }
            _ => false,
        }
    }

    pub fn analyze_expression(
        &mut self,
        expr: &Expr<'_>,
        expression_hir_id: &HirId,
        should_return: bool,
    ) -> bool {
        match &expr.kind {
            ExprKind::Block(block, _) => {
                self.analyze_block(block, expression_hir_id, should_return)
            }
            ExprKind::Ret(_) => should_return,
            ExprKind::Call(func, _) => self.analyze_function_call(func, should_return),
            ExprKind::MethodCall(path_segment, receiver, _, _) => {
                if_chain! {
                    if path_segment.ident.name == sym::unwrap;
                    if let ExprKind::Path(QPath::Resolved(_, path)) = &receiver.kind;
                    if let Res::Local(hir_id) = path.res;
                    then {
                        return hir_id == *expression_hir_id;
                    }
                }
                false
            }
            _ => should_return,
        }
    }

    pub fn analyze_function_call(&mut self, func: &Expr<'_>, should_return: bool) -> bool {
        if let ExprKind::Path(QPath::Resolved(_, path)) = &func.kind {
            if path
                .segments
                .last()
                .map_or(false, |seg| seg.ident.name.as_str().contains("panic"))
            {
                // We could add other functions that also panic like `bail!`
                return should_return;
            }
        }
        false
    }
}

impl<'tcx> Visitor<'tcx> for UnsafeUnwrapVisitor {
    fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
        let should_check_for_returns = [Symbol::intern("is_err"), Symbol::intern("is_none")];
        let should_check_for_non_returns = [Symbol::intern("is_ok"), Symbol::intern("is_some")];

        // Find statemets that might be validating agains an unwrap call
        if_chain! {
            // Find an 'if' expression and peel off any temporary values.
            if let ExprKind::If(condition, if_expr, _) = &expr.kind;
            if let ExprKind::DropTemps(peeled_expr) = condition.kind;
            // Check if the condition is a method call for further analysis.
            if let ExprKind::MethodCall(path_segment, receiver, _, _) = &peeled_expr.kind;
            then {
                // Determine if the method call is one that we are interested in.
                let should_analyze = should_check_for_returns.contains(&path_segment.ident.name) ||
                                     should_check_for_non_returns.contains(&path_segment.ident.name);

                if should_analyze {
                    // Get the receiver of the method call.
                    if let ExprKind::Path(QPath::Resolved(_, checked_expr_path)) = receiver.kind {
                        // Get the HirId of the receiver.z
                        if let Res::Local(checked_expr_hir_id) = checked_expr_path.res {
                            // Analyze the inner block of the 'if' expression to determine if it is safe to unwrap.
                            if let ExprKind::Block(if_block, _) = &if_expr.kind {
                                if self.analyze_block(if_block, &checked_expr_hir_id, should_check_for_returns.contains(&path_segment.ident.name)) {
                                    self.checked_exprs.push(checked_expr_hir_id);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Find unwrap calls
        if_chain! {
            if let ExprKind::MethodCall(path_segment, receiver, _, _) = &expr.kind;
            if path_segment.ident.name == sym::unwrap;
            // Get the receiver HirId and verify that it has been checked.
            if let ExprKind::Path(QPath::Resolved(_, unwrap_expr_path)) = receiver.kind;
            if let Res::Local(receiver_hir_id) = unwrap_expr_path.res;
            if !self.checked_exprs.contains(&receiver_hir_id);
            then {
                self.unwrap_spans.push(expr.span);
            }
        }

        walk_expr(self, expr);
    }
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
        // If the function comes from a macro expansion, we don't want to analyze it.
        if span.from_expansion() {
            return;
        }

        let mut visitor = UnsafeUnwrapVisitor {
            unwrap_spans: Vec::new(),
            checked_exprs: Vec::new(),
        };

        walk_expr(&mut visitor, body.value);

        visitor.unwrap_spans.iter().for_each(|span| {
            span_lint_and_help(
                cx,
                UNSAFE_UNWRAP,
                *span,
                LINT_MESSAGE,
                None,
                "Please, use a custom error instead of `unwrap`",
            );
        });
    }
}
