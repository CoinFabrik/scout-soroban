#![feature(rustc_private)]
#![feature(let_chains)]
extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_span;
extern crate rustc_middle;
extern crate rustc_type_ir;

use rustc_ast::{
    LitKind,
    LitIntType,
    Label,
};
use rustc_hir::def_id::LocalDefId;
use rustc_hir::{
    def::{
        Res,
    },
    intravisit::{
        walk_expr,
        FnKind,
        Visitor,
    },
    Expr,
    ExprKind,
    HirId,
    LangItem,
    LoopSource,
    MatchSource,
    PatKind,
    QPath,
    StmtKind,
    ExprField,
    Block,
    PatField,
    BindingAnnotation,
    Pat,
    Path,
    PathSegment,
    Ty,
};
use rustc_middle::ty::{
    TyKind,
    TyCtxt,
    Interner,
};
use rustc_lint::LateLintPass;
use rustc_span::{
    Span,
    symbol::{
        Ident,
    },
};
use scout_audit_internal::Detector;

dylint_linting::declare_late_lint! {
    pub ITERATOR_OVER_INDEXING,
    Warn,
    Detector::IteratorsOverIndexing.get_lint_message()
}

struct ForLoopVisitor<'a, 'b> {
    span_constant: Vec<Span>,
    cx: &'b rustc_lint::LateContext<'a>,
}
struct VectorAccessVisitor<'a, 'b> {
    index_id: HirId,
    has_vector_access: bool,
    cx: &'b rustc_lint::LateContext<'a>,
}

fn get_node_type<'a, 'b>(cx: &'b rustc_lint::LateContext<'a>, hir_id: &HirId) -> rustc_middle::ty::Ty<'a> {
    cx.typeck_results().node_type(*hir_id)
}

impl<'a, 'b> Visitor<'a> for VectorAccessVisitor<'a, 'b> {
    fn visit_expr(&mut self, expr: &'a Expr<'a>){
        let _ = (|| -> Result<(), ()>{
            let (path_segment, object, arguments, _) = expr_to_method_call(&expr.kind)?;
            let name = path_segment.ident.name.as_str();
            if !(name == "get" || name == "get_unchecked"){
                return Ok(());
            }
            let object_path = expr_to_path(&object.kind)?;
            let (_, object_path) = path_to_resolved(&object_path)?;
            let object_decl_hir_id = resolution_to_local(&object_path.res)?;

            let object_type = get_node_type(self.cx, object_decl_hir_id);
            let (def, _generic_args) = type_to_adt(object_type.kind())?;
            let type_name = self.cx.get_def_path(def.did())
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("::");

            if type_name != "soroban_sdk::vec::Vec"{
                return Ok(());
            }

            if arguments.len() != 1{
                return Ok(());
            }

            let index_qpath = expr_to_path(&arguments.first().unwrap().kind)?;
            let (_, index_path) = path_to_resolved(&index_qpath)?;
            let index_hir_id = resolution_to_local(&index_path.res)?;
            if *index_hir_id == self.index_id{
                self.has_vector_access = true;
            }
            Ok(())
        })();
        walk_expr(self, expr);
    }
}

//---------------------------------------------------------------------

fn type_to_adt<'hir>(kind: &'hir rustc_type_ir::sty::TyKind<TyCtxt<'hir>>) -> Result<(&'hir <TyCtxt<'hir> as Interner>::AdtDef, &'hir <TyCtxt<'hir> as Interner>::GenericArgsRef), ()>{
    if let TyKind::Adt(a, b) = kind{
        Ok((&a, &b))
    }else{
        Err(())
    }
}

//---------------------------------------------------------------------

fn stmt_to_expr<'hir>(kind: &'hir StmtKind<'hir>) -> Result<&'hir Expr<'hir>, ()>{
    if let StmtKind::Expr(a) = kind{
        Ok(&a)
    }else{
        Err(())
    }
}

//---------------------------------------------------------------------

