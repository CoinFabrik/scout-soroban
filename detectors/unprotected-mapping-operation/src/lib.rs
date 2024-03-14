#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use if_chain::if_chain;
use rustc_hir::{
    def::Res,
    intravisit::{walk_body, walk_expr, Visitor},
    Expr, ExprKind, HirId, Param, PatKind, QPath, StmtKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use scout_audit_internal::Detector;

dylint_linting::declare_late_lint! {
    pub UNPROTECTED_MAPPING_OPERATION,
    Warn,
    Detector::UnprotectedMappingOperation.get_lint_message()
}

struct AuthStatus {
    authed: bool,
}

struct UnauthorizedAddress {
    span: Span,
    name: String,
}

struct UnprotectedMappingOperationFinder<'tcx, 'tcx_ref> {
    cx: &'tcx_ref LateContext<'tcx>,
    linked_addresses: Vec<(AuthStatus, Vec<HirId>)>,
    unauthorized_span: Vec<UnauthorizedAddress>,
}

const SOROBAN_MAP: &str = "soroban_sdk::Map";
const SOROBAN_ADDRESS: &str = "soroban_sdk::Address";

impl<'tcx> LateLintPass<'tcx> for UnprotectedMappingOperation {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: Span,
        _: rustc_span::def_id::LocalDefId,
    ) {
        let mut visitor = UnprotectedMappingOperationFinder {
            cx,
            linked_addresses: Vec::new(),
            unauthorized_span: Vec::new(),
        };

        visitor.parse_body_params(body.params);

        walk_body(&mut visitor, body);

        visitor
            .unauthorized_span
            .iter()
            .for_each(|unauthorized_address| {
                Detector::UnprotectedMappingOperation.span_lint_and_help(
                    cx,
                    UNPROTECTED_MAPPING_OPERATION,
                    unauthorized_address.span,
                    &format!(
                        "Address not authorized, please use `{}.require_auth();` prior to the mapping operation",
                        unauthorized_address.name
                    ),
                );
            });
    }
}

impl<'tcx> Visitor<'tcx> for UnprotectedMappingOperationFinder<'tcx, '_> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Block(block, _) = &expr.kind {
            block.stmts.iter().for_each(|stmt| {
                if_chain! {
                    if let StmtKind::Local(local) = &stmt.kind;
                    if self.get_node_type(local.hir_id) == SOROBAN_ADDRESS;
                    if let PatKind::Binding(_, target_hir_id, _, _) = &local.pat.kind;
                    if let Some(init) = &local.init;
                    let source_hir_id = self.get_expr_hir_id(init);
                    if let Some(source_hir_id) = source_hir_id;
                    then {
                        // Insert the new address into the linked_addresses
                        self.insert_new_address(source_hir_id, *target_hir_id);
                    }
                }
            })
        }

        if let ExprKind::MethodCall(method_path, method_expr, method_args, _) = &expr.kind {
            // Get the method expression type and check if it's a map with address
            let method_expr_type = self.get_node_type(method_expr.hir_id);

            if method_expr_type.starts_with(SOROBAN_MAP)
                && method_expr_type.contains(SOROBAN_ADDRESS)
            {
                // Iterate through the method arguments and check if any of them is an address and not authed
                method_args.iter().for_each(|arg| {
                    if_chain! {
                        if let Some(id) = self.get_expr_hir_id(arg);
                        if self.get_node_type(id).contains(SOROBAN_ADDRESS);
                        then {
                            // Obtain the linked_addresses record in wich the address id is contained
                            let linked_address = self.get_linked_address(id);

                            // If the address does not exist, of if it does but the AuthStatus is false, then we need to add it to the unauthorized_span
                            if linked_address.is_none() || !linked_address.unwrap().0.authed {
                                self.unauthorized_span.push(UnauthorizedAddress {
                                    span: expr.span,
                                    name: self.cx.tcx.hir().name(id).to_string(),
                                });
                            }
                        }
                    }
                });
            }

            // Check if the method call is a require_auth call and if it is, then we need to update the AuthStatus
            if_chain! {
                if method_expr_type.contains(SOROBAN_ADDRESS);
                if method_path.ident.name.as_str() == "require_auth";
                if let Some(id) = self.get_expr_hir_id(method_expr);
                then {
                    self.auth_address(id)
                }
            }
        }

        walk_expr(self, expr);
    }
}

impl<'tcx> UnprotectedMappingOperationFinder<'tcx, '_> {
    fn parse_body_params(&mut self, params: &'tcx [Param<'_>]) {
        params.iter().for_each(|param| {
            if self.get_node_type(param.hir_id) == SOROBAN_ADDRESS {
                self.linked_addresses
                    .push((AuthStatus { authed: false }, vec![param.pat.hir_id]));
            }
        });
    }

    fn get_node_type(&self, hir_id: HirId) -> String {
        self.cx.typeck_results().node_type(hir_id).to_string()
    }

    fn insert_new_address(&mut self, source_hir_id: HirId, target_hir_id: HirId) {
        if let Some((_, linked_addresses)) = self
            .linked_addresses
            .iter_mut()
            .find(|(_, addresses)| addresses.iter().any(|&id| id == source_hir_id))
        {
            linked_addresses.push(target_hir_id);
        }
    }

    fn get_expr_hir_id(&self, expr: &Expr) -> Option<HirId> {
        let mut stack = vec![expr];

        while let Some(current_expr) = stack.pop() {
            match current_expr.kind {
                ExprKind::MethodCall(_, call_expr, _, _) => stack.push(call_expr),
                ExprKind::Path(QPath::Resolved(_, path)) => match path.res {
                    Res::Local(hir_id) => return Some(hir_id),
                    _ => continue,
                },
                _ => continue,
            }
        }

        None
    }

    fn get_linked_address(&self, id: HirId) -> Option<&(AuthStatus, Vec<HirId>)> {
        self.linked_addresses.iter().find(|(_, linked_addresses)| {
            linked_addresses
                .iter()
                .any(|&address_hir_id| address_hir_id == id)
        })
    }

    fn auth_address(&mut self, id: HirId) {
        self.linked_addresses
            .iter_mut()
            .for_each(|(auth_status, linked_addresses)| {
                if linked_addresses
                    .iter()
                    .any(|&address_hir_id| address_hir_id == id)
                {
                    auth_status.authed = true;
                }
            });
    }
}
