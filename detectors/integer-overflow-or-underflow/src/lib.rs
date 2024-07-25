#![feature(rustc_private)]

extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_span;

use clippy_utils::{consts::constant_simple, diagnostics::span_lint_and_sugg};
use rustc_errors::Applicability;
use rustc_hir::{
    intravisit::{walk_expr, FnKind, Visitor},
    BinOpKind, Body, Expr, ExprKind, FnDecl, UnOp,
};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::{def_id::LocalDefId, Span};

pub const LINT_MESSAGE: &str = "Potential for integer arithmetic overflow/underflow. Consider checked, wrapping or saturating arithmetic.";

dylint_linting::declare_late_lint! {
    pub INTEGER_OVERFLOW_UNDERFLOW,
    Warn,
    LINT_MESSAGE,
    {
        name: "Integer Overflow/Underflow",
        long_message: "An overflow/underflow is typically caught and generates an error. When it is not caught, the operation will result in an inexact result which could lead to serious problems.",
        severity: "Critical",
        help: "https://coinfabrik.github.io/scout-soroban/docs/vulnerabilities/integer-overflow-or-underflow",
        vulnerability_class: "Arithmetic",
    }
}
enum Type {
    Overflow,
    Underflow,
}

impl Type {
    fn message(&self) -> &'static str {
        match self {
            Type::Overflow => "overflow",
            Type::Underflow => "underflow",
        }
    }
}

enum Cause {
    Add,
    Sub,
    Mul,
    Pow,
    Negate,
}

impl Cause {
    fn message(&self) -> &'static str {
        match self {
            Cause::Add => "addition",
            Cause::Sub => "subtraction",
            Cause::Mul => "multiplication",
            Cause::Pow => "exponentiation",
            Cause::Negate => "negation",
        }
    }
}

pub struct Finding {
    span: Span,
    type_: Type,
    cause: Cause,
    left: String,
    right: String,
}

impl Finding {
    fn new(span: Span, type_: Type, cause: Cause, left: String, right: String) -> Self {
        Finding {
            span,
            type_,
            cause,
            left,
            right,
        }
    }

    fn get_suggestion(&self) -> String {
        match self.cause {
            Cause::Add => format!("{}.checked_add({})", self.left, self.right),
            Cause::Sub => format!("{}.checked_sub({})", self.left, self.right),
            Cause::Mul => format!("{}.checked_mul({})", self.left, self.right),
            Cause::Pow => format!("{}.checked_pow({})", self.left, self.right),
            Cause::Negate => format!("{}.checked_neg()", self.left),
        }
        .to_string()
    }

    fn generate_message(&self) -> String {
        format!(
            "This {} operation could {}.",
            self.cause.message(),
            self.type_.message()
        )
    }
}
pub struct IntegerOverflowUnderflowVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    findings: Vec<Finding>,
}

impl IntegerOverflowUnderflowVisitor<'_, '_> {
    pub fn check_pow<'tcx>(
        &mut self,
        expr: &'tcx Expr<'_>,
        receiver: &'tcx Expr<'tcx>,
        exponent: &'tcx Expr<'tcx>,
    ) {
        let base_ty = self.cx.typeck_results().expr_ty(receiver);
        if base_ty.is_integral() {
            self.findings.push(Finding::new(
                expr.span,
                Type::Overflow,
                Cause::Pow,
                self.get_snippet(receiver),
                self.get_snippet(exponent),
            ));
        }
    }

    pub fn check_negate<'tcx>(&mut self, expr: &'tcx Expr<'_>, arg: &'tcx Expr<'_>) {
        let ty = self.cx.typeck_results().expr_ty(arg);
        if ty.is_integral() && ty.is_signed() {
            // We only care about non-constant values
            if constant_simple(self.cx, self.cx.typeck_results(), arg).is_none() {
                self.findings.push(Finding::new(
                    expr.span,
                    Type::Overflow,
                    Cause::Negate,
                    self.get_snippet(arg),
                    "".to_string(),
                ));
            }
        }
    }

    pub fn check_binary<'tcx>(
        &mut self,
        expr: &'tcx Expr<'_>,
        op: BinOpKind,
        l: &'tcx Expr<'_>,
        r: &'tcx Expr<'_>,
    ) {
        // Check if either operand is non-constant
        let l_const = constant_simple(self.cx, self.cx.typeck_results(), l);
        let r_const = constant_simple(self.cx, self.cx.typeck_results(), r);
        if l_const.is_some() && r_const.is_some() {
            return;
        }

        // Check if any of the operands is not an integer
        let (l_ty, r_ty) = (
            self.cx.typeck_results().expr_ty(l),
            self.cx.typeck_results().expr_ty(r),
        );
        if !l_ty.peel_refs().is_integral() || !r_ty.peel_refs().is_integral() {
            return;
        }

        // Check if the operation is an addition, subtraction or multiplication
        let (finding_type, cause) = match op {
            BinOpKind::Add => (Type::Overflow, Cause::Add),
            BinOpKind::Sub => (Type::Underflow, Cause::Sub),
            BinOpKind::Mul => (Type::Overflow, Cause::Mul),
            _ => return, // We're not interested in other operations
        };

        self.findings.push(Finding::new(
            expr.span,
            finding_type,
            cause,
            self.get_snippet(l),
            self.get_snippet(r),
        ));
    }

    fn get_snippet(&self, expr: &Expr<'_>) -> String {
        self.cx
            .sess()
            .source_map()
            .span_to_snippet(expr.span)
            .unwrap()
    }
}

impl<'a, 'tcx> Visitor<'tcx> for IntegerOverflowUnderflowVisitor<'a, 'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        match expr.kind {
            ExprKind::Binary(op, lhs, rhs) | ExprKind::AssignOp(op, lhs, rhs) => {
                self.check_binary(expr, op.node, lhs, rhs);
            }
            ExprKind::Unary(UnOp::Neg, arg) => {
                self.check_negate(expr, arg);
            }
            ExprKind::MethodCall(method_name, receiver, args, ..) => {
                if method_name.ident.as_str() == "pow" {
                    self.check_pow(expr, receiver, &args[0]);
                }
            }
            _ => (),
        }
        walk_expr(self, expr);
    }
}

impl<'tcx> LateLintPass<'tcx> for IntegerOverflowUnderflow {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        _: LocalDefId,
    ) {
        if span.from_expansion() {
            return;
        }

        let mut visitor = IntegerOverflowUnderflowVisitor {
            cx,
            findings: Vec::new(),
        };
        visitor.visit_body(body);

        for finding in visitor.findings {
            span_lint_and_sugg(
                cx,
                INTEGER_OVERFLOW_UNDERFLOW,
                finding.span,
                finding.generate_message(),
                "Consider using the checked version of this operation",
                finding.get_suggestion(),
                Applicability::MaybeIncorrect,
            );
        }
    }
}
