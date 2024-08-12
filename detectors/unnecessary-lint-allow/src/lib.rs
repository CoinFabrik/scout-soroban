#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_span;

use clippy_wrappers::span_lint;
use if_chain::if_chain;
use rustc_ast::{
    token::{Delimiter, Token, TokenKind},
    tokenstream::{TokenStream, TokenTree},
    AttrArgs, AttrKind,
};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::Symbol;
use std::collections::VecDeque;

const LINT_MESSAGE: &str = "This `#[allow]` attribute may be unnecessary. Consider removing it if the lint is no longer triggered.";

dylint_linting::declare_pre_expansion_lint! {
    pub UNNECESSARY_LINT_ALLOW,
    Warn,
    LINT_MESSAGE,
    {
        name: "Unnecessary Lint Allow",
        long_message: "The `#[allow]` attribute is used to disable lints. It is recommended to fix the issues instead of disabling them.",
        severity: "Enhancement",
        help: "https://coinfabrik.github.io/scout-soroban/docs/detectors/unnecessary-lint-allow",
        vulnerability_class: "Code Quality",
    }
}

impl UnnecessaryLintAllow {
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

impl EarlyLintPass for UnnecessaryLintAllow {
    fn check_attribute(&mut self, cx: &EarlyContext<'_>, attr: &rustc_ast::Attribute) {
        if_chain! {
            if !attr.span.from_expansion();
            if attr.has_name(Symbol::intern("scout_allow"));
            if let AttrKind::Normal(item) = &attr.kind;
            if let AttrArgs::Delimited(delimited_args) = &item.item.args;
            then {
                let lint_names = self.extract_lint_names(&delimited_args.tokens);
                for lint_name in lint_names {
                    span_lint(cx, UNNECESSARY_LINT_ALLOW, attr.span, lint_name);
                }
            }
        }
    }
}
