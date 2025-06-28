//!
//! The error types for the Unilang framework.
//!

use crate::data::ErrorData;

///
/// The main error type for the Unilang framework.
///
/// This enum consolidates all possible errors that can occur within the
/// framework, providing a single, consistent error handling mechanism.
#[ derive( Debug ) ]
pub enum Error
{
  /// An error that occurred during semantic analysis or execution,
  /// containing detailed information about the failure.
  Execution( ErrorData ),
  /// An error that occurred during command registration.
  Registration( String ),
}

impl From< ErrorData > for Error
{
  /// Converts an `ErrorData` into an `Error`.
  fn from( error : ErrorData ) -> Self
  {
    Error::Execution( error )
  }
}