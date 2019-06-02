use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    Error(String),
    IO(std::io::Error),
    DieselConnection(diesel::ConnectionError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Error(s) => f.write_str(s),
            Error::IO(e) => e.fmt(f),
            Error::DieselConnection(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Error(_) => None,
            Error::IO(e) => Some(e),
            Error::DieselConnection(e) => Some(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<diesel::ConnectionError> for Error {
    fn from(e: diesel::ConnectionError) -> Self {
        Error::DieselConnection(e)
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Self {
        Error::Error(String::from(e))
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Error(e)
    }
}
