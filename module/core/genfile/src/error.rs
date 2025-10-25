//! Error handling and formatting
//!
//! Provides utilities for converting `genfile_core` errors to unilang `ErrorData`
//! and formatting error messages according to cli.rulebook.md standards.

use unilang::data::{ ErrorData, ErrorCode };

/// Format `genfile_core` error as `ErrorData` for unilang
///
/// Converts errors from `genfile_core` operations into structured `ErrorData`
/// with proper context labeling following the format: `[ERROR] [CONTEXT]: message`
///
/// # Parameters
///
/// - `error`: The `genfile_core::Error` to format
/// - `context`: Context label (e.g., "FILE", "PARAMETER", "RENDER")
///
/// # Examples
///
/// ```
/// use genfile::error::format_error;
/// use genfile_core::Error;
///
/// let err = Error::Fs( std::io::Error::from( std::io::ErrorKind::NotFound ) );
/// let error_data = format_error( err, "FILE" );
/// assert!( error_data.message.contains( "[ERROR] [FILE]:" ) );
/// ```
#[must_use] 
pub fn format_error( error : genfile_core::Error, context : &str ) -> ErrorData
{
  let message = format!( "[ERROR] [{}]: {}", context.to_uppercase(), error );

  ErrorData
  {
    code : ErrorCode::InternalError,
    message,
    source : None,
  }
}

/// Format usage error for missing or invalid parameters
///
/// Creates `ErrorData` for user input problems.
///
/// # Examples
///
/// ```
/// use genfile::error::usage_error;
///
/// let err = usage_error( "Missing required parameter: name" );
/// assert!( err.message.contains( "[ERROR] [USAGE]:" ) );
/// ```
pub fn usage_error( message : impl Into< String > ) -> ErrorData
{
  ErrorData
  {
    code : ErrorCode::ArgumentMissing,
    message : format!( "[ERROR] [USAGE]: {}", message.into() ),
    source : None,
  }
}

/// Format parameter-related error
pub fn _parameter_error( message : impl Into< String > ) -> ErrorData
{
  ErrorData
  {
    code : ErrorCode::UnknownParameter,
    message : format!( "[ERROR] [PARAMETER]: {}", message.into() ),
    source : None,
  }
}

/// Format file-related error
pub fn file_error( message : impl Into< String > ) -> ErrorData
{
  ErrorData
  {
    code : ErrorCode::InternalError,
    message : format!( "[ERROR] [FILE]: {}", message.into() ),
    source : None,
  }
}

/// Format validation error
pub fn validation_error( message : impl Into< String > ) -> ErrorData
{
  ErrorData
  {
    code : ErrorCode::ValidationRuleFailed,
    message : format!( "[ERROR] [VALIDATION]: {}", message.into() ),
    source : None,
  }
}

/// Format state-related error
pub fn state_error( message : impl Into< String > ) -> ErrorData
{
  ErrorData
  {
    code : ErrorCode::InternalError,
    message : format!( "[ERROR] [STATE]: {}", message.into() ),
    source : None,
  }
}
