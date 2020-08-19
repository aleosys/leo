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

use crate::{Identifier, Span, Type};
use leo_input::parameters::Parameter as AstParameter;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Parameter {
    pub variable: Identifier,
    pub type_: Type,
    pub span: Span,
}

impl<'ast> From<AstParameter<'ast>> for Parameter {
    fn from(parameter: AstParameter<'ast>) -> Self {
        Self {
            variable: Identifier::from(parameter.variable),
            type_: Type::from(parameter.type_),
            span: Span::from(parameter.span),
        }
    }
}