fn expr_to_index<'hir>(kind: &'hir ExprKind<'hir>) -> Result<(&'hir Expr<'hir>, &'hir Expr<'hir>, Span), ()>{
    if let ExprKind::Index(a, b, c) = kind{
        Ok((a, b, c.clone()))
    }else{
        Err(())
    }
}

fn expr_to_drop_temps<'hir>(kind: &'hir ExprKind<'hir>) -> Result<&'hir Expr<'hir>, ()>{
    if let ExprKind::DropTemps(a) = kind{
        Ok(a)
    }else{
        Err(())
    }
}

fn expr_to_match<'hir>(kind: &'hir ExprKind<'hir>) -> Result<(&'hir Expr<'hir>, &'hir [rustc_hir::Arm<'hir>], MatchSource), ()>{
    if let ExprKind::Match(a, b, c) = kind{
        Ok((a, b, c.clone()))
    }else{
        Err(())
    }
}

fn expr_to_call<'hir>(kind: &'hir ExprKind<'hir>) -> Result<(&'hir Expr<'hir>, &'hir [Expr<'hir>]), ()>{
    if let ExprKind::Call(a, b) = kind{
        Ok((a, b))
    }else{
        Err(())
    }
}

fn expr_to_path<'hir>(kind: &'hir ExprKind<'hir>) -> Result<QPath<'hir>, ()>{
    if let ExprKind::Path(a) = kind{
        Ok(a.clone())
    }else{
        Err(())
    }
}

fn expr_to_struct<'hir>(kind: &'hir ExprKind<'hir>) -> Result<(&'hir QPath<'hir>, &'hir [ExprField<'hir>], Option<&'hir Expr<'hir>>), ()>{
    if let ExprKind::Struct(a, b, c) = kind{
        Ok((a, b, c.clone()))
    }else{
        Err(())
    }
}

fn expr_to_lit<'hir>(kind: &'hir ExprKind<'hir>) -> Result<&'hir rustc_hir::Lit, ()>{
    if let ExprKind::Lit(a) = kind{
        Ok(a)
    }else{
        Err(())
    }
}

fn expr_to_loop<'hir>(kind: &'hir ExprKind<'hir>) -> Result<(&'hir Block<'hir>, &Option<Label>, LoopSource, &Span), ()>{
    if let ExprKind::Loop(a, b, c, d) = kind{
        Ok((a, b, c.clone(), d))
    }else{
        Err(())
    }
}

fn expr_to_method_call<'hir>(kind: &'hir ExprKind<'hir>) -> Result<(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span), ()>{
    if let ExprKind::MethodCall(a, b, c, d) = kind{
        Ok((a, b, c, d.clone()))
    }else{
        Err(())
    }
}

//---------------------------------------------------------------------

fn path_to_lang_item(path: &QPath) -> Result<(LangItem, Span, Option<HirId>), ()>{
    if let QPath::LangItem(a, b, c) = path{
        Ok((a.clone(), b.clone(), c.clone()))
    }else{
        Err(())
    }
}

fn path_to_resolved<'hir>(path: &'hir QPath<'hir>) -> Result<(&'hir Option<&'hir Ty<'hir>>, &'hir Path<'hir>), ()>{
    if let QPath::Resolved(a, b) = path{
        Ok((a, b))
    }else{
        Err(())
    }
}

//---------------------------------------------------------------------

fn resolution_to_local(resolution: &Res) -> Result<&HirId, ()>{
    if let Res::Local(a) = resolution{
        Ok(a)
    }else{
        Err(())
    }
}

//---------------------------------------------------------------------

fn lit_to_int(kind: &LitKind) -> Result<(u128, LitIntType), ()>{
    if let LitKind::Int(a, b) = kind{
        Ok((a.clone(), b.clone()))
    }else{
        Err(())
    }
}

//---------------------------------------------------------------------

fn pattern_to_struct<'hir>(pat: &'hir PatKind<'hir>) -> Result<(&QPath<'hir>, &'hir [PatField<'hir>], bool), ()>{
    if let PatKind::Struct(a, b, c) = pat{
        Ok((a, b, c.clone()))
    }else{
        Err(())
    }
}

