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

//! Evaluates a formatted string in a compiled Leo program.

use crate::{errors::ConsoleError, program::ConstrainedProgram, GroupType};
use leo_typed::FormattedString;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn format<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        file_scope: String,
        function_scope: String,
        formatted: FormattedString,
    ) -> Result<String, ConsoleError> {
        // Check that containers and parameters match
        if formatted.containers.len() != formatted.parameters.len() {
            return Err(ConsoleError::length(
                formatted.containers.len(),
                formatted.parameters.len(),
                formatted.span.clone(),
            ));
        }

        // Trim starting double quote `"`
        let mut string = formatted.string.as_str();
        string = string.trim_start_matches("\"");

        // Trim everything after the ending double quote `"`
        let parts: Vec<&str> = string.split("\"").collect();
        string = parts[0];

        // Insert the parameter for each container `{}`
        let mut result = string.to_string();

        for parameter in formatted.parameters.into_iter() {
            let parameter_value = self.enforce_expression(
                cs,
                file_scope.clone(),
                function_scope.clone(),
                None,
                parameter.expression,
            )?;

            result = result.replacen("{}", &parameter_value.to_string(), 1);
        }

        Ok(result)
    }
}
