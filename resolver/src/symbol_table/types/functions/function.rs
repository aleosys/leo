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

use crate::{
    types::functions::{FunctionInputType, FunctionOutputType},
    ResolvedNode,
    SymbolTable,
    TypeError,
};
use leo_typed::{Function, Identifier};

use serde::{Deserialize, Serialize};

/// Stores function definition details
/// This type should be added to the function symbol table for a type checked program
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionType {
    /// The name of the function definition
    pub identifier: Identifier,

    /// The function inputs
    pub inputs: Vec<FunctionInputType>,

    /// The function output
    pub output: FunctionOutputType,
}

impl ResolvedNode for FunctionType {
    type Error = TypeError;
    type UnresolvedNode = Function;

    /// Type check a function definition
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        let mut inputs_resolved = vec![];

        // Type check function inputs
        for input in unresolved.input {
            let input = FunctionInputType::resolve(table, input)?;
            inputs_resolved.push(input);
        }

        // Type check function output
        let output = FunctionOutputType::resolve(table, (unresolved.returns, unresolved.span))?;

        Ok(FunctionType {
            identifier: unresolved.identifier,
            inputs: inputs_resolved,
            output,
        })
    }
}

impl FunctionType {
    /// Resolve a function definition and insert it into the given symbol table
    pub fn insert_definition(table: &mut SymbolTable, unresolved_function: Function) -> Result<(), TypeError> {
        let function_identifier = unresolved_function.identifier.clone();
        let function = Self::resolve(table, unresolved_function)?;

        table.insert_function(function_identifier, function);

        Ok(())
    }

    /// Resolve a circuit function definition and return it {
    pub fn from_circuit(
        table: &mut SymbolTable,
        circuit_name: Identifier,
        unresolved_function: Function,
    ) -> Result<Self, TypeError> {
        let function_identifier = unresolved_function.identifier;
        let mut inputs = vec![];

        // Type check function inputs
        for unresolved_input in unresolved_function.input {
            let input = FunctionInputType::from_circuit(table, unresolved_input, circuit_name.clone())?;
            inputs.push(input);
        }

        // Type check function output
        let output = FunctionOutputType::from_circuit(
            table,
            circuit_name.clone(),
            unresolved_function.returns,
            unresolved_function.span,
        )?;

        Ok(FunctionType {
            identifier: function_identifier.clone(),
            inputs,
            output,
        })
    }
}
