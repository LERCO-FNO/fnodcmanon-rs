use std::fmt;

#[derive(Debug)]
pub enum UIDRootError {
    InvalidCharacter(String),
    ExtraPeriod(String),
}

impl std::error::Error for UIDRootError {}

impl fmt::Display for UIDRootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIDRootError::InvalidCharacter(uid) => write!(
                f,
                "invalid character in UID root '{}', only period separated digits allowed",
                uid
            ),
            UIDRootError::ExtraPeriod(uid) => write!(f, "extra period in UID root '{}'", uid),
        }
    }
}
