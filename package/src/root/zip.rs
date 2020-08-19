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

//! The program package zip file.

use crate::{
    errors::ZipFileError,
    imports::IMPORTS_DIRECTORY_NAME,
    inputs::{INPUTS_DIRECTORY_NAME, INPUT_FILE_EXTENSION},
    outputs::{
        CHECKSUM_FILE_EXTENSION,
        CIRCUIT_FILE_EXTENSION,
        OUTPUTS_DIRECTORY_NAME,
        PROOF_FILE_EXTENSION,
        PROVING_KEY_FILE_EXTENSION,
        VERIFICATION_KEY_FILE_EXTENSION,
    },
};

use serde::Deserialize;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};
use walkdir::WalkDir;
use zip::write::{FileOptions, ZipWriter};

pub static ZIP_FILE_EXTENSION: &str = ".zip";

#[derive(Deserialize)]
pub struct ZipFile {
    pub package_name: String,
}

impl ZipFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(&self, path: &PathBuf) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    pub fn get_file_path(&self, current_dir: &PathBuf) -> PathBuf {
        self.setup_file_path(current_dir)
    }

    // /// Reads the program bytes from the given file path if it exists.
    // pub fn read_from(&self, path: &PathBuf) -> Result<Vec<u8>, ZipFileError> {
    //     let path = self.setup_file_path(path);
    //
    //     Ok(fs::read(&path).map_err(|_| ZipFileError::FileReadError(path.clone()))?)
    // }

    /// Writes the current package contents to a zip file.
    pub fn write(&self, src_dir: &PathBuf) -> Result<(), ZipFileError> {
        // Build walkdir iterator from current package
        let walkdir = WalkDir::new(src_dir.clone());

        // Create zip file
        let path = self.setup_file_path(src_dir);

        let file = &mut File::create(&path)?;
        let mut zip = ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        // Walk through files in directory and write desired ones to the zip file
        let mut buffer = Vec::new();
        for entry in walkdir.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = path.strip_prefix(src_dir.as_path()).unwrap();

            // filter excluded paths
            if is_excluded(name) {
                continue;
            }

            // write file or directory
            if path.is_file() {
                log::info!("Adding file {:?} as {:?}", path, name);
                zip.start_file_from_path(name, options)?;
                let mut f = File::open(path)?;

                f.read_to_end(&mut buffer)?;
                zip.write_all(&*buffer)?;
                buffer.clear();
            } else if name.as_os_str().len() != 0 {
                // Only if not root Avoids path spec / warning
                // and mapname conversion failed error on unzip
                log::info!("Adding directory {:?} as {:?}", path, name);
                zip.add_directory_from_path(name, options)?;
            }
        }

        zip.finish()?;

        log::info!("Package zip file created successfully {:?}", path);

        Ok(())
    }

    /// Removes the zip file at the given path if it exists. Returns `true` on success,
    /// `false` if the file doesn't exist, and `Error` if the file system fails during operation.
    pub fn remove(&self, path: &PathBuf) -> Result<bool, ZipFileError> {
        let path = self.setup_file_path(path);
        if !path.exists() {
            return Ok(false);
        }

        fs::remove_file(&path).map_err(|_| ZipFileError::FileRemovalError(path.clone()))?;
        Ok(true)
    }

    fn setup_file_path(&self, path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(OUTPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!("{}{}", self.package_name, ZIP_FILE_EXTENSION)));
        }
        path
    }
}

fn is_excluded(path: &Path) -> bool {
    log::debug!("Checking if {:?} is excluded", path);

    // excluded directories: `input`, `output`, `imports`
    if path.ends_with(INPUTS_DIRECTORY_NAME.trim_end_matches("/"))
        | path.ends_with(OUTPUTS_DIRECTORY_NAME.trim_end_matches("/"))
        | path.ends_with(IMPORTS_DIRECTORY_NAME.trim_end_matches("/"))
    {
        return true;
    }

    // excluded extensions: `.in`, `.bytes`, `lpk`, `lvk`, `.proof`, `.sum`, `.zip`, `.bytes`
    path.extension()
        .map(|ext| {
            ext.eq(INPUT_FILE_EXTENSION.trim_start_matches("."))
                | ext.eq(ZIP_FILE_EXTENSION.trim_start_matches("."))
                | ext.eq(PROVING_KEY_FILE_EXTENSION.trim_start_matches("."))
                | ext.eq(VERIFICATION_KEY_FILE_EXTENSION.trim_start_matches("."))
                | ext.eq(PROOF_FILE_EXTENSION.trim_start_matches("."))
                | ext.eq(CHECKSUM_FILE_EXTENSION.trim_start_matches("."))
                | ext.eq(ZIP_FILE_EXTENSION.trim_start_matches("."))
                | ext.eq(CIRCUIT_FILE_EXTENSION.trim_start_matches("."))
        })
        .unwrap_or(false)
}
