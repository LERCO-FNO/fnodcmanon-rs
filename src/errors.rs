use std::fmt;

#[derive(Debug)]
pub struct NoDicomDirectoriesFound {
    message: String,
}

impl NoDicomDirectoriesFound {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for NoDicomDirectoriesFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for NoDicomDirectoriesFound {}
