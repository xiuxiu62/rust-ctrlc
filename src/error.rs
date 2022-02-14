use crate::platform;
use std::{fmt, sync::PoisonError};

/// Ctrl-C error.
#[derive(Debug)]
pub enum Error {
    /// Signal could not be found from the system.
    NoSuchSignal(crate::SignalType),
    /// Ctrl-C signal handler already registered.
    MultipleHandlers,
    /// Unexpected system error.
    System(std::io::Error),
    /// Lock poisoned
    Poison(String),
    /// Other external errors
    Other(Box<dyn std::error::Error>),
}

impl Error {
    fn describe(&self) -> String {
        match *self {
            Error::NoSuchSignal(_) => "Signal could not be found from the system".to_owned(),
            Error::MultipleHandlers => "Ctrl-C signal handler already registered".to_owned(),
            Error::System(_) => "Unexpected system error".to_owned(),
            Error::Poison(ref msg) => msg.to_owned(),
            Error::Other(ref err) => err.to_string(),
        }
    }
}

impl From<platform::Error> for Error {
    fn from(e: platform::Error) -> Error {
        let system_error = std::io::Error::new(std::io::ErrorKind::Other, e);
        Error::System(system_error)
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        Self::Poison(err.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ctrl-C error: {}", self.describe())
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::System(ref e) => Some(e),
            _ => None,
        }
    }
}

unsafe impl Send for Error {}
