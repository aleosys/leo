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

//! Compiles a Leo program from a file path.

use crate::{
    constraints::{generate_constraints, generate_test_constraints},
    errors::CompilerError,
    GroupType,
    OutputBytes,
    OutputFile,
};
use leo_ast::LeoAst;
use leo_imports::ImportParser;
use leo_input::LeoInputParser;
use leo_package::inputs::InputPairs;
use leo_state::verify_local_data_commitment;
use leo_typed::{Input, LeoTypedAst, MainInput, Program};

use snarkos_dpc::{base_dpc::instantiated::Components, SystemParameters};
use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::{ConstraintSynthesizer, ConstraintSystem},
};

use leo_resolver::LeoResolvedAst;
use sha2::{Digest, Sha256};
use std::{fs, marker::PhantomData, path::PathBuf};

#[derive(Clone)]
pub struct Compiler<F: Field + PrimeField, G: GroupType<F>> {
    package_name: String,
    main_file_path: PathBuf,
    output_directory: PathBuf,
    program: Program,
    program_input: Input,
    imported_programs: ImportParser,
    _engine: PhantomData<F>,
    _group: PhantomData<G>,
}

impl<F: Field + PrimeField, G: GroupType<F>> Compiler<F, G> {
    pub fn new(package_name: String, main_file_path: PathBuf, output_directory: PathBuf) -> Self {
        Self {
            package_name: package_name.clone(),
            main_file_path,
            output_directory,
            program: Program::new(package_name),
            program_input: Input::new(),
            imported_programs: ImportParser::new(),
            _engine: PhantomData,
            _group: PhantomData,
        }
    }

    /// Parse the input and state files.
    /// Stores a typed ast of all input variables to the program.
    pub fn parse_input(
        &mut self,
        input_string: &str,
        input_path: PathBuf,
        state_string: &str,
        state_path: PathBuf,
    ) -> Result<(), CompilerError> {
        let input_syntax_tree = LeoInputParser::parse_file(&input_string).map_err(|mut e| {
            e.set_path(input_path.clone());

            e
        })?;
        let state_syntax_tree = LeoInputParser::parse_file(&state_string).map_err(|mut e| {
            e.set_path(state_path.clone());

            e
        })?;

        self.program_input.parse_input(input_syntax_tree).map_err(|mut e| {
            e.set_path(input_path);

            e
        })?;
        self.program_input.parse_state(state_syntax_tree).map_err(|mut e| {
            e.set_path(state_path);

            e
        })?;

        Ok(())
    }

    /// Parses program files.
    /// Returns a compiler struct that stores the typed program abstract syntax trees (ast).
    pub fn parse_program_without_input(
        package_name: String,
        main_file_path: PathBuf,
        output_directory: PathBuf,
    ) -> Result<Self, CompilerError> {
        let mut compiler = Self::new(package_name, main_file_path, output_directory);

        compiler.parse_program()?;

        Ok(compiler)
    }

    /// Parses input, state, and program files.
    /// Returns a compiler struct that stores the typed input and typed program abstract syntax trees (ast).
    pub fn parse_program_with_input(
        package_name: String,
        main_file_path: PathBuf,
        output_directory: PathBuf,
        input_string: &str,
        input_path: PathBuf,
        state_string: &str,
        state_path: PathBuf,
    ) -> Result<Self, CompilerError> {
        let mut compiler = Self::new(package_name, main_file_path, output_directory);

        compiler.parse_input(input_string, input_path, state_string, state_path)?;

        compiler.parse_program()?;

        Ok(compiler)
    }

    /// Parses the Leo program file, constructs a syntax tree, and generates a program.
    pub(crate) fn parse_program(&mut self) -> Result<(), CompilerError> {
        // Use the parser to construct the abstract syntax tree.
        let program_string = LeoAst::load_file(&self.main_file_path)?;

        self.parse_program_from_string(&program_string)
    }

