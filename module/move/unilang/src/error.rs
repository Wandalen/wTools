//!
//! The error types for the Unilang framework.
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
