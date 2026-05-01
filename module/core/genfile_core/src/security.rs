/// Security validation functions for genfile operations
///
/// Provides security checks to prevent directory traversal attacks and other
/// security vulnerabilities when materializing templates.
use std::path::{ Path, Component };
use crate::Error;

/// Validates that path doesnt contain directory traversal sequences.
///
/// # Security
///
/// Prevents malicious templates from writing files outside the target
/// directory by rejecting paths containing `..` segments.
///
/// # Errors
///
/// Returns `Error::InvalidTemplate` if path contains `..` segments.
///
/// # Examples
///
/// ```rust
/// use genfile_core::validate_path;
/// use std::path::Path;
///
/// // Valid paths
/// assert!( validate_path( Path::new( "foo/bar.txt" ) ).is_ok() );
/// assert!( validate_path( Path::new( "./foo/bar.txt" ) ).is_ok() );
/// assert!( validate_path( Path::new( "src/main.rs" ) ).is_ok() );
///
/// // Invalid paths
/// assert!( validate_path( Path::new( "../etc/passwd" ) ).is_err() );
/// assert!( validate_path( Path::new( "foo/../../bar" ) ).is_err() );
/// assert!( validate_path( Path::new( "a/../b" ) ).is_err() );
/// ```
pub fn validate_path( path: &Path ) -> Result< (), Error >
{
  for component in path.components()
  {
    if component == Component::ParentDir
    {
      return Err( Error::InvalidTemplate(
        format!( "Path contains directory traversal: {}", path.display() )
      ));
    }
  }
  Ok( () )
}