    /// Parses the Leo program string, constructs a syntax tree, and generates a program.
    /// Used for testing only.
    #[deprecated(note = "Please use the 'parse_program' method instead.")]
    pub fn parse_program_from_string(&mut self, program_string: &str) -> Result<(), CompilerError> {
        // Use the given bytes to construct the abstract syntax tree.
        let ast = LeoAst::new(&self.main_file_path, &program_string).map_err(|mut e| {
            e.set_path(self.main_file_path.clone());

            e
        })?;

        // Derive the package name.
        let package_name = self.package_name.clone();

        // Use the typed parser to construct the typed syntax tree.
        let typed_tree = LeoTypedAst::new(&package_name, &ast);

        let _resolved_tree = LeoResolvedAst::new(typed_tree.clone());

        self.program = typed_tree.into_repr();
        self.imported_programs = ImportParser::parse(&self.program)?;

        tracing::debug!("Program parsing complete\n{:#?}", self.program);

        Ok(())
    }

    /// Manually sets main function input
    pub fn set_main_input(&mut self, input: MainInput) {
        self.program_input.set_main_input(input);
    }

    /// Verifies the input to the program
    pub fn verify_local_data_commitment(
        &self,
        system_parameters: &SystemParameters<Components>,
    ) -> Result<bool, CompilerError> {
        let result = verify_local_data_commitment(system_parameters, &self.program_input)?;

        Ok(result)
    }

    pub fn checksum(&self) -> Result<String, CompilerError> {
        // Read in the main file as string
        let unparsed_file = fs::read_to_string(&self.main_file_path)
            .map_err(|_| CompilerError::FileReadError(self.main_file_path.clone()))?;

        // Hash the file contents
        let mut hasher = Sha256::new();
        hasher.update(unparsed_file.as_bytes());
        let hash = hasher.finalize();

        Ok(hex::encode(hash))
    }

    /// Synthesizes the circuit without program input to verify correctness.
    pub fn compile_constraints<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<OutputBytes, CompilerError> {
        let path = self.main_file_path;

        generate_constraints::<F, G, CS>(cs, self.program, self.program_input, &self.imported_programs).map_err(
            |mut error| {
                error.set_path(path);

                error
            },
        )
    }

    /// Synthesizes the circuit for test functions with program input.
    pub fn compile_test_constraints(self, input_pairs: InputPairs) -> Result<(u32, u32), CompilerError> {
        generate_test_constraints::<F, G>(
            self.program,
            input_pairs,
            &self.imported_programs,
            &self.main_file_path,
            &self.output_directory,
        )
    }

    /// Calls the internal generate_constraints method with arguments
    pub fn generate_constraints_helper<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<OutputBytes, CompilerError> {
        let path = self.main_file_path;
        generate_constraints::<_, G, _>(cs, self.program, self.program_input, &self.imported_programs).map_err(
            |mut error| {
                error.set_path(path);
                error
            },
        )
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CompilerError> {
        Ok(bincode::serialize(&self.program)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CompilerError> {
        let program: Program = bincode::deserialize(bytes)?;
        let program_input = Input::new();

        Ok(Self {
            package_name: program.name.clone(),
            main_file_path: PathBuf::new(),
            output_directory: PathBuf::new(),
            program,
            program_input,
            imported_programs: ImportParser::new(),
            _engine: PhantomData,
            _group: PhantomData,
        })
    }
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstraintSynthesizer<F> for Compiler<F, G> {
    /// Synthesizes the circuit with program input.
    fn generate_constraints<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let output_directory = self.output_directory.clone();
        let package_name = self.package_name.clone();
        let result = self.generate_constraints_helper(cs).map_err(|e| {
            tracing::error!("{}", e);
            SynthesisError::Unsatisfiable
        })?;

        // Write results to file
        let output_file = OutputFile::new(&package_name);
        output_file.write(&output_directory, result.bytes()).unwrap();

        Ok(())
    }
}
