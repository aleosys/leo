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

use leo_package::errors::ManifestError;

use std::{ffi::OsString, io};

#[derive(Debug, Error)]
pub enum InitError {
    #[error("root directory {:?} creating: {}", _0, _1)]
    CreatingRootDirectory(OsString, io::Error),

    #[error("directory {:?} does not exist", _0)]
    DirectoryDoesNotExist(OsString),

    #[error("{}", _0)]
    ManifestError(#[from] ManifestError),

    #[error("package at path {:?} already exists", _0)]
    PackageAlreadyExists(OsString),

    #[error("package name is missing - {:?}", _0)]
    ProjectNameInvalid(OsString),
}
