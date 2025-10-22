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

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use std::path::Path;

  #[ test ]
  fn rejects_parent_dir()
  {
    assert!( validate_path( Path::new( "../etc/passwd" ) ).is_err() );
  }

  #[ test ]
  fn rejects_nested_parent_dir()
  {
    assert!( validate_path( Path::new( "foo/../../bar" ) ).is_err() );
  }

  #[ test ]
  fn rejects_single_parent_in_middle()
  {
    assert!( validate_path( Path::new( "a/../b" ) ).is_err() );
  }

  #[ test ]
  fn allows_normal_paths()
  {
    assert!( validate_path( Path::new( "foo/bar.txt" ) ).is_ok() );
    assert!( validate_path( Path::new( "src/lib.rs" ) ).is_ok() );
    assert!( validate_path( Path::new( "a/b/c/d.txt" ) ).is_ok() );
  }

  #[ test ]
  fn allows_current_dir()
  {
    assert!( validate_path( Path::new( "./foo/bar.txt" ) ).is_ok() );
    assert!( validate_path( Path::new( "foo/./bar.txt" ) ).is_ok() );
  }

  #[ test ]
  fn allows_simple_filename()
  {
    assert!( validate_path( Path::new( "file.txt" ) ).is_ok() );
  }

  #[ test ]
  fn error_message_includes_path()
  {
    let result = validate_path( Path::new( "../malicious" ) );
    assert!( result.is_err() );

    let err = result.unwrap_err();
    let msg = format!( "{err}" );
    assert!( msg.contains( "directory traversal" ) );
    assert!( msg.contains( "../malicious" ) );
  }
}
