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

//! Allocates an array as a main function input parameter in a compiled Leo program.

use crate::{
    errors::FunctionError,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};

use leo_typed::{InputValue, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn allocate_array<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        name: String,
        array_type: Type,
        array_dimensions: Vec<usize>,
        input_value: Option<InputValue>,
        span: Span,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        let expected_length = array_dimensions[0];
        let mut array_value = vec![];

        match input_value {
            Some(InputValue::Array(arr)) => {
                // Allocate each value in the current row
                for (i, value) in arr.into_iter().enumerate() {
                    let value_name = new_scope(name.clone(), i.to_string());
                    let value_type = array_type.outer_dimension(&array_dimensions);

                    array_value.push(self.allocate_main_function_input(
                        cs,
                        value_type,
                        value_name,
                        Some(value),
                        span.clone(),
                    )?)
                }
            }
            None => {
                // Allocate all row values as none
                for i in 0..expected_length {
                    let value_name = new_scope(name.clone(), i.to_string());
                    let value_type = array_type.outer_dimension(&array_dimensions);

                    array_value.push(self.allocate_main_function_input(
                        cs,
                        value_type,
                        value_name,
                        None,
                        span.clone(),
                    )?);
                }
            }
            _ => return Err(FunctionError::invalid_array(input_value.unwrap().to_string(), span)),
        }

        Ok(ConstrainedValue::Array(array_value))
    }
}
