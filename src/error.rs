use std::fmt::Display;

#[derive(Debug)]
pub struct Error(Box<ErrorImpl>);

impl Error {
    fn new(err: ErrorImpl) -> Self {
        Self(Box::new(err))
    }
}

impl<T: Into<ErrorImpl>> From<T> for Error {
    fn from(cause: T) -> Self {
        Self::new(cause.into())
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(thiserror::Error, Debug)]
enum ErrorImpl {}
