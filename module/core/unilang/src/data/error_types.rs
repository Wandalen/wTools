//! Error types for command execution failures
//!
//! Provides standardized error codes and error data structures for reporting
//! command execution failures with machine-readable codes and human-readable messages.


/// Standard error codes for command execution failures
///
/// Each variant maps to a unique string code (e.g., "UNILANG_COMMAND_NOT_FOUND")
/// for machine-readable error reporting.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum ErrorCode
{
  /// Command was not found in the registry.
  CommandNotFound,
  /// Required argument is missing.
  ArgumentMissing,
  /// Argument value has wrong type.
  ArgumentTypeMismatch,
  /// Interactive argument requires user input.
  ArgumentInteractiveRequired,
  /// Validation rule failed for argument value.
  ValidationRuleFailed,
  /// Too many positional arguments provided.
  TooManyArguments,
  /// Unknown named parameter provided.
  UnknownParameter,
  /// Command with same name already exists.
  CommandAlreadyExists,
  /// Command not implemented.
  CommandNotImplemented,
  /// Type conversion or mismatch error.
  TypeMismatch,
  /// Internal framework error.
  InternalError,
  /// Help information requested.
  HelpRequested,
}

impl ErrorCode
{
  /// Returns the canonical string representation of the error code.
  #[ must_use ]
  pub fn as_str( &self ) -> &'static str
  {
    match self
    {
      Self::CommandNotFound => "UNILANG_COMMAND_NOT_FOUND",
      Self::ArgumentMissing => "UNILANG_ARGUMENT_MISSING",
      Self::ArgumentTypeMismatch => "UNILANG_ARGUMENT_TYPE_MISMATCH",
      Self::ArgumentInteractiveRequired => "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED",
      Self::ValidationRuleFailed => "UNILANG_VALIDATION_RULE_FAILED",
      Self::TooManyArguments => "UNILANG_TOO_MANY_ARGUMENTS",
      Self::UnknownParameter => "UNILANG_UNKNOWN_PARAMETER",
      Self::CommandAlreadyExists => "UNILANG_COMMAND_ALREADY_EXISTS",
      Self::CommandNotImplemented => "UNILANG_COMMAND_NOT_IMPLEMENTED",
      Self::TypeMismatch => "UNILANG_TYPE_MISMATCH",
      Self::InternalError => "UNILANG_INTERNAL_ERROR",
      Self::HelpRequested => "HELP_REQUESTED",
    }
  }
}

impl core::fmt::Display for ErrorCode
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    write!( f, "{}", self.as_str() )
  }
}

impl core::str::FromStr for ErrorCode
{
  type Err = ();

  fn from_str( s : &str ) -> Result< Self, Self::Err >
  {
    match s
    {
      "UNILANG_COMMAND_NOT_FOUND" => Ok( Self::CommandNotFound ),
      "UNILANG_ARGUMENT_MISSING" => Ok( Self::ArgumentMissing ),
      "UNILANG_ARGUMENT_TYPE_MISMATCH" => Ok( Self::ArgumentTypeMismatch ),
      "UNILANG_ARGUMENT_INTERACTIVE_REQUIRED" => Ok( Self::ArgumentInteractiveRequired ),
      "UNILANG_VALIDATION_RULE_FAILED" => Ok( Self::ValidationRuleFailed ),
      "UNILANG_TOO_MANY_ARGUMENTS" => Ok( Self::TooManyArguments ),
      "UNILANG_UNKNOWN_PARAMETER" => Ok( Self::UnknownParameter ),
      "UNILANG_COMMAND_ALREADY_EXISTS" => Ok( Self::CommandAlreadyExists ),
      "UNILANG_COMMAND_NOT_IMPLEMENTED" => Ok( Self::CommandNotImplemented ),
      "UNILANG_TYPE_MISMATCH" => Ok( Self::TypeMismatch ),
      "UNILANG_INTERNAL_ERROR" => Ok( Self::InternalError ),
      "HELP_REQUESTED" => Ok( Self::HelpRequested ),
      _ => Err( () ),
    }
  }
}

/// Represents an error that occurred during command execution
///
/// Provides standardized error reporting with machine-readable codes,
/// human-readable messages, and error chaining support.
#[ derive( Debug, Clone /*, Former*/ ) ]
pub struct ErrorData
{
  /// A unique, machine-readable code for the error.
  pub code : ErrorCode,
  /// A human-readable message explaining the error.
  pub message : String,
  /// Optional source error for error chaining.
  pub source : Option< Box< ErrorData > >,
}

impl core::fmt::Display for ErrorData
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    writeln!( f, "{}", self.message )?;

    // Display error chain if present
    if let Some( source ) = &self.source
    {
      Self::fmt_error_chain( f, source, 1 )?;
    }

    Ok(())
  }
}

impl ErrorData
{
  /// Creates a new `ErrorData` with no source error
  #[ must_use ]
  pub fn new( code : ErrorCode, message : String ) -> Self
  {
    Self { code, message, source : None }
  }

  /// Creates a new `ErrorData` with a source error for chaining
  #[ must_use ]
  pub fn with_source( code : ErrorCode, message : String, source : ErrorData ) -> Self
  {
    Self { code, message, source : Some( Box::new( source ) ) }
  }

  /// Formats the error chain recursively with proper indentation
  fn fmt_error_chain( f : &mut core::fmt::Formatter< '_ >, error : &ErrorData, depth : usize ) -> core::fmt::Result
  {
    // Create indentation
    let indent = "  ".repeat( depth );
    writeln!( f, "{}↳ {}", indent, error.message )?;

    // Recursively display deeper sources
    if let Some( source ) = &error.source
    {
      Self::fmt_error_chain( f, source, depth + 1 )?;
    }

    Ok(())
  }
}
