#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use clippy_wrappers::span_lint;
use if_chain::if_chain;
use rustc_ast::{
    token::{Delimiter, Token, TokenKind},
    tokenstream::{TokenStream, TokenTree},
    AttrArgs, AttrKind, Attribute,
};
use rustc_hir::{intravisit::FnKind, Body, Expr, FnDecl, HirId, Item, Stmt, CRATE_HIR_ID};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{def_id::LocalDefId, Span, Symbol};
use std::collections::{HashSet, VecDeque};

const LINT_MESSAGE: &str = "This `#[allow]` attribute may be unnecessary. Consider removing it if the lint is no longer triggered.";

dylint_linting::impl_late_lint!(
    pub UNNECESSARY_LINT_ALLOW,
    Warn,
    LINT_MESSAGE,
    UnnecessaryLintAllow::default(),
    {
        name: "Unnecessary Lint Allow",
        long_message: "The `#[allow]` attribute is used to disable lints. It is recommended to fix the issues instead of disabling them.",
        severity: "Enhancement",
        help: "https://coinfabrik.github.io/scout-soroban/docs/detectors/unnecessary-lint-allow",
        vulnerability_class: "Code Quality",
    }
);

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct AllowInfo {
    pub lint_name: String,
    pub span: Span,
}

#[derive(Default, Debug, Clone)]
struct UnnecessaryLintAllow {
    findings: HashSet<AllowInfo>,
}

impl UnnecessaryLintAllow {
    fn check_and_collect_attrs(&mut self, cx: &LateContext, hir_id: HirId, span: Span) {
        if span.from_expansion() {
            return;
        }

        let attrs = cx.tcx.hir().attrs(hir_id);
        if !attrs.is_empty() {
            for attr in attrs.iter() {
                self.collect_attribute(attr, span);
            }
        }
    }

    fn collect_attribute(&mut self, attr: &Attribute, span: Span) {
        if_chain! {
            if !attr.span.from_expansion();
            if attr.has_name(Symbol::intern("scout_allow"));
            if let AttrKind::Normal(item) = &attr.kind;
            if let AttrArgs::Delimited(delimited_args) = &item.item.args;
            then {
                let lint_names = self.extract_lint_names(&delimited_args.tokens);
                for lint_name in lint_names {
                    self.findings.insert(AllowInfo {
                        lint_name,
                        span,
                    });
                }
            }
        }
    }

    fn extract_lint_names(&self, tokens: &TokenStream) -> Vec<String> {
        let mut lint_names = Vec::new();
        let mut stack = VecDeque::new();
        stack.push_back(tokens);

        while let Some(current_stream) = stack.pop_back() {
            for tree in current_stream.trees() {
                match tree {
                    TokenTree::Token(
                        Token {
                            kind: TokenKind::Ident(ident, _),
                            ..
                        },
                        _,
                    ) => {
                        lint_names.push(ident.to_string());
                    }
                    TokenTree::Delimited(_, _, Delimiter::Parenthesis, inner_stream) => {
                        // Push the inner stream onto the stack for later processing
                        stack.push_back(inner_stream);
                    }
                    _ => {}
                }
            }
        }

        lint_names
    }
}

impl<'tcx> LateLintPass<'tcx> for UnnecessaryLintAllow {
    fn check_crate_post(&mut self, cx: &LateContext<'tcx>) {
        for finding in &self.findings {
            span_lint(cx, UNNECESSARY_LINT_ALLOW, finding.span, LINT_MESSAGE);
        }
    }

    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        // Collect crate-level attributes
        self.check_and_collect_attrs(cx, CRATE_HIR_ID, cx.tcx.hir().span(CRATE_HIR_ID));
    }

    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        // Collect item-level attributes (struct, enum, impl)
        self.check_and_collect_attrs(cx, item.hir_id(), item.span);
    }

    fn check_stmt(&mut self, cx: &LateContext<'tcx>, stmt: &'tcx Stmt<'tcx>) {
        // Collect statement-level attributes (let, return, etc.)
        self.check_and_collect_attrs(cx, stmt.hir_id, stmt.span);
    }

    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        // Collect expression-level attributes (function call, etc.)
        self.check_and_collect_attrs(cx, expr.hir_id, expr.span);
    }

    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        _: &'tcx Body<'tcx>,
        span: Span,
        local_def_id: LocalDefId,
    ) {
        // Collect function level attributes (function)
        let hir_id = cx.tcx.local_def_id_to_hir_id(local_def_id);
        self.check_and_collect_attrs(cx, hir_id, span);
    }
}
