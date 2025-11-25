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

/// Validates parameter storage types match their multiple attribute.
///
/// Prevents the wplan bug pattern where `multiple: true` is used with
/// non-List storage types, causing silent data loss when multiple values
/// overwrite each other.
///
/// # Errors
///
/// Returns `Error::Registration` if any parameter has `multiple: true`
/// but kind is not `Kind::List`.
///
/// # Examples
///
/// ```rust
/// use unilang::prelude::*;
///
/// // CORRECT: multiple:true with Kind::List
/// let cmd = CommandDefinition::former()
///   .name( ".test" )
///   .description( "Test command" )
///   .arguments( vec![
///     ArgumentDefinition {
///       name: "files".to_string(),
///       description: "Input files".to_string(),
///       kind: Kind::List( Box::new( Kind::String ), None ),
///       hint: String::new(),
///       attributes: ArgumentAttributes {
///         multiple: true,
///         optional: true,
///         ..Default::default()
///       },
///       validation_rules: vec![],
///       aliases: vec![],
///       tags: vec![],
///     }
///   ])
///   .end();
///
/// assert!( validate_parameter_storage_types( &cmd ).is_ok() );
/// ```
pub fn validate_parameter_storage_types( cmd : &CommandDefinition ) -> Result< (), Error >
{
  use crate::data::Kind;

  for arg in cmd.arguments()
  {
    if arg.attributes.multiple
    {
      match &arg.kind
      {
        Kind::List( _, _ ) => {
          // Correct - multiple:true with List storage
        }
        _ => {
          return Err( Error::Registration( format!(
            "Parameter '{}' in command '{}' has multiple:true but storage type is {:?}. \
            Parameters accepting multiple values must use Kind::List storage to prevent data loss. \
            \n\nThis prevents the wplan bug pattern where multiple values silently overwrite each other. \
            \n\nChange to: Kind::List( Box::new( Kind::String ), None ) or similar List variant.",
            arg.name,
            cmd.name().as_str(),
            arg.kind
          )));
        }
      }
    }
  }
  Ok(())
}

/// Validates entire command definition for registration.
///
/// Checks:
/// - Command name has dot prefix
/// - Namespace has dot prefix (if non-empty)
/// - Parameter storage types match multiple attribute
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
  // Validate the final full name (which combines namespace + name)
  let full_name = cmd.full_name();
  if !full_name.starts_with( '.' )
  {
    return Err( Error::Registration( format!(
      "Invalid command name '{}'. All commands must start with dot prefix (e.g., '.chat'). \
      This enforces explicit naming with minimal implicit transformations.",
      full_name
    )));
  }

  validate_namespace( cmd.namespace() )?;
  validate_parameter_storage_types( cmd )?;
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
  exposed use private::validate_parameter_storage_types;
  exposed use private::validate_command_for_registration;
  exposed use private::is_help_command;
  exposed use private::make_help_command_name;

  prelude use private::validate_command_name;
  prelude use private::validate_namespace;
  prelude use private::validate_parameter_storage_types;
  prelude use private::validate_command_for_registration;
  prelude use private::is_help_command;
  prelude use private::make_help_command_name;
}
