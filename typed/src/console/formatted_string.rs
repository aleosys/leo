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

use crate::{FormattedContainer, FormattedParameter, Span};
use leo_ast::console::FormattedString as AstFormattedString;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormattedString {
    pub string: String,
    pub containers: Vec<FormattedContainer>,
    pub parameters: Vec<FormattedParameter>,
    pub span: Span,
}

impl<'ast> From<AstFormattedString<'ast>> for FormattedString {
    fn from(formatted: AstFormattedString<'ast>) -> Self {
        let string = formatted.string;
        let span = Span::from(formatted.span);
        let containers = formatted
            .containers
            .into_iter()
            .map(|container| FormattedContainer::from(container))
            .collect();
        let parameters = formatted
            .parameters
            .into_iter()
            .map(|parameter| FormattedParameter::from(parameter))
            .collect();

        Self {
            string,
            containers,
            parameters,
            span,
        }
    }
}

impl fmt::Display for FormattedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl fmt::Debug for FormattedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}
