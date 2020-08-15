use crate::{ast::Rule, types::*};

use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_))]
pub enum Type<'ast> {
    Basic(DataType),
    Array(ArrayType<'ast>),
    Tuple(TupleType<'ast>),
}

impl<'ast> fmt::Display for Type<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Basic(ref basic) => write!(f, "{}", basic),
            Type::Array(ref array) => write!(f, "{}", array),
            Type::Tuple(ref tuple) => write!(f, "{}", tuple),
        }
    }
}
