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

use crate::errors::SourceDirectoryError;

use std::{fs, path::PathBuf};

pub static SOURCE_DIRECTORY_NAME: &str = "src/";

static SOURCE_FILE_EXTENSION: &str = "leo";

pub struct SourceDirectory;

impl SourceDirectory {
    /// Creates a directory at the provided path with the default directory name.
    pub fn create(path: &PathBuf) -> Result<(), SourceDirectoryError> {
        let mut path = path.to_owned();
        if path.is_dir() && !path.ends_with(SOURCE_DIRECTORY_NAME) {
            path.push(PathBuf::from(SOURCE_DIRECTORY_NAME));
        }

        fs::create_dir_all(&path).map_err(SourceDirectoryError::Creating)
    }

    /// Returns a list of files in the source directory.
    pub fn files(path: &PathBuf) -> Result<Vec<PathBuf>, SourceDirectoryError> {
        let mut path = path.to_owned();
        path.push(PathBuf::from(SOURCE_DIRECTORY_NAME));
        let directory = fs::read_dir(&path).map_err(SourceDirectoryError::Reading)?;

        let mut file_paths = Vec::new();
        for file_entry in directory.into_iter() {
            let file_entry = file_entry.map_err(SourceDirectoryError::GettingFileEntry)?;
            let file_path = file_entry.path();

            // Verify that the entry is structured as a valid file
            let file_type = file_entry
                .file_type()
                .map_err(|error| SourceDirectoryError::GettingFileType(file_path.as_os_str().to_owned(), error))?;
            if !file_type.is_file() {
                return Err(SourceDirectoryError::InvalidFileType(
                    file_path.as_os_str().to_owned(),
                    file_type,
                ));
            }

            // Verify that the file has the default file extension
            let file_extension = file_path
                .extension()
                .ok_or_else(|| SourceDirectoryError::GettingFileExtension(file_path.as_os_str().to_owned()))?;
            if file_extension != SOURCE_FILE_EXTENSION {
                return Err(SourceDirectoryError::InvalidFileExtension(
                    file_path.as_os_str().to_owned(),
                    file_extension.to_owned(),
                ));
            }

            file_paths.push(file_path);
        }

        Ok(file_paths)
    }
}
