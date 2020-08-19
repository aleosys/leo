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

use crate::{errors::FunctionError, ConstrainedCircuitMember, ConstrainedProgram, ConstrainedValue, GroupType};

use leo_typed::{Identifier, InputValue, Parameter};
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};
use std::collections::HashMap;

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn allocate_input_section<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        identifier: Identifier,
        section: HashMap<Parameter, Option<InputValue>>,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        let mut members = vec![];

        // Allocate each section definition as a circuit member value

        for (parameter, option) in section.into_iter() {
            let member_name = parameter.variable.clone();
            let member_value = self.allocate_main_function_input(
                cs,
                parameter.type_,
                parameter.variable.name,
                option,
                parameter.span,
            )?;
            let member = ConstrainedCircuitMember(member_name, member_value);

            members.push(member)
        }

        // Return section as circuit expression

        Ok(ConstrainedValue::CircuitExpression(identifier, members))
    }
}
