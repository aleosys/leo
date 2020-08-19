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

//! Methods to enforce constraints on input boolean values in a resolved Leo program.

use crate::{errors::BooleanError, value::ConstrainedValue, GroupType};
use leo_typed::{InputValue, Span};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{alloc::AllocGadget, boolean::Boolean},
    },
};

pub(crate) fn new_bool_constant(string: String, span: Span) -> Result<Boolean, BooleanError> {
    let boolean = string
        .parse::<bool>()
        .map_err(|_| BooleanError::invalid_boolean(string, span))?;

    Ok(Boolean::constant(boolean))
}

pub(crate) fn allocate_bool<F: Field + PrimeField, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: String,
    option: Option<bool>,
    span: Span,
) -> Result<Boolean, BooleanError> {
    let boolean_name = format!("{}: bool", name);
    let boolean_name_unique = format!("`{}` {}:{}", boolean_name, span.line, span.start);

    Boolean::alloc(cs.ns(|| boolean_name_unique), || {
        option.ok_or(SynthesisError::AssignmentMissing)
    })
    .map_err(|_| BooleanError::missing_boolean(boolean_name, span))
}

pub(crate) fn bool_from_input<F: Field + PrimeField, G: GroupType<F>, CS: ConstraintSystem<F>>(
    cs: &mut CS,
    name: String,
    input_value: Option<InputValue>,
    span: Span,
) -> Result<ConstrainedValue<F, G>, BooleanError> {
    // Check that the input value is the correct type
    let option = match input_value {
        Some(input) => {
            if let InputValue::Boolean(bool) = input {
                Some(bool)
            } else {
                return Err(BooleanError::invalid_boolean(name, span));
            }
        }
        None => None,
    };

    let number = allocate_bool(cs, name, option, span)?;

    Ok(ConstrainedValue::Boolean(number))
}
