//! Core validation logic shared between runtime and build.rs.
//!
//! **IMPORTANT**: This module is designed to be included in build.rs via `include!()`.
//! It MUST NOT depend on any crate types (Error, CommandDefinition, etc.).
//! All functions return `Result<(), String>` for portability.
//!
//! # Usage in build.rs
//!
//! ```ignore
//! // In build.rs, include this module:
//! include!("src/validation_core.rs");
//!
//! fn process_command(name: &str) {
//!   if let Err(msg) = validate_command_name_core(name) {
//!     panic!("ERROR: {}", msg);
//!   }
//! }
//! ```
//!
//! # Usage in runtime
//!
//! The `command_validation` module wraps these functions and converts
//! String errors to the proper Error type.
//!
//! # Design Rationale
//!
//! This module exists to solve H37 (build.rs does NOT use validation functions).
//! By extracting core validation logic with no dependencies, we can:
//! 1. Share validation between runtime and compile-time
//! 2. Ensure consistent validation rules
//! 3. Avoid code duplication (per project rules)
//! 4. Guarantee that if build.rs validates successfully, From conversion cannot panic

/// Internal namespace.
mod private
{

/// Validates command name follows dot-prefix naming convention.
///
/// # Arguments
///
/// * `name` - The command name to validate
///
/// # Returns
///
/// * `Ok(())` if name starts with '.'
/// * `Err(String)` with error message if validation fails
///
/// # Examples
///
/// ```rust
/// use unilang::validation_core::validate_command_name_core;
///
/// assert!(validate_command_name_core(".hello").is_ok());
/// assert!(validate_command_name_core("hello").is_err());
/// ```
pub fn validate_command_name_core( name : &str ) -> Result< (), String >
{
  if name.is_empty()
  {
    return Err( "Command name cannot be empty".to_string() );
  }

  if !name.starts_with( '.' )
  {
    return Err( format!(
      "Invalid command name '{}'. All commands must start with dot prefix (e.g., '.chat'). \
      This enforces explicit naming with minimal implicit transformations.",
      name
    ));
  }

  Ok(())
}

/// Validates namespace follows dot-prefix naming convention.
///
/// Empty namespaces are allowed (for root-level commands).
/// Non-empty namespaces must start with '.'.
///
/// # Arguments
///
/// * `namespace` - The namespace to validate
///
/// # Returns
///
/// * `Ok(())` if namespace is empty or starts with '.'
/// * `Err(String)` with error message if validation fails
///
/// # Examples
///
/// ```rust
/// use unilang::validation_core::validate_namespace_core;
///
/// assert!(validate_namespace_core("").is_ok());          // Empty OK
/// assert!(validate_namespace_core(".session").is_ok());
/// assert!(validate_namespace_core("session").is_err());  // Missing dot
/// ```
pub fn validate_namespace_core( namespace : &str ) -> Result< (), String >
{
  if !namespace.is_empty() && !namespace.starts_with( '.' )
  {
    return Err( format!(
      "Invalid namespace '{}'. Non-empty namespaces must start with dot prefix (e.g., '.session'). \
      Use empty namespace for root-level commands.",
      namespace
    ));
  }

  Ok(())
}

/// Validates version string is non-empty.
///
/// # Arguments
///
/// * `version` - The version string to validate
///
/// # Returns
///
/// * `Ok(())` if version is non-empty
/// * `Err(String)` with error message if validation fails
///
/// # Examples
///
/// ```rust
/// use unilang::validation_core::validate_version_core;
///
/// assert!(validate_version_core("1.0.0").is_ok());
/// assert!(validate_version_core("").is_err());
/// ```
pub fn validate_version_core( version : &str ) -> Result< (), String >
{
  if version.is_empty()
  {
    return Err( "Version string cannot be empty".to_string() );
  }

  Ok(())
}

/// Validates the full name (namespace + name combination).
///
/// The full name must start with '.' to be valid.
/// This is the final validation after namespace and name are combined.
///
/// # Arguments
///
/// * `full_name` - The combined full command name
///
/// # Returns
///
/// * `Ok(())` if full_name starts with '.'
/// * `Err(String)` with error message if validation fails
///
/// # Examples
///
/// ```rust
/// use unilang::validation_core::validate_full_name_core;
///
/// assert!(validate_full_name_core(".test.command").is_ok());
/// assert!(validate_full_name_core("test.command").is_err());
/// ```
pub fn validate_full_name_core( full_name : &str ) -> Result< (), String >
{
  if full_name.is_empty()
  {
    return Err( "Full command name cannot be empty".to_string() );
  }

  if !full_name.starts_with( '.' )
  {
    return Err( format!(
      "Invalid full command name '{}'. Commands must start with dot prefix.",
      full_name
    ));
  }

  Ok(())
}

/// Computes full command name from namespace and name.
///
/// This mirrors the logic in CommandDefinition::full_name() for use in build.rs.
///
/// # Arguments
///
/// * `namespace` - The command namespace (may be empty)
/// * `name` - The command name
///
/// # Returns
///
/// The combined full name
///
/// # Examples
///
/// ```rust
/// use unilang::validation_core::compute_full_name_core;
///
/// assert_eq!(compute_full_name_core("", ".chat"), ".chat");
/// assert_eq!(compute_full_name_core(".session", "list"), ".session.list");
/// ```
#[ must_use ]
pub fn compute_full_name_core( namespace : &str, name : &str ) -> String
{
  if namespace.is_empty()
  {
    name.to_string()
  }
  else
  {
    format!( "{}.{}", namespace, name )
  }
}

/// Validates a complete command definition at build time.
///
/// Performs all validations needed to guarantee that From conversion
/// will not panic at runtime.
///
/// # Arguments
///
/// * `name` - The command name
/// * `namespace` - The command namespace
/// * `version` - The command version
/// * `file_path` - Path to source file (for error messages)
///
/// # Returns
///
/// * `Ok(())` if all validations pass
/// * `Err(String)` with detailed error message if any validation fails
pub fn validate_command_definition_core(
  name : &str,
  namespace : &str,
  version : &str,
  file_path : &str,
) -> Result< (), String >
{
  // Validate individual fields
  if let Err( e ) = validate_command_name_core( name )
  {
    return Err( format!( "In file '{}': {}", file_path, e ) );
  }

  if let Err( e ) = validate_namespace_core( namespace )
  {
    return Err( format!( "In file '{}': {}", file_path, e ) );
  }

  if let Err( e ) = validate_version_core( version )
  {
    return Err( format!(
      "In file '{}': Command '{}' has invalid version: {}",
      file_path, name, e
    ));
  }

  // Validate full name
  let full_name = compute_full_name_core( namespace, name );
  if let Err( e ) = validate_full_name_core( &full_name )
  {
    return Err( format!( "In file '{}': {}", file_path, e ) );
  }

  Ok(())
}

}

mod_interface::mod_interface!
{
  exposed use private::validate_command_name_core;
  exposed use private::validate_namespace_core;
  exposed use private::validate_version_core;
  exposed use private::validate_full_name_core;
  exposed use private::compute_full_name_core;
  exposed use private::validate_command_definition_core;

  prelude use private::validate_command_name_core;
  prelude use private::validate_namespace_core;
  prelude use private::validate_version_core;
  prelude use private::validate_full_name_core;
  prelude use private::compute_full_name_core;
  prelude use private::validate_command_definition_core;
}
