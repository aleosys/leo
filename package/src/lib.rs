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

#[macro_use]
extern crate thiserror;

pub mod errors;
pub use errors::*;

pub mod imports;
pub mod inputs;
pub mod outputs;
pub mod package;
pub mod root;
pub mod source;

use std::path::PathBuf;

pub struct LeoPackage;

impl LeoPackage {
    /// Initializes a Leo package at the given path.
    pub fn initialize(package_name: &str, is_lib: bool, path: &PathBuf) -> Result<(), PackageError> {
        package::Package::initialize(package_name, is_lib, path)
    }

    /// Removes an imported Leo package
    pub fn remove_imported_package(package_name: &str, path: &PathBuf) -> Result<(), PackageError> {
        package::Package::remove_imported_package(package_name, path)
    }

    /// Installs a package as an import
    pub fn install_package(package_name: &str, package_bytes: Vec<u8>, path: &PathBuf) -> Result<(), PackageError> {
        package::Package::install_package(package_name, package_bytes, path)
    }
}
