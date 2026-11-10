//! Error definitions and helper functions for the vesting vault contract.

/// Structured error codes returned across all contract entrypoints.
pub mod codes;
/// Validation and error unwrap helper utilities.
pub mod helpers;

pub use codes::Error;
pub use helpers::*;