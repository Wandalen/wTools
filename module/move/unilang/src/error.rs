//!
//! The error types for the Unilang framework.
//!
//! # Error Handling Patterns for REPL Applications
//!
//! This module defines error types optimized for REPL (Read-Eval-Print Loop) usage:
//!
//! ## Critical Error Codes for REPL Integration
//! - `UNILANG_ARGUMENT_INTERACTIVE_REQUIRED`: Signals need for secure user input
//! - `UNILANG_TYPE_MISMATCH`: Type conversion failures that should show user-friendly suggestions  
//! - `UNILANG_ARGUMENT_MISSING`: Missing required arguments with correction hints
//!
//! ## Error Recovery Strategy
//! All errors are designed to be non-fatal for REPL sessions:
//! - Parse errors don't corrupt the parser state
//! - Semantic errors don't affect the registry
//! - Execution errors don't crash the application
//! - Interactive errors provide clear next steps for the user
//!
//! ## Security Considerations
//! - Error messages never contain sensitive argument values
//! - Interactive argument errors are deliberately generic
//! - Stack traces are sanitized in production REPL environments
//!

/// Internal namespace.
mod private
{
  use crate::data::ErrorData;
  use serde_json;
  use serde_yaml;
  use error_tools::dependency::thiserror;

  ///
  /// The main error type for the Unilang framework.
  ///
  /// This enum consolidates all possible errors that can occur within the
  /// framework, providing a single, consistent error handling mechanism.
  #[ derive( thiserror::Error, Debug ) ]
  pub enum Error
  {
    /// An error that occurred during semantic analysis or execution,
    /// containing detailed information about the failure.
    #[ error( "Execution Error: {0}" ) ]
    Execution( ErrorData ),
    /// An error that occurred during command registration.
    #[ error( "Registration Error: {0}" ) ]
    Registration( String ),
    /// An error that occurred during YAML deserialization.
    #[ error( "YAML Deserialization Error: {0}" ) ]
    Yaml( #[ from ] serde_yaml::Error ),
    /// An error that occurred during JSON deserialization.
    #[ error( "JSON Deserialization Error: {0}" ) ]
    Json( #[ from ] serde_json::Error ),
    /// An error that occurred during parsing.
    #[ error( "Parse Error: {0}" ) ]
    Parse( #[ from ] unilang_parser::error::ParseError ),
  }

  impl From< crate::types::TypeError > for Error
  {
    fn from( error : crate::types::TypeError ) -> Self
    {
      Error::Execution( crate::data::ErrorData::new(
        "UNILANG_TYPE_MISMATCH".to_string(),
        format!( "Type Error: {}. Please provide a valid value for this type.", error.reason ),
      ))
    }
  }

  impl From< ErrorData > for Error
  {
    /// Converts an `ErrorData` into an `Error`.
    fn from( error : ErrorData ) -> Self
    {
      Error::Execution( error )
    }
  }

}

mod_interface::mod_interface!
{
  exposed use private::Error;
  
  prelude use private::Error;
}
