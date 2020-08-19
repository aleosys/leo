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

//! Methods to enforce constraints on input field values in a compiled Leo program.

use crate::{errors::FieldError, value::ConstrainedValue, FieldType, GroupType};
use leo_typed::{InputValue, Span};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::alloc::AllocGadget},
};

pub(crate) fn allocate_field<F: Field + PrimeField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: String,
    option: Option<String>,
    span: Span,
) -> Result<FieldType<F>, FieldError> {
    let field_name = format!("{}: field", name);
    let field_name_unique = format!("`{}` {}:{}", field_name, span.line, span.start);

    FieldType::alloc(cs.ns(|| field_name_unique), || {
        option.ok_or(SynthesisError::AssignmentMissing)
    })
    .map_err(|_| FieldError::missing_field(field_name, span))
}

pub(crate) fn field_from_input<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: String,
    input_value: Option<InputValue>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, FieldError> {
    // Check that the parameter value is the correct type
    let option = match input_value {
        Some(input) => {
            if let InputValue::Field(string) = input {
                Some(string)
            } else {
                return Err(FieldError::invalid_field(input.to_string(), span));
            }
        }
        None => None,
    };

    let field = allocate_field(cs, name, option, span)?;

    Ok(ConstrainedValue::Field(field))
}
