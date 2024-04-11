#![feature(rustc_private)]

extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::{
    def_id::LocalDefId,
    intravisit::{walk_expr, FnKind, Visitor},
    Body, Expr, ExprKind, FnDecl,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use scout_audit_clippy_utils::diagnostics::span_lint_and_help;

const LINT_MESSAGE: &str = "Abitrary users should not have control over keys because it implies writing any value of left mapping, lazy variable, or the main struct of the contract located in position 0 of the storage";

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// Checks for calls to env.storage() without a prior call to env.require_auth()
    ///
    /// ### Why is this bad?
    /// Functions using keys as variables without proper access control or input sanitation can allow users to perform changes in arbitrary memory locations.
    ///
    /// ### Known problems
    /// Only check the function call, so false positives could result.
    ///
    /// ### Example
    /// ```rust
    /// fn set_contract_storage(env: Env) {
    ///   let _storage = env.storage().instance();
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// fn set_contract_storage(env: Env, user: Address) {
    ///   user.require_auth();
    ///   let _storage = env.storage().instance();
    /// }
    /// ```
    pub SET_CONTRACT_STORAGE,
    Warn,
    LINT_MESSAGE,
    {
        name: "Set Contract Storage",
        long_message: "Functions using keys as variables without proper access control or input sanitation can allow users to perform changes in arbitrary memory locations.",
        severity: "Critical",
        help: "https://coinfabrik.github.io/scout/docs/vulnerabilities/set-contract-storage",
        vulnerability_class: "Authorization",
    }
}

impl<'tcx> LateLintPass<'tcx> for SetContractStorage {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        _: LocalDefId,
    ) {
        struct SetContractStorageVisitor {
            found_auth: bool,
            storage_without_auth: Vec<Span>,
        }

        impl<'tcx> Visitor<'tcx> for SetContractStorageVisitor {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if let ExprKind::MethodCall(path, _, _, _) = &expr.kind {
                    let method_name = path.ident.name.as_str();
                    if method_name == "require_auth" {
                        self.found_auth = true;
                    } else if method_name == "storage" && !self.found_auth {
                        self.storage_without_auth.push(expr.span);
                    }
                }
                walk_expr(self, expr);
            }
        }

        let mut visitor = SetContractStorageVisitor {
            found_auth: false,
            storage_without_auth: Vec::new(),
        };

        walk_expr(&mut visitor, body.value);

        for span in visitor.storage_without_auth {
            span_lint_and_help(
                cx,
                SET_CONTRACT_STORAGE,
                span,
                LINT_MESSAGE,
                None,
                "Ensure that the caller is authorized to use storage",
            );
        }
    }
}
