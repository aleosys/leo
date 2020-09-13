use std::{ffi::OsString, io};

#[derive(Debug, Error)]
pub enum PackageError {
    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("`{}` creating: {}", _0, _1)]
    Creating(&'static str, io::Error),

    #[error("Failed to initialize package {:?} ({:?})", _0, _1)]
    FailedToInitialize(String, OsString),

    #[error("`{}` metadata: {}", _0, _1)]
    Removing(&'static str, io::Error),
}

impl From<std::io::Error> for PackageError {
    fn from(error: std::io::Error) -> Self {
        PackageError::Crate("std::io", format!("{}", error))
    }
}

impl From<crate::errors::GitignoreError> for PackageError {
    fn from(error: crate::errors::GitignoreError) -> Self {
        PackageError::Crate("leo-package", format!("{}", error))
    }
}

impl From<crate::errors::InputFileError> for PackageError {
    fn from(error: crate::errors::InputFileError) -> Self {
        PackageError::Crate("leo-package", format!("{}", error))
    }
}

impl From<crate::errors::InputsDirectoryError> for PackageError {
    fn from(error: crate::errors::InputsDirectoryError) -> Self {
        PackageError::Crate("leo-package", format!("{}", error))
    }
}

impl From<crate::errors::ImportsDirectoryError> for PackageError {
    fn from(error: crate::errors::ImportsDirectoryError) -> Self {
        PackageError::Crate("leo-package", format!("{}", error))
    }
}

impl From<crate::errors::OutputsDirectoryError> for PackageError {
    fn from(error: crate::errors::OutputsDirectoryError) -> Self {
        PackageError::Crate("leo-package", format!("{}", error))
    }
}

impl From<crate::errors::READMEError> for PackageError {
    fn from(error: crate::errors::READMEError) -> Self {
        PackageError::Crate("leo-package", format!("{}", error))
    }
}

impl From<crate::errors::SourceDirectoryError> for PackageError {
    fn from(error: crate::errors::SourceDirectoryError) -> Self {
        PackageError::Crate("leo-package", format!("{}", error))
    }
}

impl From<crate::errors::StateFileError> for PackageError {
    fn from(error: crate::errors::StateFileError) -> Self {
        PackageError::Crate("leo-package", format!("{}", error))
    }
}

impl From<crate::errors::LibraryFileError> for PackageError {
    fn from(error: crate::errors::LibraryFileError) -> Self {
        PackageError::Crate("leo-package", format!("{}", error))
    }
}

impl From<crate::errors::ManifestError> for PackageError {
    fn from(error: crate::errors::ManifestError) -> Self {
        PackageError::Crate("leo-package", format!("{}", error))
    }
}

impl From<crate::errors::MainFileError> for PackageError {
    fn from(error: crate::errors::MainFileError) -> Self {
        PackageError::Crate("leo-package", format!("{}", error))
    }
}

impl From<zip::result::ZipError> for PackageError {
    fn from(error: zip::result::ZipError) -> Self {
        PackageError::Crate("zip", format!("{}", error))
    }
}
