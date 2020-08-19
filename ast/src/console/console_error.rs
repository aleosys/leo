use crate::{ast::Rule, console::FormattedString, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::console_error))]
pub struct ConsoleError<'ast> {
    pub string: FormattedString<'ast>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for ConsoleError<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error({})", self.string)
    }
}
