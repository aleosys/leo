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

use crate::{CircuitMember, Identifier};
use leo_ast::circuits::Circuit as AstCircuit;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Circuit {
    pub circuit_name: Identifier,
    pub members: Vec<CircuitMember>,
}

impl<'ast> From<AstCircuit<'ast>> for Circuit {
    fn from(circuit: AstCircuit<'ast>) -> Self {
        let circuit_name = Identifier::from(circuit.identifier);
        let members = circuit
            .members
            .into_iter()
            .map(|member| CircuitMember::from(member))
            .collect();

        Self { circuit_name, members }
    }
}

impl Circuit {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "circuit {} {{ \n", self.circuit_name)?;
        for field in self.members.iter() {
            write!(f, "    {}\n", field)?;
        }
        write!(f, "}}")
    }
}

// TODO (Collin): Uncomment when we no longer print out Program
// impl fmt::Display for Circuit {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.format(f)
//     }
// }

impl fmt::Debug for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
