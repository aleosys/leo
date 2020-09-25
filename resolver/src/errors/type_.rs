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

use leo_typed::{Error as FormattedError, Identifier, Span};

use std::path::PathBuf;

/// Errors encountered when resolving types
#[derive(Debug, Error)]
pub enum TypeError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl TypeError {
    /// Set the filepath for the error stacktrace
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            TypeError::Error(error) => error.set_path(path),
        }
    }

    /// Return a new formatted error with a given message and span information
    fn new_from_span(message: String, span: Span) -> Self {
        TypeError::Error(FormattedError::new_from_span(message, span))
    }

    /// Found an unknown circuit name
    pub fn undefined_circuit(identifier: Identifier) -> Self {
        let message = format!(
            "Type circuit `{}` must be defined before it is used in an expression",
            identifier.name
        );

        Self::new_from_span(message, identifier.span)
    }

    /// The `Self` keyword was used outside of a circuit
    pub fn self_not_available(span: Span) -> Self {
        let message = format!("Type `Self` is only available in circuit definitions and functions");

        Self::new_from_span(message, span)
    }
}
