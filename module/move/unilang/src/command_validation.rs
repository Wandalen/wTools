//! Command registration validation and utilities.
//!
//! This module centralizes all validation logic used across different
//! registration approaches (runtime, YAML, JSON, Rust DSL) to ensure
//! consistent behavior and avoid code duplication.
//!
//! # Validation Rules
//!
//! All commands must follow these rules:
//! - Command names must start with '.' prefix
//! - Non-empty namespaces must start with '.' prefix
//! - Empty namespaces are allowed for root-level commands
//!
//! # Examples
//!
//! ```rust
//! use unilang::command_validation::{ validate_command_name, validate_namespace };
//!
//! // Valid names
//! assert!( validate_command_name( ".hello" ).is_ok() );
//! assert!( validate_command_name( ".video.search" ).is_ok() );
//!
//! // Invalid names
//! assert!( validate_command_name( "hello" ).is_err() );
//! assert!( validate_command_name( "hello.world" ).is_err() );
//!
//! // Valid namespaces
//! assert!( validate_namespace( "" ).is_ok() );           // Empty is OK
//! assert!( validate_namespace( ".video" ).is_ok() );
//!
//! // Invalid namespaces
//! assert!( validate_namespace( "video" ).is_err() );     // Missing dot
//! ```

/// Internal namespace.
mod private
{
  use crate::{ error::Error, data::CommandDefinition };

/// Validates command name follows dot-prefix naming convention.
///
/// # Errors
///
/// Returns `Error::Registration` if name doesn't start with '.'
///
/// # Examples
///
/// ```rust
/// use unilang::command_validation::validate_command_name;
///
/// assert!( validate_command_name( ".hello" ).is_ok() );
/// assert!( validate_command_name( "hello" ).is_err() );
/// ```
pub fn validate_command_name( name : &str ) -> Result< (), Error >
{
  if !name.starts_with( '.' )
  {
    return Err( Error::Registration( format!(
      "Invalid command name '{}'. All commands must start with dot prefix (e.g., '.chat'). \
      This enforces explicit naming with minimal implicit transformations.",
      name
    )));
  }
  Ok(())
}

/// Validates namespace follows dot-prefix naming convention.
///
/// Empty namespaces are allowed (for root-level commands).
/// Non-empty namespaces must start with '.'.
///
/// # Errors
///
/// Returns `Error::Registration` if non-empty namespace doesn't start with '.'
///
/// # Examples
///
/// ```rust
/// use unilang::command_validation::validate_namespace;
///
/// assert!( validate_namespace( "" ).is_ok() );         // Empty OK
/// assert!( validate_namespace( ".session" ).is_ok() );
/// assert!( validate_namespace( "session" ).is_err() ); // Missing dot
/// ```
pub fn validate_namespace( namespace : &str ) -> Result< (), Error >
{
  if !namespace.is_empty() && !namespace.starts_with( '.' )
  {
    return Err( Error::Registration( format!(
      "Invalid namespace '{}'. Non-empty namespaces must start with dot prefix (e.g., '.session'). \
      Use empty namespace for root-level commands.",
      namespace
    )));
  }
  Ok(())
}

/// Validates entire command definition for registration.
///
/// Checks:
/// - Command name has dot prefix
/// - Namespace has dot prefix (if non-empty)
///
/// This is the primary validation function used by all registration paths.
///
/// # Errors
///
/// Returns `Error::Registration` if validation fails
///
/// # Examples
///
/// ```rust
/// use unilang::prelude::*;
///
/// let cmd = CommandDefinition::former()
///   .name( ".hello".to_string() )
///   .description( "Test command".to_string() )
///   .end();
///
/// assert!( validate_command_for_registration( &cmd ).is_ok() );
/// ```
pub fn validate_command_for_registration( cmd : &CommandDefinition ) -> Result< (), Error >
{
  validate_command_name( &cmd.name )?;
  validate_namespace( &cmd.namespace )?;
  Ok(())
}

/// Checks if command name ends with ".help" suffix.
///
/// Used to avoid creating help commands for help commands (prevent recursion).
///
/// # Examples
///
/// ```rust
/// use unilang::command_validation::is_help_command;
///
/// assert!( is_help_command( ".hello.help" ) );
/// assert!( !is_help_command( ".hello" ) );
/// ```
#[ must_use ]
#[ allow( clippy::case_sensitive_file_extension_comparisons ) ] // .help is not a file extension
pub fn is_help_command( full_name : &str ) -> bool
{
  full_name.ends_with( ".help" )
}

/// Builds help command name from command name.
///
/// # Examples
///
/// ```rust
/// use unilang::command_validation::make_help_command_name;
///
/// assert_eq!( make_help_command_name( ".hello" ), ".hello.help" );
/// assert_eq!( make_help_command_name( ".video.search" ), ".video.search.help" );
/// ```
#[ must_use ]
pub fn make_help_command_name( full_name : &str ) -> String
{
  format!( "{}.help", full_name )
}

}

mod_interface::mod_interface!
{
  exposed use private::validate_command_name;
  exposed use private::validate_namespace;
  exposed use private::validate_command_for_registration;
  exposed use private::is_help_command;
  exposed use private::make_help_command_name;

  prelude use private::validate_command_name;
  prelude use private::validate_namespace;
  prelude use private::validate_command_for_registration;
  prelude use private::is_help_command;
  prelude use private::make_help_command_name;
}
