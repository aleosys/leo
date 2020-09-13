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

//! Enforces constraints on a function in a compiled Leo program.

use crate::{
    errors::FunctionError,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    GroupType,
};

use leo_typed::{Expression, Function, InputVariable, Span, Type};

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

pub fn check_arguments_length(expected: usize, actual: usize, span: Span) -> Result<(), FunctionError> {
    // Make sure we are given the correct number of arguments
    if expected != actual {
        Err(FunctionError::arguments_length(expected, actual, span))
    } else {
        Ok(())
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn enforce_function<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: String,
        caller_scope: String,
        function: Function,
        input: Vec<Expression>,
        declared_circuit_reference: String,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        let function_name = new_scope(scope.clone(), function.get_name());

        // Make sure we are given the correct number of input variables
        check_arguments_length(function.input.len(), input.len(), function.span.clone())?;

        // Store input values as new variables in resolved program
        for (input_model, input_expression) in function.input.clone().iter().zip(input.into_iter()) {
            let (name, value) = match input_model {
                InputVariable::InputKeyword(identifier) => {
                    let input_value = self.enforce_function_input(
                        cs,
                        scope.clone(),
                        caller_scope.clone(),
                        function_name.clone(),
                        None,
                        input_expression,
                    )?;

                    (identifier.name.clone(), input_value)
                }
                InputVariable::FunctionInput(input_model) => {
                    // First evaluate input expression
                    let mut input_value = self.enforce_function_input(
                        cs,
                        scope.clone(),
                        caller_scope.clone(),
                        function_name.clone(),
                        Some(input_model.type_.clone()),
                        input_expression,
                    )?;

                    if input_model.mutable {
                        input_value = ConstrainedValue::Mutable(Box::new(input_value))
                    }

                    (input_model.identifier.name.clone(), input_value)
                }
            };

            // Store input as variable with {function_name}_{input_name}
            let input_program_identifier = new_scope(function_name.clone(), name);
            self.store(input_program_identifier, value);
        }

        // Evaluate every statement in the function and save all potential results
        let mut results = vec![];

        for statement in function.statements.iter() {
            let mut result = self.enforce_statement(
                cs,
                scope.clone(),
                function_name.clone(),
                None,
                statement.clone(),
                function.returns.clone(),
                declared_circuit_reference.clone(),
            )?;

            results.append(&mut result);
        }

        // Conditionally select a result based on returned indicators
        let mut return_values = ConstrainedValue::Tuple(vec![]);

        Self::conditionally_select_result(cs, &mut return_values, results, function.span.clone())?;

        if let ConstrainedValue::Tuple(ref returns) = return_values {
            let return_types = match function.returns {
                Some(Type::Tuple(types)) => types.len(),
                Some(_) => 1usize,
                None => 0usize,
            };

            if return_types != returns.len() {
                return Err(FunctionError::return_arguments_length(
                    return_types,
                    returns.len(),
                    function.span.clone(),
                ));
            }
        }

        Ok(return_values)
    }
}
