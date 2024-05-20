use std::fmt::Display;

#[derive(Debug)]
pub enum WcStackError {
    BadChangeId(String),
    IoError(std::io::Error),
}

impl Display for WcStackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WcStackError::BadChangeId(s) => write!(f, "Invalid change ID: {}", s),
            Self::IoError(io_error) => write!(f, "{}", io_error),
        }
    }
}

impl From<std::io::Error> for WcStackError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
