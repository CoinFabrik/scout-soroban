#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::def_id::LocalDefId;
use rustc_hir::intravisit::Visitor;
use rustc_hir::intravisit::{walk_expr, FnKind};
use rustc_hir::{Body, FnDecl};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use scout_audit_internal::Detector;

dylint_linting::declare_late_lint! {
    /// ### What it does
    /// Checks for calls to env::set_contract_storage.
    ///
    /// ### Why is this bad?
    /// Functions using keys as variables without proper access control or input sanitation can allow users to perform changes in arbitrary memory locations.
    ///
    /// ### Known problems
    /// Only check the function call, so false positives could result.
    ///
    /// ### Example
    /// ```rust
    /// fn set_contract_storage(
    ///     &mut self,
    ///     user_input_key: [u8; 68],
    ///     user_input_data: u128,
    /// ) -> Result<()> {
    ///     env::set_contract_storage(&user_input_key, &user_input_data);
    ///     Ok(())
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// fn set_contract_storage(
    ///     &mut self,
    ///     user_input_key: [u8; 68],
    ///     user_input_data: u128,
    /// ) -> Result<()> {
    ///     if self.env().caller() == self.owner {
    ///         env::set_contract_storage(&user_input_key, &user_input_data);
    ///         Ok(())
    ///     } else {
    ///         Err(Error::UserNotOwner)
    ///     }
    /// }
    /// ```
    pub SET_STORAGE_WARN,
    Warn,
    Detector::SetContractStorage.get_lint_message()
}

impl<'tcx> LateLintPass<'tcx> for SetStorageWarn {
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
                match &expr.kind {
                    ExprKind::MethodCall(path, _, _, _) => {
                        let method_name = path.ident.name.as_str();
                        if method_name == "require_auth" {
                            self.found_auth = true;
                        } else if method_name == "storage" && !self.found_auth {
                            self.storage_without_auth.push(expr.span);
                        }
                    }
                    _ => {}
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
            Detector::SetContractStorage.span_lint_and_help(
                cx,
                SET_STORAGE_WARN,
                span,
                "Ensure user.require_auth() is called before accessing storage.",
            );
        }
    }
}
