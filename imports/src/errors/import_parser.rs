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
use leo_ast::ParserError;
use leo_typed::{Error as FormattedError, Identifier, Span};

use std::{io, path::PathBuf};

#[derive(Debug, Error)]
pub enum ImportParserError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    ParserError(#[from] ParserError),
}

impl ImportParserError {
    fn new_from_span(message: String, span: Span) -> Self {
        ImportParserError::Error(FormattedError::new_from_span(message, span))
    }

    fn new_from_span_with_path(message: String, span: Span, path: PathBuf) -> Self {
        ImportParserError::Error(FormattedError::new_from_span_with_path(message, span, path))
    }

    pub fn conflicting_imports(identifier: Identifier) -> Self {
        let message = format!("conflicting imports found for `{}`", identifier.name);

        Self::new_from_span(message, identifier.span)
    }

    pub fn convert_os_string(span: Span) -> Self {
        let message = format!("failed to convert file string name, maybe an illegal character?");

        Self::new_from_span(message, span)
    }

    pub fn current_directory_error(error: io::Error) -> Self {
        let span = Span {
            text: "".to_string(),
            line: 0,
            start: 0,
            end: 0,
        };
        let message = format!("compilation failed trying to find current directory - {:?}", error);

        Self::new_from_span(message, span)
    }

    pub fn directory_error(error: io::Error, span: Span, path: PathBuf) -> Self {
        let message = format!("compilation failed due to directory error - {:?}", error);

        Self::new_from_span_with_path(message, span, path)
    }

    pub fn star(path: PathBuf, span: Span) -> Self {
        let message = format!("cannot import `*` from path `{:?}`", path);

        Self::new_from_span(message, span)
    }

    pub fn expected_lib_file(entry: String, span: Span) -> Self {
        let message = format!(
            "expected library file`{}` when looking for symbol `{}`",
            entry, span.text
        );

        Self::new_from_span(message, span)
    }

    pub fn unknown_package(identifier: Identifier) -> Self {
        let message = format!(
            "cannot find imported package `{}` in source files or import directory",
            identifier.name
        );

        Self::new_from_span(message, identifier.span)
    }
}
