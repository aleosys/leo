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

//! An in memory store to keep track of defined names when constraining a Leo program.

use crate::{value::ConstrainedValue, GroupType};

use snarkos_models::curves::{Field, PrimeField};

use std::collections::HashMap;

#[derive(Clone)]
pub struct ConstrainedProgram<F: Field + PrimeField, G: GroupType<F>> {
    pub identifiers: HashMap<String, ConstrainedValue<F, G>>,
}

pub fn new_scope(outer: String, inner: String) -> String {
    format!("{}_{}", outer, inner)
}

pub fn is_in_scope(current_scope: &String, desired_scope: &String) -> bool {
    current_scope.ends_with(desired_scope)
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn new() -> Self {
        Self {
            identifiers: HashMap::new(),
        }
    }

    pub(crate) fn store(&mut self, name: String, value: ConstrainedValue<F, G>) {
        self.identifiers.insert(name, value);
    }

    pub(crate) fn get(&self, name: &String) -> Option<&ConstrainedValue<F, G>> {
        self.identifiers.get(name)
    }

    pub(crate) fn get_mut(&mut self, name: &String) -> Option<&mut ConstrainedValue<F, G>> {
        self.identifiers.get_mut(name)
    }
}
