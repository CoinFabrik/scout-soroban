extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_middle;

use rustc_hir::HirId;
use rustc_lint::LateContext;
use rustc_middle::ty::Ty;

/// Get the type of a node, if it exists.
pub fn get_node_type_opt<'tcx>(cx: &LateContext<'tcx>, hir_id: &HirId) -> Option<Ty<'tcx>> {
    cx.typeck_results().node_type_opt(*hir_id)
}
