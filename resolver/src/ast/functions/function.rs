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

use crate::{FunctionError, FunctionType, ResolvedNode, Statement, SymbolTable, TypeError};
use leo_typed::Function as UnresolvedFunction;

use serde::{Deserialize, Serialize};

/// Stores function definition and function statements
/// This struct should be part of the type checked AST
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
    /// The name of the function definition
    pub type_: FunctionType,

    /// The function statements
    pub statements: Vec<Statement>,
}

impl ResolvedNode for Function {
    type Error = FunctionError;
    type UnresolvedNode = UnresolvedFunction;

    /// Type check a function inside a program AST
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        // Lookup function identifier in symbol table
        let identifier = unresolved.identifier;

        // Throw an error if the function does not exist
        let type_ = table
            .get_function(&identifier)
            .ok_or(FunctionError::TypeError(TypeError::undefined_function(identifier)))?
            .clone();

        // // Create function context
        // let mut child_table = SymbolTable::new(Some(Box::new(table.clone())));

        // Insert function input types into the symbol table
        for input in type_.inputs.clone() {
            let exists = input.insert(table);

            // Throw an error if two function inputs have been defined with the same name
            if exists.is_some() {
                return Err(FunctionError::duplicate_input(input.identifier().clone()));
            }
        }

        // Pass expected function output to resolved statements
        let output = type_.output.clone();
        let mut statements = vec![];

        // Resolve all function statements
        for (_i, statement) in unresolved.statements.into_iter().enumerate() {
            let statement = Statement::resolve(table, (output.clone(), statement))?;

            statements.push(statement);
        }

        Ok(Function { type_, statements })
    }
}
