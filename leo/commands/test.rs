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

use crate::{
    cli::*,
    cli_types::*,
    errors::{CLIError, TestError},
};
use leo_compiler::{compiler::Compiler, group::targets::edwards_bls12::EdwardsGroupType};
use leo_package::{
    inputs::*,
    outputs::{OutputsDirectory, OUTPUTS_DIRECTORY_NAME},
    root::Manifest,
    source::{MainFile, MAIN_FILE_NAME, SOURCE_DIRECTORY_NAME},
};

use snarkos_curves::edwards_bls12::Fq;

use clap::ArgMatches;
use std::{convert::TryFrom, env::current_dir};

#[derive(Debug)]
pub struct TestCommand;

impl CLI for TestCommand {
    type Options = ();
    type Output = ();

    const ABOUT: AboutType = "Compile and run all tests in the current package";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "test";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(_options: Self::Options) -> Result<Self::Output, CLIError> {
        let path = current_dir()?;

        // Get the package name
        let manifest = Manifest::try_from(&path)?;
        let package_name = manifest.get_package_name();

        // Sanitize the package path to the root directory
        let mut package_path = path.clone();
        if package_path.is_file() {
            package_path.pop();
        }

        // Verify the main file exists
        if !MainFile::exists_at(&package_path) {
            return Err(TestError::MainFileDoesNotExist(package_path.as_os_str().to_owned()).into());
        }

        // Construct the path to the main file in the source directory
        let mut main_file_path = package_path.clone();
        main_file_path.push(SOURCE_DIRECTORY_NAME);
        main_file_path.push(MAIN_FILE_NAME);

        // Construct the path to the output directory;
        let mut output_directory = package_path.clone();
        output_directory.push(OUTPUTS_DIRECTORY_NAME);

        // Create the output directory
        OutputsDirectory::create(&package_path)?;

        // Parse the current main program file
        let program = Compiler::<Fq, EdwardsGroupType>::parse_program_without_input(
            package_name.clone(),
            main_file_path.clone(),
            output_directory,
        )?;

        // Parse all inputs as input pairs
        let pairs = InputPairs::try_from(&package_path)?;

        // Run tests
        let temporary_program = program.clone();
        let output = temporary_program.compile_test_constraints(pairs)?;
        log::debug!("Compiled constraints - {:#?}", output);

        Ok(())
    }
}
