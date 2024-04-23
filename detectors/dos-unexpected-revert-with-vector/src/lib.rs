#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![feature(let_chains)]

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use std::collections::HashSet;

use rustc_hir::intravisit::walk_expr;
use rustc_hir::intravisit::Visitor;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::mir::Const;
use rustc_middle::mir::{
    BasicBlock, BasicBlockData, BasicBlocks, Operand, Place, StatementKind, TerminatorKind,
};
use rustc_middle::ty::{Ty, TyKind};
use rustc_span::def_id::DefId;
use rustc_span::Span;
use scout_audit_clippy_utils::diagnostics::span_lint;

const LINT_MESSAGE: &str =
    "This vector operation is called without considering storage limitations";

dylint_linting::impl_late_lint! {
    pub UNEXPECTED_REVERT_WARN,
    Warn,
    "",
    UnexpectedRevertWarn::default(),
    {
        name: "Unexpected Revert Inserting to Storage",
        long_message: " It occurs by preventing transactions by other users from being successfully executed forcing the blockchain state to revert to its original state.",
        severity: "Medium",
        help: "https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/dos-unexpected-revert-with-vector",
        vulnerability_class: "Denial of Service",
    }
}

#[derive(Default)]
pub struct UnexpectedRevertWarn {}
impl UnexpectedRevertWarn {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'tcx> LateLintPass<'tcx> for UnexpectedRevertWarn {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: rustc_hir::intravisit::FnKind<'tcx>,
        _: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: Span,
        localdef: rustc_span::def_id::LocalDefId,
    ) {
        struct UnprotectedVectorFinder<'tcx, 'tcx_ref> {
            cx: &'tcx_ref LateContext<'tcx>,
            callers_def_id: HashSet<DefId>,
            push_def_id: Option<DefId>,
        }
        impl<'tcx> Visitor<'tcx> for UnprotectedVectorFinder<'tcx, '_> {
            fn visit_expr(&mut self, expr: &'tcx Expr<'_>) {
                if let ExprKind::MethodCall(path, _receiver, ..) = expr.kind {
                    let defid = self.cx.typeck_results().type_dependent_def_id(expr.hir_id);
                    let ty = Ty::new_foreign(self.cx.tcx, defid.unwrap());
                    if ty.to_string().contains("soroban_sdk::Vec")
                        && (path.ident.name.to_string() == "push_back"
                            || path.ident.name.to_string() == "push_front")
                    {
                        self.push_def_id = defid;
                    }
                }
                walk_expr(self, expr);
            }
        }

        let mut uvf_storage = UnprotectedVectorFinder {
            cx,
            callers_def_id: HashSet::default(),
            push_def_id: None,
        };

        walk_expr(&mut uvf_storage, body.value);

        let mir_body = cx.tcx.optimized_mir(localdef);

        struct CallersAndVecOps<'tcx> {
            callers: Vec<(&'tcx BasicBlockData<'tcx>, BasicBlock)>,
            vec_ops: Vec<(&'tcx BasicBlockData<'tcx>, BasicBlock)>,
        }

