#![feature(rustc_private)]
#![feature(let_chains)]
extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_type_ir;

use std::collections::HashSet;

use if_chain::if_chain;
use rustc_ast::LitKind;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::{
    def::Res,
    intravisit::{walk_expr, FnKind, Visitor},
    BinOpKind, Body, BorrowKind, Expr, ExprKind, FnDecl, HirId, Mutability, Param, PatKind, Path,
    PathSegment, QPath, Ty, TyKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::middle::privacy::Level;
use rustc_span::Span;
use scout_audit_clippy_utils::diagnostics::span_lint_and_help;

const LINT_MESSAGE: &str = "Not checking for a zero-address could lead to an insecure contract";

dylint_linting::declare_late_lint! {
    pub ZERO_ADDRESS,
    Warn,
    LINT_MESSAGE,
    {
        name: "Zero Address",
        long_message: "In the elliptic curve used by Soroban (Ed25519), the zero address has a known private key. Using this address as a null value (for example, for a contract's administrative account) is a common mistake, and can lead to losing control of the contract, instead of the contract being locked.",
        severity: "Minor",
        help: "https://coinfabrik.github.io/scout-soroban/docs/detectors/zero-or-test-address",
        vulnerability_class: "Validations and error handling",
    }
}

//---------------------------------------------------------------------

fn type_to_path<'hir>(kind: &'hir TyKind<'hir>) -> Result<&'hir QPath<'hir>, ()> {
    if let TyKind::Path(a) = kind {
        Ok(a)
    } else {
        Err(())
    }
}

//---------------------------------------------------------------------

fn expr_to_call<'hir>(
    kind: &'hir ExprKind<'hir>,
) -> Result<(&'hir Expr<'hir>, &'hir [Expr<'hir>]), ()> {
    if let ExprKind::Call(a, b) = kind {
        Ok((a, b))
    } else {
        Err(())
    }
}

fn expr_to_path<'hir>(kind: &'hir ExprKind<'hir>) -> Result<QPath<'hir>, ()> {
    if let ExprKind::Path(a) = kind {
        Ok(*a)
    } else {
        Err(())
    }
}

fn expr_to_lit<'hir>(kind: &'hir ExprKind<'hir>) -> Result<&'hir rustc_hir::Lit, ()> {
    if let ExprKind::Lit(a) = kind {
        Ok(a)
    } else {
        Err(())
    }
}

fn expr_to_address_of<'hir>(
    kind: &'hir ExprKind<'hir>,
) -> Result<(&BorrowKind, &Mutability, &'hir Expr<'hir>), ()> {
    if let ExprKind::AddrOf(a, b, c) = kind {
        Ok((a, b, c))
    } else {
        Err(())
    }
}

//---------------------------------------------------------------------

fn path_to_resolved<'hir>(
    path: &'hir QPath<'hir>,
) -> Result<(&'hir Option<&'hir Ty<'hir>>, &'hir Path<'hir>), ()> {
    if let QPath::Resolved(a, b) = path {
        Ok((a, b))
    } else {
        Err(())
    }
}

fn path_to_type_relative<'hir>(
    path: &'hir QPath<'hir>,
) -> Result<(&'hir Ty<'hir>, &'hir PathSegment<'hir>), ()> {
    if let QPath::TypeRelative(a, b) = path {
        Ok((a, b))
    } else {
        Err(())
    }
}

//---------------------------------------------------------------------

fn resolution_to_local(resolution: &Res) -> Result<&HirId, ()> {
    if let Res::Local(a) = resolution {
        Ok(a)
    } else {
        Err(())
    }
}

//---------------------------------------------------------------------

fn get_node_type<'a>(cx: &rustc_lint::LateContext<'a>, hir_id: &HirId) -> rustc_middle::ty::Ty<'a> {
    cx.typeck_results().node_type(*hir_id)
}

fn match_expr_as_function_call<'hir>(
    expr: &'hir Expr<'hir>,
    type_name: &str,
    function_name: &str,
) -> Result<&'hir [Expr<'hir>], ()> {
    let (expr_fn, exprs_args) = expr_to_call(&expr.kind)?;
    let qpath = expr_to_path(&expr_fn.kind)?;
    let (ty, path) = path_to_type_relative(&qpath)?;
    let qpath2 = type_to_path(&ty.kind)?;
    let (_, path2) = path_to_resolved(qpath2)?;
    let type_id = path2
        .segments
        .iter()
        .map(|x| x.ident.to_string())
        .collect::<Vec<_>>()
        .join("::");
    //TODO: Need a better method to determine the type.
    if type_id != type_name {
        return Err(());
    }

    if path.ident.to_string() != function_name {
        return Err(());
    }

    Ok(exprs_args)
}

fn remove_address_of<'hir>(expr: &'hir Expr<'hir>) -> &'hir Expr<'hir> {
    match expr_to_address_of(&expr.kind) {
        Ok((_, _, sub)) => sub,
        Err(_) => expr,
    }
}

