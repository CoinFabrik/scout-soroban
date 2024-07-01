#![feature(rustc_private)]
#![feature(let_chains)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_type_ir;

use rustc_hir::{
    def_id::LocalDefId,
    intravisit::{walk_expr, FnKind, Visitor},
    Expr, HirId, LangItem, LoopSource, MatchSource,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use scout_audit_clippy_utils::diagnostics::span_lint_and_help;
use utils::{
    expr_to_call, expr_to_drop_temps, expr_to_lit, expr_to_loop, expr_to_match,
    expr_to_method_call, expr_to_path, expr_to_struct, get_node_type, is_range, lit_to_int,
    path_to_lang_item, path_to_resolved, pattern_to_binding, pattern_to_struct,
    resolution_to_local, stmt_to_expr, type_to_adt,
};

const LINT_MESSAGE: &str =
    "Hardcoding an index could lead to panic if the top bound is out of bounds.";

dylint_linting::declare_late_lint! {
    pub ITERATOR_OVER_INDEXING,
    Warn,
    LINT_MESSAGE,
    {
        name: "Iterators Over Indexing",
        long_message: "Instead, use an iterator or index to `.len()`.",
        severity: "Medium",
        help: "https://coinfabrik.github.io/scout-soroban/docs/detectors/iterators-over-indexing",
        vulnerability_class: "Incorrect Use of Indexing",
    }
}

struct ForLoopVisitor<'a, 'b> {
    span_constant: Vec<Span>,
    cx: &'b LateContext<'a>,
}
struct VectorAccessVisitor<'a, 'b> {
    index_id: HirId,
    has_vector_access: bool,
    cx: &'b LateContext<'a>,
}

impl<'a, 'b> Visitor<'a> for VectorAccessVisitor<'a, 'b> {
    fn visit_expr(&mut self, expr: &'a Expr<'a>) {
        let _ = (|| -> Result<(), ()> {
            let (path_segment, object, arguments, _) = expr_to_method_call(&expr.kind)?;
            let name = path_segment.ident.name.as_str();
            if !(name == "get" || name == "get_unchecked") {
                return Ok(());
            }
            let object_path = expr_to_path(&object.kind)?;
            let (_, object_path) = path_to_resolved(&object_path)?;
            let object_decl_hir_id = resolution_to_local(&object_path.res)?;

            let object_type = get_node_type(self.cx, object_decl_hir_id);
            let (def, _generic_args) = type_to_adt(object_type.kind())?;
            let type_name = self
                .cx
                .get_def_path(def.did())
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("::");

            if type_name != "soroban_sdk::vec::Vec" {
                return Ok(());
            }

            if arguments.len() != 1 {
                return Ok(());
            }

            let index_qpath = expr_to_path(&arguments.first().unwrap().kind)?;
            let (_, index_path) = path_to_resolved(&index_qpath)?;
            let index_hir_id = resolution_to_local(&index_path.res)?;
            if *index_hir_id == self.index_id {
                self.has_vector_access = true;
            }
            Ok(())
        })();
        walk_expr(self, expr);
    }
}

fn handle_expr<'a>(me: &mut ForLoopVisitor<'a, '_>, expr: &'a Expr<'a>) -> Result<(), ()> {
    //Ignore DropTemps()
    let expr = expr_to_drop_temps(&expr.kind).or(Ok(expr))?;

    let (match_expr, arms, source) = expr_to_match(&expr.kind)?;
    if source != MatchSource::ForLoopDesugar {
        return Ok(());
    }
    let (func, args) = expr_to_call(&match_expr.kind)?;
    let qpath = expr_to_path(&func.kind)?;
    let (item, _) = path_to_lang_item(&qpath)?;
    if item != LangItem::IntoIterIntoIter {
        return Ok(());
    }
    if args.first().is_none() {
        return Ok(());
    }
    let (qpath, fields, _) = expr_to_struct(&args.first().unwrap().kind)?;
    let (langitem, _) = path_to_lang_item(qpath)?;
    if !is_range(langitem) {
        return Ok(());
    }
    if fields.last().is_none() {
        return Ok(());
    }
    let lit = expr_to_lit(&fields.last().unwrap().expr.kind)?;
    let _ = lit_to_int(&lit.node)?;
    if arms.first().is_none() {
        return Ok(());
    }
    let (block, _, loopsource, _) = expr_to_loop(&arms.first().unwrap().body.kind)?;
    if loopsource != LoopSource::ForLoop {
        return Ok(());
    }
    if block.stmts.first().is_none() {
        return Ok(());
    }
    let stmtexpr = stmt_to_expr(&block.stmts.first().unwrap().kind)?;
    let (_, some_none_arms, match_source) = expr_to_match(&stmtexpr.kind)?;
    if match_source != MatchSource::ForLoopDesugar {
        return Ok(());
    }

    let mut visitor = VectorAccessVisitor {
        has_vector_access: false,
        index_id: expr.hir_id,
        cx: me.cx,
    };
    for arm in some_none_arms {
        let hir_id = (|| -> Result<HirId, ()> {
            let (qpath, pats, _) = pattern_to_struct(&arm.pat.kind)?;
            let (item_type, _) = path_to_lang_item(qpath)?;
            if item_type != LangItem::OptionSome {
                return Err(());
            }
            if pats.last().is_none() {
                return Err(());
            }
            let (_, hir_id, _ident, _) = pattern_to_binding(&pats.last().unwrap().pat.kind)?;
            Ok(*hir_id)
        })();

        if let Ok(hir_id) = hir_id {
            visitor.index_id = hir_id;
            walk_expr(&mut visitor, arm.body);
        }
    }

    if visitor.has_vector_access {
        me.span_constant.push(expr.span);
    }

    Ok(())
}

impl<'a, 'b> Visitor<'a> for ForLoopVisitor<'a, 'b> {
    fn visit_expr(&mut self, expr: &'a rustc_hir::Expr<'a>) {
        let _ = handle_expr(self, expr);
        walk_expr(self, expr);
    }
}

impl<'tcx> LateLintPass<'tcx> for IteratorOverIndexing {
    fn check_fn(
        &mut self,
        cx: &rustc_lint::LateContext<'tcx>,
        kind: rustc_hir::intravisit::FnKind<'tcx>,
        _decl: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: Span,
        _: LocalDefId,
    ) {
        if let FnKind::Method(_ident, _sig) = kind {
            let span_constant = {
                let mut visitor = ForLoopVisitor {
                    span_constant: vec![],
                    cx,
                };
                walk_expr(&mut visitor, body.value);
                visitor.span_constant
            };

            for span in span_constant {
                span_lint_and_help(
                    cx,
                    ITERATOR_OVER_INDEXING,
                    span,
                    LINT_MESSAGE,
                    None,
                    "Instead, use an iterator or index to `.len()`.",
                );
            }
        }
    }
}