        fn find_caller_and_vec_ops_in_mir<'tcx>(
            bbs: &'tcx BasicBlocks<'tcx>,
            callers_def_id: HashSet<DefId>,
            push_def_id: Option<DefId>,
        ) -> CallersAndVecOps {
            let mut callers_vec = CallersAndVecOps {
                callers: vec![],
                vec_ops: vec![],
            };
            for (bb, bb_data) in bbs.iter().enumerate() {
                if bb_data.terminator.as_ref().is_none() {
                    continue;
                }
                let terminator = bb_data.terminator.clone().unwrap();
                if let TerminatorKind::Call { func, .. } = terminator.kind {
                    if let Operand::Constant(fn_const) = func
                        && let Const::Val(_const_val, ty) = fn_const.const_
                        && let TyKind::FnDef(def, _subs) = ty.kind()
                    {
                        if !callers_def_id.is_empty() {
                            for caller in &callers_def_id {
                                if caller == def {
                                    callers_vec
                                        .callers
                                        .push((bb_data, BasicBlock::from_usize(bb)));
                                }
                            }
                        } else {
                            for op in &push_def_id {
                                if op == def {
                                    callers_vec
                                        .vec_ops
                                        .push((bb_data, BasicBlock::from_usize(bb)));
                                }
                            }
                        }
                    }
                }
            }
            callers_vec
        }

        let caller_and_vec_ops = find_caller_and_vec_ops_in_mir(
            &mir_body.basic_blocks,
            uvf_storage.callers_def_id,
            uvf_storage.push_def_id,
        );

        if !caller_and_vec_ops.vec_ops.is_empty() {
            let unchecked_places = navigate_trough_basicblocks(
                &mir_body.basic_blocks,
                BasicBlock::from_u32(0),
                &caller_and_vec_ops,
                false,
                &mut vec![],
                &mut HashSet::<BasicBlock>::default(),
            );
            for place in unchecked_places {
                span_lint(cx, UNEXPECTED_REVERT_WARN, place.1, LINT_MESSAGE);
            }
        }

        fn navigate_trough_basicblocks<'tcx>(
            bbs: &'tcx BasicBlocks<'tcx>,
            bb: BasicBlock,
            caller_and_vec_ops: &CallersAndVecOps<'tcx>,
            after_comparison: bool,
            tainted_places: &mut Vec<Place<'tcx>>,
            visited_bbs: &mut HashSet<BasicBlock>,
        ) -> Vec<(Place<'tcx>, Span)> {
            let mut ret_vec = Vec::<(Place, Span)>::new();
            if visited_bbs.contains(&bb) {
                return ret_vec;
            } else {
                visited_bbs.insert(bb);
            }
            if bbs[bb].terminator.is_none() {
                return ret_vec;
            }
            for statement in &bbs[bb].statements {
                if let StatementKind::Assign(assign) = &statement.kind {
                    match &assign.1 {
                        rustc_middle::mir::Rvalue::Ref(_, _, origplace)
                        | rustc_middle::mir::Rvalue::AddressOf(_, origplace)
                        | rustc_middle::mir::Rvalue::Len(origplace)
                        | rustc_middle::mir::Rvalue::CopyForDeref(origplace) => {
                            if tainted_places
                                .clone()
                                .into_iter()
                                .any(|place| place == *origplace)
                            {
                                tainted_places.push(assign.0);
                            }
                        }
                        rustc_middle::mir::Rvalue::Use(operand) => match &operand {
                            Operand::Copy(origplace) | Operand::Move(origplace) => {
                                if tainted_places
                                    .clone()
                                    .into_iter()
                                    .any(|place| place == *origplace)
                                {
                                    tainted_places.push(assign.0);
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            match &bbs[bb].terminator().kind {
                TerminatorKind::SwitchInt { discr, targets } => {
                    let comparison_with_caller = match discr {
                        Operand::Copy(place) | Operand::Move(place) => {
                            tainted_places
                                .iter()
                                .any(|tainted_place| tainted_place == place)
                                || after_comparison
                        }
                        Operand::Constant(_cons) => after_comparison,
                    };
                    for target in targets.all_targets() {
                        ret_vec.append(&mut navigate_trough_basicblocks(
                            bbs,
                            *target,
                            caller_and_vec_ops,
                            comparison_with_caller,
                            tainted_places,
                            visited_bbs,
                        ));
                    }
                    return ret_vec;
                }
                TerminatorKind::Call {
                    destination,
                    args,
                    target,
                    fn_span,
                    ..
                } => {
                    for arg in args {
                        match arg {
                            Operand::Copy(origplace) | Operand::Move(origplace) => {
                                if tainted_places
                                    .clone()
                                    .into_iter()
                                    .any(|place| place == *origplace)
                                {
                                    tainted_places.push(*destination);
                                }
                            }
                            Operand::Constant(_) => {}
                        }
                    }
                    for caller in &caller_and_vec_ops.callers {
                        if caller.1 == bb {
                            tainted_places.push(*destination);
                        }
                    }
                    for map_op in &caller_and_vec_ops.vec_ops {
                        if map_op.1 == bb
                            && !after_comparison
                            && args.get(1).map_or(true, |f| {
                                f.place().is_some_and(|f| !tainted_places.contains(&f))
                            })
                        {
                            ret_vec.push((*destination, *fn_span))
                        }
                    }
                    if target.is_some() {
                        ret_vec.append(&mut navigate_trough_basicblocks(
                            bbs,
                            target.unwrap(),
                            caller_and_vec_ops,
                            after_comparison,
                            tainted_places,
                            visited_bbs,
                        ));
                    }
                }
                TerminatorKind::Assert { target, .. }
                | TerminatorKind::Goto { target, .. }
                | TerminatorKind::Drop { target, .. } => {
                    ret_vec.append(&mut navigate_trough_basicblocks(
                        bbs,
                        *target,
                        caller_and_vec_ops,
                        after_comparison,
                        tainted_places,
                        visited_bbs,
                    ));
                }
                TerminatorKind::Yield { resume, .. } => {
                    ret_vec.append(&mut navigate_trough_basicblocks(
                        bbs,
                        *resume,
                        caller_and_vec_ops,
                        after_comparison,
                        tainted_places,
                        visited_bbs,
                    ));
                }
                TerminatorKind::FalseEdge { real_target, .. }
                | TerminatorKind::FalseUnwind { real_target, .. } => {
                    ret_vec.append(&mut navigate_trough_basicblocks(
                        bbs,
                        *real_target,
                        caller_and_vec_ops,
                        after_comparison,
                        tainted_places,
                        visited_bbs,
                    ));
                }
                TerminatorKind::InlineAsm { destination, .. } => {
                    if destination.is_some() {
                        ret_vec.append(&mut navigate_trough_basicblocks(
                            bbs,
                            destination.unwrap(),
                            caller_and_vec_ops,
                            after_comparison,
                            tainted_places,
                            visited_bbs,
                        ));
                    }
                }
                _ => {}
            }
            ret_vec
        }
    }
}