fn pattern_to_binding<'hir>(pat: &'hir PatKind<'hir>) -> Result<(&BindingAnnotation, &HirId, &Ident, &Option<&'hir Pat<'hir>>), ()>{
    if let PatKind::Binding(a, b, c, d) = pat{
        Ok((a, b, c, d))
    }else{
        Err(())
    }
}

//---------------------------------------------------------------------

fn is_range(item: LangItem) -> bool{
    match item{
        LangItem::Range => true,
        LangItem::RangeInclusiveStruct => true,
        LangItem::RangeInclusiveNew => true,
        _ => false,
    }
}

//---------------------------------------------------------------------

fn handle_expr<'a, 'b>(me: &mut ForLoopVisitor<'a, 'b>, expr: &'a Expr<'a>) -> Result<(), ()>{
    //Ignore DropTemps()
    let expr = expr_to_drop_temps(&expr.kind).or(Ok(&expr))?;

    let (match_expr, arms, source) = expr_to_match(&expr.kind)?;
    if source != MatchSource::ForLoopDesugar{
        return Ok(());
    }
    let (func, args) = expr_to_call(&match_expr.kind)?;
    let qpath = expr_to_path(&func.kind)?;
    let (item, _, _) = path_to_lang_item(&qpath)?;
    if item != LangItem::IntoIterIntoIter{
        return Ok(());
    }
    if !args.first().is_some(){
        return Ok(());
    }
    let (qpath, fields, _) = expr_to_struct(&args.first().unwrap().kind)?;
    let (langitem, _, _) = path_to_lang_item(&qpath)?;
    if !is_range(langitem){
        return Ok(());
    }
    if !fields.last().is_some(){
        return Ok(());
    }
    let lit = expr_to_lit(&fields.last().unwrap().expr.kind)?;
    let _ = lit_to_int(&lit.node)?;
    if !arms.first().is_some(){
        return Ok(());
    }
    let (block, _, loopsource, _) = expr_to_loop(&arms.first().unwrap().body.kind)?;
    if loopsource != LoopSource::ForLoop{
        return Ok(());
    }
    if !block.stmts.first().is_some(){
        return Ok(());
    }
    let stmtexpr = stmt_to_expr(&block.stmts.first().unwrap().kind)?;
    let (_, some_none_arms, match_source) = expr_to_match(&stmtexpr.kind)?;
    if match_source != MatchSource::ForLoopDesugar{
        return Ok(());
    }

    let mut visitor = VectorAccessVisitor {
        has_vector_access: false,
        index_id: expr.hir_id,
        cx: me.cx,
    };
    for arm in some_none_arms {
        let hir_id = (|| -> Result<HirId, ()>{
            let (qpath, pats, _) = pattern_to_struct(&arm.pat.kind)?;
            let (item_type, _, _) = path_to_lang_item(qpath)?;
            if item_type != LangItem::OptionSome{
                return Err(());
            }
            if !pats.last().is_some(){
                return Err(());
            }
            let (_, hir_id, _ident, _) = pattern_to_binding(&pats.last().unwrap().pat.kind)?;
            Ok(*hir_id)
        })();

        if let Ok(hir_id) = hir_id{
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
    fn visit_expr(& mut self, expr: &'a rustc_hir::Expr<'a>) {
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
        //dbg!(decl, body);
        if let FnKind::Method(_ident, _sig) = kind {
            let span_constant = {
                let mut visitor = ForLoopVisitor{
                    span_constant: vec![],
                    cx,
                };
                walk_expr(&mut visitor, body.value);
                visitor.span_constant
            };

            for span in span_constant {
                Detector::IteratorsOverIndexing.span_lint_and_help(
                    cx,
                    ITERATOR_OVER_INDEXING,
                    span,
                    "Instead, use an iterator or index to `.len()`.",
                );
            }
        }
    }
}
