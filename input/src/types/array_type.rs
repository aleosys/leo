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

use crate::{ast::Rule, types::DataType, values::PositiveNumber};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_array))]
pub struct ArrayType<'ast> {
    pub _type: DataType,
    pub dimensions: Vec<PositiveNumber<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> ArrayType<'ast> {
    pub fn next_dimension(&mut self) -> Option<PositiveNumber<'ast>> {
        self.dimensions.pop()
    }
}

impl<'ast> std::fmt::Display for ArrayType<'ast> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self._type)?;
        for row in &self.dimensions {
            write!(f, "[{}]", row)?;
        }
        write!(f, "")
    }
}
