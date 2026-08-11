use std::{error, fmt, io};

use dicom_core::value::ConvertValueError;
use dicom_object::{AccessError, ReadError, WriteError};

#[derive(Debug)]
pub enum UIDRootError {
    InvalidCharacter(String),
    ExtraPeriod(String),
    LeadingZero(String),
}

impl error::Error for UIDRootError {}

impl fmt::Display for UIDRootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIDRootError::InvalidCharacter(uid) => write!(
                f,
                "invalid character in UID root '{}', only period separated digits allowed",
                uid
            ),
            UIDRootError::ExtraPeriod(uid) => write!(f, "extra period in UID root '{}'", uid),
            UIDRootError::LeadingZero(uid) => write!(f, "leading zero in UID root `{}`", uid),
        }
    }
}

#[derive(Debug)]
pub enum AnonymizerError {
    DirectoryError(io::Error),
    OpenFile(ReadError),
    WriteFile(Box<WriteError>),
    TagAccess(AccessError),
    ConvertValue(ConvertValueError),
}

impl fmt::Display for AnonymizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AnonymizerError::DirectoryError(e) => write!(f, "{e}"),
            AnonymizerError::OpenFile(e) => write!(f, "{e}"),
            AnonymizerError::TagAccess(e) => write!(f, "{e}"),
            AnonymizerError::ConvertValue(e) => write!(f, "{e}"),
            AnonymizerError::WriteFile(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for AnonymizerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AnonymizerError::DirectoryError(e) => Some(e),
            AnonymizerError::OpenFile(e) => Some(e),
            AnonymizerError::TagAccess(e) => Some(e),
            AnonymizerError::ConvertValue(e) => Some(e),
            AnonymizerError::WriteFile(e) => Some(e),
        }
    }
}

impl From<io::Error> for AnonymizerError {
    fn from(e: io::Error) -> Self {
        Self::DirectoryError(e)
    }
}

impl From<ReadError> for AnonymizerError {
    fn from(e: ReadError) -> Self {
        Self::OpenFile(e)
    }
}

impl From<WriteError> for AnonymizerError {
    fn from(e: WriteError) -> Self {
        Self::WriteFile(Box::new(e))
    }
}

impl From<AccessError> for AnonymizerError {
    fn from(e: AccessError) -> Self {
        Self::TagAccess(e)
    }
}

impl From<ConvertValueError> for AnonymizerError {
    fn from(e: ConvertValueError) -> Self {
        Self::ConvertValue(e)
    }
}
