use std::fmt::Display;

#[derive(Debug)]
pub enum WcStackError {
    BadChangeId(String),
    IoError(std::io::Error),
    NoSuchChangeId(String),
    AmbiguousPrefix(String),
}

impl Display for WcStackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadChangeId(s) => write!(f, "Invalid change ID: {}", s),
            Self::IoError(io_error) => write!(f, "{}", io_error),
            Self::NoSuchChangeId(s) => write!(f, "Nothing in stack matches the prefix {}", s),
            Self::AmbiguousPrefix(s) => write!(f, "Multiple change ids match the prefix {}", s),
        }
    }
}

impl From<std::io::Error> for WcStackError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
