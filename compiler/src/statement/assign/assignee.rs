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

//! Resolves assignees in a compiled Leo program.

use crate::{errors::StatementError, new_scope, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_typed::{Assignee, Span};

use snarkos_models::curves::{Field, PrimeField};

pub fn resolve_assignee(scope: String, assignee: Assignee) -> String {
    match assignee {
        Assignee::Identifier(name) => new_scope(scope, name.to_string()),
        Assignee::Array(array, _index) => resolve_assignee(scope, *array),
        Assignee::Tuple(tuple, _index) => resolve_assignee(scope, *tuple),
        Assignee::CircuitField(circuit_name, _member) => resolve_assignee(scope, *circuit_name),
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn get_mutable_assignee(
        &mut self,
        name: String,
        span: Span,
    ) -> Result<&mut ConstrainedValue<F, G>, StatementError> {
        // Check that assignee exists and is mutable
        Ok(match self.get_mut(&name) {
            Some(value) => match value {
                ConstrainedValue::Mutable(mutable_value) => mutable_value,
                _ => return Err(StatementError::immutable_assign(name, span)),
            },
            None => return Err(StatementError::undefined_variable(name, span)),
        })
    }
}
