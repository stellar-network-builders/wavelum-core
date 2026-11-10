//! Helper functions for error handling and validation in vesting vault.

use crate::errors::codes::Error;

/// Asserts that a condition is true, otherwise returning the specified [`Error`].
///
/// # Parameters
/// - `condition` - The boolean condition to evaluate.
/// - `err` - The error variant to return if the condition is false.
pub fn ensure(condition: bool, err: Error) -> Result<(), Error> {
    if !condition {
        return Err(err);
    }
    Ok(())
}

/// Unwraps an `Option<T>` into `Result<T, Error>`, returning the provided [`Error`] on `None`.
///
/// # Parameters
/// - `opt` - The optional value to unwrap.
/// - `err` - The error variant to return if `opt` is `None`.
pub fn unwrap_or_error<T>(opt: Option<T>, err: Error) -> Result<T, Error> {
    match opt {
        Some(val) => Ok(val),
        None => Err(err),
    }
}