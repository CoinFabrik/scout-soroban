extern crate rustc_hir;
extern crate rustc_span;

use rustc_hir::{Expr, ExprKind, QPath};
use rustc_span::Symbol;

pub fn get_receiver_ident_name(receiver: &Expr) -> Symbol {
    if let ExprKind::Path(QPath::Resolved(_, path)) = &receiver.kind {
        let name = path.segments.first().map(|segment| segment.ident.name);
        return name.unwrap_or(Symbol::intern(""));
    }
    Symbol::intern("")
}
