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

use crate::{errors::ImportError, imported_symbols::ImportedSymbols, ConstrainedProgram, GroupType};
use leo_imports::ImportParser;
use leo_typed::Import;

use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub(crate) fn store_import(
        &mut self,
        scope: String,
        import: &Import,
        imported_programs: &ImportParser,
    ) -> Result<(), ImportError> {
        // Fetch core dependencies
        let core_dependency = imported_programs
            .core_packages()
            .iter()
            .find(|package| import.package.eq(package));

        if let Some(package) = core_dependency {
            self.store_core_package(scope.clone(), package.clone())?;

            return Ok(());
        }

        // Fetch dependencies for the current import
        let imported_symbols = ImportedSymbols::from(import);

        for (package, symbol) in imported_symbols.symbols {
            // Find imported program
            let program = imported_programs
                .get_import(&package)
                .ok_or(ImportError::unknown_package(import.package.name.clone()))?;

            // Parse imported program
            self.store_definitions(program.clone(), imported_programs)?;

            // Store the imported symbol
            self.store_symbol(scope.clone(), package, &symbol, program)?;
        }

        Ok(())
    }
}
