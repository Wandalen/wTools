//!
//! The error types for the Unilang framework.
//!

use crate::data::ErrorData;

///
/// The main error type for the Unilang framework.
///
#[ derive( Debug ) ]
pub enum Error
{
  /// An error that occurred during semantic analysis or execution.
  Execution( ErrorData ),
}

impl From< ErrorData > for Error
{
  fn from( error : ErrorData ) -> Self
  {
    Error::Execution( error )
  }
}