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

use crate::{FunctionInput, Identifier, Span, Statement, Type};
use leo_ast::functions::Function as AstFunction;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
    pub identifier: Identifier,
    pub input: Vec<FunctionInput>,
    pub returns: Option<Type>,
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl<'ast> From<AstFunction<'ast>> for Function {
    fn from(function: AstFunction<'ast>) -> Self {
        let function_name = Identifier::from(function.identifier);
        let parameters = function
            .parameters
            .into_iter()
            .map(|parameter| FunctionInput::from(parameter))
            .collect();
        let returns = function.returns.map(|type_| Type::from(type_));
        let statements = function
            .statements
            .into_iter()
            .map(|statement| Statement::from(statement))
            .collect();

        Function {
            identifier: function_name,
            input: parameters,
            returns,
            statements,
            span: Span::from(function.span),
        }
    }
}

impl Function {
    pub fn get_name(&self) -> String {
        self.identifier.name.clone()
    }

    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "function {}", self.identifier)?;
        let parameters = self
            .input
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(",");
        let returns = self.returns.as_ref().map(|type_| format!("{}", type_));
        let statements = self
            .statements
            .iter()
            .map(|s| format!("\t{}\n", s))
            .collect::<Vec<_>>()
            .join("");
        if returns.is_none() {
            write!(f, "({}) {{\n{}}}", parameters, statements,)
        } else {
            write!(f, "({}) -> {} {{\n{}}}", parameters, returns.unwrap(), statements,)
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
