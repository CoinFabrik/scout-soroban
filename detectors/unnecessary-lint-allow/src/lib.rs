#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;

use std::collections::HashSet;

use rustc_ast::{
    token::{Delimiter, Token, TokenKind},
    tokenstream::{TokenStream, TokenTree},
    AttrArgs, AttrKind, Attribute,
};
use rustc_hir::{
    intravisit::{walk_expr, FnKind, Visitor},
    Body, FnDecl,
};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::{def_id::LocalDefId, sym, FileName, FileNameDisplayPreference, Span};
use serde::{Deserialize, Serialize};

const LINT_MESSAGE: &str = "The `#[allow]` attribute is used to disable lints. It is recommended to fix the issues instead of disabling them.";

dylint_linting::impl_late_lint!(
    pub UNNECESSARY_LINT_ALLOW,
    Warn,
    LINT_MESSAGE,
    UnnecessaryLintAllow::default(),
    {
        name: "Unnecessary Lint Allow",
        long_message: "The `#[allow]` attribute is used to disable lints. It is recommended to fix the issues instead of disabling them.",
        severity: "Medium",
        help: "https://coinfabrik.github.io/scout-soroban/docs/detectors/unnecessary-lint-allow",
        vulnerability_class: "Code Quality",
    }
);

#[derive(Default, Debug)]
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
        if attr.span.from_expansion() {
            return;
        }

        if attr.has_name(sym::allow) {
            if let AttrKind::Normal(item) = &attr.kind {
                if let AttrArgs::Delimited(delimited_args) = &item.item.args {
                    let lint_names = extract_lint_names(&delimited_args.tokens);
                    for lint_name in lint_names {
                        self.findings.insert(AllowInfo {
                            lint_name,
                            span: SerializableSpan::from_span(cx, item_span),
                            scope,
                        });
                    }
                }
            }
        }
    }
}

fn extract_lint_names(tokens: &TokenStream) -> Vec<String> {
    let mut lint_names = Vec::new();
    for tree in tokens.trees() {
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
                // Recursively process nested token streams
                lint_names.extend(extract_lint_names(inner_stream));
            }
            _ => {}
        }
    }
    lint_names
}

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
pub struct AllowInfo {
    pub lint_name: String,
    pub span: SerializableSpan,
    pub scope: Scope,
}

#[derive(Serialize, Debug, Deserialize, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Scope {
    Crate,
    Enum,
    Function,
    Impl,
    Line,
    Struct,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Debug)]
pub struct SerializableSpan {
    pub file_name: String,
    pub from_line: usize,
    pub to_line: usize,
}

impl SerializableSpan {
    pub fn from_span(cx: &LateContext, span: Span) -> Self {
        let source_map = cx.sess().source_map();
        let file = source_map.lookup_source_file(span.lo());
        let file_name = match &file.name {
            FileName::Real(name) => name
                .to_string_lossy(FileNameDisplayPreference::Remapped)
                .into_owned(),
            _ => String::from("<unknown>"),
        };

        let lo_loc = source_map.lookup_char_pos(span.lo());
        let hi_loc = source_map.lookup_char_pos(span.hi());

        SerializableSpan {
            file_name,
            from_line: lo_loc.line,
            to_line: hi_loc.line,
        }
    }
}

impl<'tcx> LateLintPass<'tcx> for UnnecessaryLintAllow {
    fn check_crate_post(&mut self, _: &LateContext<'tcx>) {
        for finding in &self.findings {
            println!("Findings: {:?}", finding);
        }
    }

    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        for attr in cx.tcx.hir().attrs(rustc_hir::CRATE_HIR_ID) {
            self.collect_attribute(cx, attr, Scope::Crate, attr.span);
        }
    }

    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx rustc_hir::Item<'tcx>) {
        if item.span.from_expansion() {
            return;
        }

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
        if span.from_expansion() {
            return;
        }

        let hir_id = cx.tcx.local_def_id_to_hir_id(local_def_id);
        let attrs = cx.tcx.hir().attrs(hir_id);

        for attr in attrs.iter() {
            self.collect_attribute(cx, attr, Scope::Function, span);
        }

        // Use a visitor to check inner attributes
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

                // Continue visiting child nodes
                walk_expr(self, expr);
            }
        }

        let mut visitor = InnerAttrVisitor { cx, lint: self };
        walk_expr(&mut visitor, body.value);
    }
}
