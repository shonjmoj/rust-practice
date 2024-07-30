use std::{fmt, fmt::Display};

#[derive(Debug)]
pub struct ProgramError {
    message: String,
}

impl ProgramError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ProgramError {}
