// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{ast::Rule, types::*};

use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::type_))]
pub enum Type<'ast> {
    Basic(DataType),
    Array(ArrayType<'ast>),
    Tuple(TupleType<'ast>),
    Circuit(CircuitType<'ast>),
    SelfType(SelfType),
}

impl<'ast> fmt::Display for Type<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Basic(ref _type) => write!(f, "basic"),
            Type::Array(ref _type) => write!(f, "array"),
            Type::Tuple(ref _type) => write!(f, "tuple"),
            Type::Circuit(ref _type) => write!(f, "struct"),
            Type::SelfType(ref _type) => write!(f, "Self"),
        }
    }
}
