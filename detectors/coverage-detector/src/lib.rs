#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_span;

use rustc_ast::{
    visit::{self, FnKind, Visitor},
    AttrKind, Attribute, Expr, ExprKind, Item, ItemKind, NodeId,
};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_span::{Span, Symbol};
use std::collections::HashSet;

const LINT_MESSAGE: &str = "This function lacks test coverage.";

dylint_linting::impl_pre_expansion_lint! {
    pub COVERAGE_DETECTOR,
    Warn,
    LINT_MESSAGE,
    CoverageDetector::default(),
    {
        name: "Missing Test Coverage",
        long_message: "Functions should have corresponding test coverage to ensure code quality and prevent regressions.",
        severity: "Warning",
        help: "Consider adding a test function for this code.",
        vulnerability_class: "Code Quality",
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct SorobanFn {
    name: String,
    span: Span,
    is_tested: bool,
}

#[derive(Default, Debug)]
pub struct CoverageDetector {
    soroban_functions: HashSet<SorobanFn>,
    soroban_impl_spans: HashSet<Span>,
}

impl CoverageDetector {
    fn is_soroban_impl(&mut self, attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| {
            if let AttrKind::Normal(item) = &attr.kind {
                item.item.path.segments.last().map_or(false, |seg| {
                    seg.ident.name == Symbol::intern("contractimpl")
                })
            } else {
                false
            }
        })
    }

    fn is_soroban_function(&self, span: Span) -> bool {
        self.soroban_impl_spans
            .iter()
            .any(|soroban_span| soroban_span.contains(span))
    }

    fn is_test_function(&self, attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| {
            if let AttrKind::Normal(item) = &attr.kind {
                item.item
                    .path
                    .segments
                    .last()
                    .map_or(false, |seg| seg.ident.name == Symbol::intern("test"))
            } else {
                false
            }
        })
    }
}

struct MethodCallVisitor {
    method_name: String,
    has_method_call: bool,
}

impl<'ast> Visitor<'ast> for MethodCallVisitor {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        if let ExprKind::MethodCall(method_call) = &expr.kind {
            if method_call.seg.ident.name.to_string() == self.method_name {
                if let ExprKind::Path(_, path) = &method_call.receiver.kind {
                    // Get the type of the receiver
                }
                self.has_method_call = true;
            }
        }

        visit::walk_expr(self, expr);
    }
}

impl EarlyLintPass for CoverageDetector {
    fn check_crate_post(&mut self, _: &EarlyContext<'_>, _: &rustc_ast::Crate) {
        println!("Checking for untested functions...");
        println!("-----------------------------------");
        println!();
        println!("The following functions lack test coverage:");
        println!();
        self.soroban_functions.iter().for_each(|soroban_fn| {
            if !soroban_fn.is_tested {
                println!("Function '{}' lacks test coverage.", soroban_fn.name);
            }
        });
    }

    fn check_item(&mut self, _: &EarlyContext<'_>, item: &Item) {
        if let ItemKind::Fn(fn_) = &item.kind {
            if self.is_test_function(&item.attrs) {
                if let Some(body) = &fn_.body {
                    // Create a temporary HashSet to store newly tested functions
                    let mut newly_tested = HashSet::new();

                    for soroban_function in &self.soroban_functions {
                        let mut visitor = MethodCallVisitor {
                            method_name: soroban_function.name.clone(),
                            has_method_call: false,
                        };

                        visitor.visit_block(body);

                        if visitor.has_method_call {
                            newly_tested.insert(soroban_function.name.clone());
                        }
                    }

                    // Update the is_tested status for functions that were called
                    self.soroban_functions = self
                        .soroban_functions
                        .drain()
                        .map(|mut func| {
                            if newly_tested.contains(&func.name) {
                                func.is_tested = true;
                            }
                            func
                        })
                        .collect();
                }
            }
        } else if let ItemKind::Impl(_) = &item.kind {
            if self.is_soroban_impl(&item.attrs) {
                self.soroban_impl_spans.insert(item.span);
            }
        }
    }

    fn check_fn(&mut self, _: &EarlyContext<'_>, fn_kind: FnKind<'_>, span: Span, _: NodeId) {
        if let FnKind::Fn(_, ident, ..) = fn_kind {
            if self.is_soroban_function(span) {
                self.soroban_functions.insert(SorobanFn {
                    name: ident.name.to_string(),
                    span,
                    is_tested: false,
                });
            }
        };
    }
}
