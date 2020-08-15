use crate::{access::*, ast::Rule};

use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::access))]
pub enum Access<'ast> {
    Array(ArrayAccess<'ast>),
    Tuple(TupleAccess<'ast>),
    Call(CallAccess<'ast>),
    Object(MemberAccess<'ast>),
    StaticObject(StaticMemberAccess<'ast>),
}
