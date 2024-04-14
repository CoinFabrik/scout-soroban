#![feature(rustc_private)]
#![allow(clippy::enum_variant_names)]

extern crate rustc_hir;
extern crate rustc_span;

use std::collections::HashSet;

use if_chain::if_chain;
use rustc_hir::{
    def::Res,
    def_id::LocalDefId,
    intravisit::{walk_expr, FnKind, Visitor},
    BinOpKind, Body, Expr, ExprKind, FnDecl, HirId, QPath, UnOp,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{sym, Span, Symbol};
use scout_audit_clippy_utils::{diagnostics::span_lint_and_help, higher};

const LINT_MESSAGE: &str = "Unsafe usage of `unwrap`";
const PANIC_INDUCING_FUNCTIONS: [&str; 2] = ["panic", "bail"];

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

#[derive(Clone, Copy)]
enum CheckType {
    IsSome,
    IsNone,
    IsOk,
    IsErr,
}

impl CheckType {
    fn from_method_name(name: Symbol) -> Option<Self> {
        match name.as_str() {
            "is_some" => Some(Self::IsSome),
            "is_none" => Some(Self::IsNone),
            "is_ok" => Some(Self::IsOk),
            "is_err" => Some(Self::IsErr),
            _ => None,
        }
    }

    fn inverse(self) -> Self {
        match self {
            Self::IsSome => Self::IsNone,
            Self::IsNone => Self::IsSome,
            Self::IsOk => Self::IsErr,
            Self::IsErr => Self::IsOk,
        }
    }

    fn should_halt_execution(self) -> bool {
        matches!(self, Self::IsNone | Self::IsErr)
    }

    fn is_safe_to_unwrap(self) -> bool {
        matches!(self, Self::IsSome | Self::IsOk)
    }
}
#[derive(Clone, Copy)]
struct ConditionalChecker {
    check_type: CheckType,
    checked_expr_hir_id: HirId,
}

impl ConditionalChecker {
    fn handle_expression(expr: &Expr<'_>, inverse: bool) -> Option<Self> {
        if_chain! {
            if let ExprKind::MethodCall(path_segment, receiver, _, _) = expr.kind;
            if let Some(check_type) = CheckType::from_method_name(path_segment.ident.name);
            if let ExprKind::Path(QPath::Resolved(_, checked_expr_path)) = receiver.kind;
            if let Res::Local(checked_expr_hir_id) = checked_expr_path.res;
            then {
                return Some(Self {
                    check_type: if inverse { check_type.inverse() } else { check_type },
                    checked_expr_hir_id,
                });
            }
        }
        None
    }

    /// Constructs a ConditionalChecker from an expression if it matches a method call with a valid CheckType.
    fn from_expression(expr: &Expr<'_>) -> Option<Self> {
        match expr.kind {
            ExprKind::Unary(op, expr) => Self::handle_expression(expr, op == UnOp::Not),
            // For now we will only support the `or` operator (`||`), and we will get the first expression that is a CheckType
            // In the future we could support the `or` operator with n operands and construct a HashMap out of it.
            ExprKind::Binary(op, left_expr, right_expr) => {
                if op.node == BinOpKind::Or {
                    return Self::from_expression(left_expr)
                        .or_else(|| Self::from_expression(right_expr));
                }
                None
            }
            ExprKind::MethodCall(..) => Self::handle_expression(expr, false),
            _ => None,
        }
    }
}

struct UnsafeUnwrapVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    conditional_checker: Option<ConditionalChecker>,
    checked_exprs: HashSet<HirId>,
}

impl UnsafeUnwrapVisitor<'_, '_> {
    fn is_panic_inducing_call(&self, func: &Expr<'_>) -> bool {
        if_chain! {
            if let ExprKind::Path(QPath::Resolved(_, path)) = &func.kind;
            then {
                return PANIC_INDUCING_FUNCTIONS.iter().any(|&func| {
                    path.segments
                        .iter()
                        .any(|segment| segment.ident.name.as_str().contains(func))
                });
            }
        }
        false
    }

    fn get_unwrap_info(&self, receiver: &Expr<'_>) -> Option<HirId> {
        if_chain! {
            if let ExprKind::Path(QPath::Resolved(_, path)) = &receiver.kind;
            if let Res::Local(hir_id) = path.res;
            then {
                Some(hir_id)
            } else {
                None
            }
        }
    }

    fn set_conditional_checker(&mut self, conditional_checker: ConditionalChecker) {
        self.conditional_checker = Some(conditional_checker);
        if conditional_checker.check_type.is_safe_to_unwrap() {
            self.checked_exprs
                .insert(conditional_checker.checked_expr_hir_id);
        };
    }

    fn reset_conditional_checker(&mut self, conditional_checker: ConditionalChecker) {
        if conditional_checker.check_type.is_safe_to_unwrap() {
            self.checked_exprs
                .remove(&conditional_checker.checked_expr_hir_id);
        }
        self.conditional_checker = None;
    }
}

impl<'a, 'tcx> Visitor<'tcx> for UnsafeUnwrapVisitor<'a, 'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
        // If we are inside an `if` or `if let` expression, we analyze the expressions
        if_chain! {
            if let Some(conditional_checker) = &self.conditional_checker;
            if conditional_checker.check_type.should_halt_execution();
            then {
                // If the execution should be halted, we ensure it.
                match &expr.kind {
                    ExprKind::Ret(..) => {
                        self.checked_exprs.insert(conditional_checker.checked_expr_hir_id);
                    },
                    ExprKind::Call(func, _) => {
                        if self.is_panic_inducing_call(func) {
                            self.checked_exprs.insert(conditional_checker.checked_expr_hir_id);
                        }
                    },
                    _ => {}
                }

            }
        }

        // Find `if` or `if let` expressions
        if let Some(higher::IfOrIfLet {
            cond,
            then: if_expr,
            r#else: _,
        }) = higher::IfOrIfLet::hir(expr)
        {
            // If we are interested in the condition (if it is a CheckType) we analyze the if expression
            if let Some(conditional_checker) = ConditionalChecker::from_expression(cond) {
                self.set_conditional_checker(conditional_checker);
                walk_expr(self, if_expr);
                self.reset_conditional_checker(conditional_checker);
                return;
            }
        }

        // If we find an unsafe `unwrap`, we raise a warning
        if_chain! {
            if let ExprKind::MethodCall(path_segment, receiver, _, _) = &expr.kind;
            if path_segment.ident.name == sym::unwrap;
            then {
                let receiver_hir_id = self.get_unwrap_info(receiver);
                // If the receiver is `None`, then we asume that the `unwrap` is unsafe
                let is_checked_safe = receiver_hir_id.map_or(false, |id| self.checked_exprs.contains(&id));
                if !is_checked_safe {
                    span_lint_and_help(
                        self.cx,
                        UNSAFE_UNWRAP,
                        expr.span,
                        LINT_MESSAGE,
                        None,
                        "Please, use a custom error instead of `unwrap`",
                    );
                }
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
            cx,
            checked_exprs: HashSet::new(),
            conditional_checker: None,
        };

        walk_expr(&mut visitor, body.value);
    }
}
