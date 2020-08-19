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

use crate::common::{Identifier, Span};
use leo_ast::common::VariableName as AstVariableName;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableName {
    pub mutable: bool,
    pub identifier: Identifier,
    pub span: Span,
}

impl<'ast> From<AstVariableName<'ast>> for VariableName {
    fn from(name: AstVariableName<'ast>) -> Self {
        Self {
            mutable: name.mutable.is_some(),
            identifier: Identifier::from(name.identifier),
            span: Span::from(name.span),
        }
    }
}

impl fmt::Display for VariableName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.mutable {
            write!(f, "mut ")?;
        }

        write!(f, "{}", self.identifier)
    }
}
