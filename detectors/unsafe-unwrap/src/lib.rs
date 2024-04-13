#![feature(rustc_private)]
#![allow(clippy::enum_variant_names)]

extern crate rustc_hir;
extern crate rustc_span;

use if_chain::if_chain;
use rustc_hir::{
    def::Res,
    def_id::LocalDefId,
    intravisit::{walk_expr, FnKind, Visitor},
    Block, Body, Expr, ExprKind, FnDecl, HirId, Local, QPath, Stmt, StmtKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{sym, Span, Symbol};
use scout_audit_clippy_utils::{diagnostics::span_lint_and_help, higher};

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

#[derive(Debug, Clone, Copy)]
enum CheckType {
    IsSome,
    IsNone,
    IsOk,
    IsErr,
}

impl CheckType {
    /// Converts a `Symbol` to a `CheckType` if it matches a known method name.
    fn from_method_name(name: Symbol) -> Option<Self> {
        match name.as_str() {
            "is_some" => Some(Self::IsSome),
            "is_none" => Some(Self::IsNone),
            "is_ok" => Some(Self::IsOk),
            "is_err" => Some(Self::IsErr),
            _ => None,
        }
    }

    /// Determines if the check type allows safe unwrapping.
    fn is_safe_to_unwrap(self) -> bool {
        matches!(self, Self::IsSome | Self::IsOk)
    }

    /// Determines if the check type implies that a function should return or halt.
    fn is_safe_to_return(self) -> bool {
        matches!(self, Self::IsNone | Self::IsErr)
    }
}

impl UnsafeUnwrapVisitor {
    fn analyze_if_expr(
        &mut self,
        expr: &Expr<'_>,
        check_type: CheckType,
        target_hir_id: &HirId,
    ) -> bool {
        match &expr.kind {
            ExprKind::Block(block, _) => self.analyze_block(block, target_hir_id, check_type),
            _ => {
                println!("Expr kind: {:?}", expr.kind);
                false
            }
        }
    }

    fn analyze_block(
        &mut self,
        block: &Block<'_>,
        expression_hir_id: &HirId,
        check_type: CheckType,
    ) -> bool {
        block
            .stmts
            .iter()
            .any(|stmt| self.analyze_statement(stmt, expression_hir_id, &check_type))
    }

    fn analyze_statement(
        &mut self,
        stmt: &Stmt<'_>,
        expression_hir_id: &HirId,
        check_type: &CheckType,
    ) -> bool {
        match &stmt.kind {
            StmtKind::Expr(expr) | StmtKind::Semi(expr) => {
                self.analyze_expression(expr, expression_hir_id, check_type)
            }
            StmtKind::Local(local) => {
                self.analyze_local_statement(local, expression_hir_id, check_type)
            }
            _ => false,
        }
    }

    fn analyze_local_statement(
        &mut self,
        local: &Local<'_>,
        expression_hir_id: &HirId,
        check_type: &CheckType,
    ) -> bool {
        if let Some(init) = local.init {
            return self.analyze_expression(init, expression_hir_id, check_type);
        }
        false
    }

    fn analyze_expression(
        &mut self,
        expr: &Expr<'_>,
        expression_hir_id: &HirId,
        check_type: &CheckType,
    ) -> bool {
        match &expr.kind {
            ExprKind::Block(block, _) => self.analyze_block(block, expression_hir_id, *check_type),
            ExprKind::Ret(_) => check_type.is_safe_to_return(),
            ExprKind::Call(func, args) => {
                self.analyze_call(func, args, expression_hir_id, check_type)
            }
            ExprKind::MethodCall(..) => {
                self.analyze_method_call(expr, expression_hir_id, check_type)
            }
            _ => false,
        }
    }

    fn analyze_method_call(
        &mut self,
        expr: &Expr<'_>,
        expression_hir_id: &HirId,
        check_type: &CheckType,
    ) -> bool {
        self.is_unwrap_call(expr, expression_hir_id) && check_type.is_safe_to_unwrap()
    }

    fn analyze_call(
        &mut self,
        func: &Expr<'_>,
        args: &[Expr<'_>],
        expression_hir_id: &HirId,
        check_type: &CheckType,
    ) -> bool {
        if self.is_unwrap_call(func, expression_hir_id) {
            return check_type.is_safe_to_unwrap();
        }

        if self.is_panic_inducing_call(func) {
            return check_type.is_safe_to_return();
        }

        args.iter()
            .any(|arg| self.analyze_expression(arg, expression_hir_id, check_type))
    }

    fn is_panic_inducing_call(&self, expr: &Expr<'_>) -> bool {
        // TODO: should also check for other panic-inducing calls like `bail!`
        if let ExprKind::Path(QPath::Resolved(_, path)) = &expr.kind {
            return path
                .segments
                .iter()
                .any(|segment| segment.ident.name.as_str().contains("panic"));
        }
        false
    }

    fn is_unwrap_call(&self, expr: &Expr<'_>, target_hir_id: &HirId) -> bool {
        if_chain! {
            if let ExprKind::MethodCall(path_segment, receiver, _, _) = &expr.kind;
            if path_segment.ident.name == sym::unwrap;
            if let ExprKind::Path(QPath::Resolved(_, path)) = &receiver.kind;
            if let Res::Local(hir_id) = path.res;
            then {
                return hir_id == *target_hir_id;
            }
        }
        false
    }
}

impl<'tcx> Visitor<'tcx> for UnsafeUnwrapVisitor {
    fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
        // Find statemets that might be validating agains an unwrap call
        if_chain! {
            // Find an 'if' expression without DropTemps.
            if let Some(higher::If { cond, then: if_expr, r#else: _ }) = higher::If::hir(expr);
            // Check if the condition is a method call that we are interested in.
            // //TODO: handle more expressions
            if let ExprKind::MethodCall(path_segment, receiver, _, _) = &cond.kind;
            if let Some(check_type) = CheckType::from_method_name(path_segment.ident.name);
            // Get the receiver of the method call and its HirId
            if let ExprKind::Path(QPath::Resolved(_, checked_expr_path)) = receiver.kind;
            if let Res::Local(checked_expr_hir_id) = checked_expr_path.res;
            // Analyze the if expression to determine if it is safe to unwrap
            if self.analyze_if_expr(if_expr, check_type, &checked_expr_hir_id);
            then {
                self.checked_exprs.push(checked_expr_hir_id);
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
