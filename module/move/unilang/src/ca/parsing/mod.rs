//!
//! Parsing module for the command aggregator.
//!

/// Handles the input abstraction for the parser.
pub mod input;
/// Defines the generic instruction format.
pub mod instruction;
/// Defines parsing error types.
pub mod error;
/// The main parsing engine.
pub mod engine;