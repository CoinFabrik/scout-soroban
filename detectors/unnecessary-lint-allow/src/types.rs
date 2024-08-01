use rustc_lint::{LateContext, LintContext};
use rustc_span::{FileName, FileNameDisplayPreference, Span};

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct AllowInfo {
    pub lint_name: String,
    pub span: SpanInfo,
    pub scope: Scope,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Scope {
    Crate,
    Enum,
    Function,
    Impl,
    Line,
    Struct,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct SpanInfo {
    pub file_name: String,
    pub from_line: usize,
    pub to_line: usize,
}

impl SpanInfo {
    pub fn from_span(cx: &LateContext, span: Span) -> Self {
        let source_map = cx.sess().source_map();
        let file = source_map.lookup_source_file(span.lo());
        let file_name = match &file.name {
            FileName::Real(name) => name
                .to_string_lossy(FileNameDisplayPreference::Remapped)
                .into_owned(),
            _ => String::from("<unknown>"),
        };

        let lo_loc = source_map.lookup_char_pos(span.lo());
        let hi_loc = source_map.lookup_char_pos(span.hi());

        SpanInfo {
            file_name,
            from_line: lo_loc.line,
            to_line: hi_loc.line,
        }
    }
}