fn expr_is_zero_addr<'hir>(expr: &'hir Expr<'hir>, cx: &rustc_lint::LateContext) -> bool {
    let r = || -> Result<bool, ()> {
        let args = || -> Result<&'hir [Expr<'hir>], ()> {
            let r = match_expr_as_function_call(expr, "soroban_sdk::Address", "from_string");
            if r.is_ok() {
                return r;
            }
            match_expr_as_function_call(expr, "Address", "from_string")
        }()?;
        if args.len() != 1 {
            return Ok(false);
        }
        let expr = remove_address_of(args.first().unwrap());
        let args = || -> Result<&'hir [Expr<'hir>], ()> {
            let r = match_expr_as_function_call(expr, "soroban_sdk::String", "from_bytes");
            if r.is_ok() {
                return r;
            }
            match_expr_as_function_call(expr, "String", "from_bytes")
        }()?;

        if args.len() != 2 {
            return Ok(false);
        }
        let env_arg = args.first().unwrap();
        let addr_arg = args.get(1).unwrap();

        //Check that the first argument is either of type soroban_sdk::Env or of type &soroban_sdk::Env.
        let env_arg = remove_address_of(env_arg);
        let env_path = expr_to_path(&env_arg.kind)?;
        let (_, env_resolved) = path_to_resolved(&env_path)?;
        let env_id = resolution_to_local(&env_resolved.res)?;
        let env_type = get_node_type(cx, env_id);
        if env_type.to_string() != "soroban_sdk::Env" {
            return Ok(false);
        }

        let addr_arg = expr_to_lit(&addr_arg.kind)?;

        if let LitKind::ByteStr(data, _) = &addr_arg.node as &LitKind {
            if data.len() != 56 {
                return Ok(false);
            }
            //'G'
            if *data.first().unwrap() != 71_u8 {
                return Ok(false);
            }
            //52 times 'A'
            for i in 1..53 {
                if *data.get(i).unwrap() != 65_u8 {
                    return Ok(false);
                }
            }

            //'W'
            if *data.get(53).unwrap() != 87_u8 {
                return Ok(false);
            }

            //'H'
            if *data.get(54).unwrap() != 72_u8 {
                return Ok(false);
            }

            //'F'
            Ok(*data.get(55).unwrap() == 70_u8)
        } else {
            Ok(false)
        }
    }();
    r.unwrap_or(false)
}

fn get_param_hir_id(param: &Param) -> Option<HirId> {
    if let PatKind::Binding(_, b, _, _) = param.pat.kind {
        Some(b)
    } else {
        None
    }
}

fn get_path_local_hir_id(expr: &Expr<'_>) -> Option<HirId> {
    if let ExprKind::Path(qpath) = &expr.kind
        && let QPath::Resolved(_, path) = qpath
        && let Res::Local(local) = path.res
    {
        Some(local)
    } else {
        None
    }
}

impl<'tcx> LateLintPass<'tcx> for ZeroAddress {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'_>,
        body: &'tcx Body<'_>,
        _: Span,
        id: LocalDefId,
    ) {
        if !cx
            .effective_visibilities
            .is_public_at_level(id, Level::Reexported)
        {
            return;
        }

        struct ZeroCheckStorage<'tcx, 'tcx_ref> {
            cx: &'tcx_ref LateContext<'tcx>,
            acc_id_params: Vec<&'tcx Param<'tcx>>,
            checked_params: HashSet<&'tcx HirId>,
        }

        impl<'tcx> Visitor<'tcx> for ZeroCheckStorage<'tcx, '_> {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                //Look if those params are compared with zero address
                if let ExprKind::If(mut cond, _, _) = &expr.kind {
                    if let ExprKind::DropTemps(drop) = cond.kind {
                        cond = drop;
                    }
                    if_chain! {
                        if let ExprKind::Binary(op, lexpr, rexpr) = cond.kind;
                        if BinOpKind::Eq == op.node;
                        then {
                            for param in &self.acc_id_params {
                                let param_hir_id = get_param_hir_id(param);
                                if (param_hir_id == get_path_local_hir_id(lexpr)
                                    && expr_is_zero_addr(rexpr, self.cx)) ||
                                    (param_hir_id == get_path_local_hir_id(rexpr)
                                    && expr_is_zero_addr(lexpr, self.cx)) {
                                    self.checked_params.insert(&param.hir_id);
                                }
                            }
                        }
                    }
                }
                walk_expr(self, expr);
            }
        }

        let mut zerocheck_storage = ZeroCheckStorage {
            cx,
            acc_id_params: Vec::default(),
            checked_params: HashSet::default(),
        };

        let mir_body = cx.tcx.optimized_mir(id);
        for (arg, hir_param) in mir_body.args_iter().zip(body.params.iter()) {
            let arg = &mir_body.local_decls[arg];
            if arg.ty.to_string() == "soroban_sdk::Address" {
                zerocheck_storage.acc_id_params.push(hir_param);
            }
        }

        // If no arguments of accountId type is found, ignore this function
        if zerocheck_storage.acc_id_params.is_empty() {
            return;
        }

        walk_expr(&mut zerocheck_storage, body.value);

        for param in zerocheck_storage.acc_id_params {
            if zerocheck_storage.checked_params.contains(&param.hir_id) {
                continue;
            }
            span_lint_and_help(
                cx,
                ZERO_ADDRESS,
                param.span,
                LINT_MESSAGE,
                None,
                "This function should check if the AccountId passed is zero and revert if it is",
            );
        }
    }
}
