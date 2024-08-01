#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

mod types;

use if_chain::if_chain;
use rustc_ast::{
    token::{Delimiter, Token, TokenKind},
    tokenstream::{TokenStream, TokenTree},
    AttrArgs, AttrKind, Attribute,
};
use rustc_hir::{
    intravisit::{walk_expr, FnKind, Visitor},
    Body, FnDecl,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{def_id::LocalDefId, sym, Span};
use std::collections::{HashSet, VecDeque};
use types::{AllowInfo, Scope, SpanInfo};

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

#[derive(Default, Debug, Clone)]
struct UnnecessaryLintAllow {
    findings: HashSet<AllowInfo>,
}

impl UnnecessaryLintAllow {
    pub fn collect_attribute(
        &mut self,
        cx: &LateContext,
        attr: &Attribute,
        scope: Scope,
        item_span: Span,
    ) {
        if_chain! {
            if !attr.span.from_expansion();
            if attr.has_name(sym::allow);
            if let AttrKind::Normal(item) = &attr.kind;
            if let AttrArgs::Delimited(delimited_args) = &item.item.args;
            then {
                let lint_names = self.extract_lint_names(&delimited_args.tokens);
                for lint_name in lint_names {
                    self.findings.insert(AllowInfo {
                        lint_name,
                        span: SpanInfo::from_span(cx, item_span),
                        scope,
                    });
                }
            }
        }
    }

    pub fn extract_lint_names(&self, tokens: &TokenStream) -> Vec<String> {
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
                    _ => {} // Ignore other token types
                }
            }
        }

        lint_names
    }
}

impl<'tcx> LateLintPass<'tcx> for UnnecessaryLintAllow {
    fn check_crate_post(&mut self, _: &LateContext<'tcx>) {
        for finding in &self.findings {
            println!(
                "Found unnecessary `#[allow({})]` attribute at {}:{}-{}",
                finding.lint_name,
                finding.span.file_name,
                finding.span.from_line,
                finding.span.to_line
            );
        }
    }

    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        // Collect crate-level attributes
        for attr in cx.tcx.hir().attrs(rustc_hir::CRATE_HIR_ID) {
            self.collect_attribute(cx, attr, Scope::Crate, attr.span);
        }
    }

    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx rustc_hir::Item<'tcx>) {
        if item.span.from_expansion() {
            return;
        }

        // Collect item-level attributes (struct, enum, impl)
        let attrs = cx.tcx.hir().attrs(item.hir_id());
        let scope = match item.kind {
            rustc_hir::ItemKind::Struct(..) => Scope::Struct,
            rustc_hir::ItemKind::Enum(..) => Scope::Enum,
            rustc_hir::ItemKind::Impl(..) => Scope::Impl,
            _ => return,
        };

        for attr in attrs.iter() {
            self.collect_attribute(cx, attr, scope, item.span);
        }
    }

    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        local_def_id: LocalDefId,
    ) {
        // If the function comes from a macro expansion, we ignore it
        if span.from_expansion() {
            return;
        }

        // Collect function level attributes (function)
        let hir_id = cx.tcx.local_def_id_to_hir_id(local_def_id);
        let attrs = cx.tcx.hir().attrs(hir_id);
        for attr in attrs.iter() {
            self.collect_attribute(cx, attr, Scope::Function, span);
        }

        // Collect inner-level attributes (line)
        struct InnerAttrVisitor<'a, 'tcx> {
            cx: &'a LateContext<'tcx>,
            lint: &'a mut UnnecessaryLintAllow,
        }

        impl<'a, 'tcx> Visitor<'tcx> for InnerAttrVisitor<'a, 'tcx> {
            fn visit_expr(&mut self, expr: &'tcx rustc_hir::Expr<'tcx>) {
                let attrs = self.cx.tcx.hir().attrs(expr.hir_id);
                if !attrs.is_empty() {
                    for attr in attrs.iter() {
                        self.lint
                            .collect_attribute(self.cx, attr, Scope::Line, expr.span);
                    }
                }

                walk_expr(self, expr);
            }
        }

        let mut visitor = InnerAttrVisitor { cx, lint: self };
        walk_expr(&mut visitor, body.value);
    }
}
