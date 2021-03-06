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

//! Enforces that one return value is produced in a compiled Leo program.

use crate::{errors::StatementError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};

use leo_typed::Span;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{boolean::Boolean, select::CondSelectGadget},
    },
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// iterates through a vector of results and selects one based off of indicators
    pub fn conditionally_select_result<CS: ConstraintSystem<F>>(
        cs: &mut CS,
        return_value: &mut ConstrainedValue<F, G>,
        results: Vec<(Option<Boolean>, ConstrainedValue<F, G>)>,
        span: Span,
    ) -> Result<(), StatementError> {
        // if there are no results, continue
        if results.len() == 0 {
            return Ok(());
        }

        // If all indicators are none, then there are no branch conditions in the function.
        // We simply return the last result.

        if let None = results.iter().find(|(indicator, _res)| indicator.is_some()) {
            let result = &results[results.len() - 1].1;

            *return_value = result.clone();

            return Ok(());
        }

        // If there are branches in the function we need to use the `ConditionalSelectGadget` to parse through and select the correct one.
        // This can be thought of as de-multiplexing all previous wires that may have returned results into one.
        for (i, (indicator, result)) in results.into_iter().enumerate() {
            // Set the first value as the starting point
            if i == 0 {
                *return_value = result.clone();
            }

            let condition = indicator.unwrap_or(Boolean::Constant(true));
            let name_unique = format!("select {} {}:{}", result, span.line, span.start);
            let selected_value =
                ConstrainedValue::conditionally_select(cs.ns(|| name_unique), &condition, &result, return_value)
                    .map_err(|_| {
                        StatementError::select_fail(result.to_string(), return_value.to_string(), span.clone())
                    })?;

            *return_value = selected_value;
        }

        Ok(())
    }
}
