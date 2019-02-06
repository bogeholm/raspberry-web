// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html
// https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html

use actix_web::error::ResponseError;
use diesel::result::Error as dieselError;
use std::env::VarError as stdVarError;
use std::io::Error as stdIoError;
use std::num::ParseIntError as numParseIntError;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum RpWebError {
    // Defer to other error type implementation.
    ParseIntError(numParseIntError),
    VarError(stdVarError),
    IoError(stdIoError),
    DbError(dieselError),
    Generic(String),
}

/// RpWebError::new("error string")
impl RpWebError {
    pub fn new(message: &str) -> RpWebError {
        RpWebError::Generic(message.to_string())
    }
}

impl fmt::Display for RpWebError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
            RpWebError::ParseIntError(ref err) => err.fmt(formatter),
            RpWebError::VarError(ref err) => err.fmt(formatter),
            RpWebError::IoError(ref err) => err.fmt(formatter),
            RpWebError::DbError(ref err) => err.fmt(formatter),
            RpWebError::Generic(ref errs) => write!(formatter, "{}", errs),
        }
    }
}

impl error::Error for RpWebError {
    fn description(&self) -> &str {
        match *self {
            // These already impls `Error`, so defer to its own implementation.
            RpWebError::ParseIntError(ref err) => err.description(),
            RpWebError::VarError(ref err) => err.description(),
            RpWebError::IoError(ref err) => err.description(),
            RpWebError::DbError(ref err) => err.description(),
            RpWebError::Generic(ref _errs) => "Generic RpWebError",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This number because the
            // underlying type already implements the `Error` trait.
            RpWebError::ParseIntError(ref err) => Some(err),
            RpWebError::VarError(ref err) => Some(err),
            RpWebError::IoError(ref err) => Some(err),
            RpWebError::DbError(ref err) => Some(err),
            RpWebError::Generic(ref _errs) => None,
        }
    }
}

// Implement the conversion from `<E>` to `RpWebError`.
// This will be automatically called by `?` if an `<E>>`
// needs to be converted into a `RpWebError`.
impl From<numParseIntError> for RpWebError {
    fn from(err: numParseIntError) -> RpWebError {
        RpWebError::ParseIntError(err)
    }
}

impl From<stdVarError> for RpWebError {
    fn from(err: stdVarError) -> RpWebError {
        RpWebError::VarError(err)
    }
}

impl From<stdIoError> for RpWebError {
    fn from(err: stdIoError) -> RpWebError {
        RpWebError::IoError(err)
    }
}

impl From<dieselError> for RpWebError {
    fn from(err: dieselError) -> RpWebError {
        RpWebError::DbError(err)
    }
}

// Avoids the trait `actix_web::error::ResponseError` is not implemented for `RpWebError`
// https://github.com/actix/actix-website/blob/master/content/docs/errors.md
// Use default implementation for `error_response()` method
impl ResponseError for RpWebError {}
