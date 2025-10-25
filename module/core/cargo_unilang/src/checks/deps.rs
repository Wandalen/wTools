//! Check for duplicate dependencies already provided by unilang

use super::CheckResult;
use std::{ fs, path::Path };
use toml_edit::{ DocumentMut, Item };

/// Check if Cargo.toml has duplicate dependencies
pub fn check_duplicate_deps( project_path : &Path ) -> CheckResult
{
  let cargo_toml_path = project_path.join( "Cargo.toml" );

  // Read Cargo.toml
  let content = match fs::read_to_string( &cargo_toml_path )
  {
    Ok( c ) => c,
    Err( _ ) => return CheckResult::passed(), // No Cargo.toml, skip check
  };

  // Parse TOML
  let doc = match content.parse::< DocumentMut >()
  {
    Ok( d ) => d,
    Err( _ ) => return CheckResult::passed(), // Invalid TOML, skip check
  };

  // Check for duplicate dependencies in [dependencies] and [build-dependencies]
  let mut duplicates = Vec::new();

  // Check [dependencies]
  if let Some( Item::Table( deps ) ) = doc.get( "dependencies" )
  {
    if deps.contains_key( "serde_yaml" )
    {
      duplicates.push( "serde_yaml" );
    }
    if deps.contains_key( "walkdir" )
    {
      duplicates.push( "walkdir" );
    }
    if deps.contains_key( "phf" ) || deps.contains_key( "phf_codegen" )
    {
      duplicates.push( "phf" );
    }
  }

  // Check [build-dependencies]
  if let Some( Item::Table( build_deps ) ) = doc.get( "build-dependencies" )
  {
    if build_deps.contains_key( "serde_yaml" ) && !duplicates.contains( &"serde_yaml" )
    {
      duplicates.push( "serde_yaml" );
    }
    if build_deps.contains_key( "walkdir" ) && !duplicates.contains( &"walkdir" )
    {
      duplicates.push( "walkdir" );
    }
    if ( build_deps.contains_key( "phf" ) || build_deps.contains_key( "phf_codegen" ) ) && !duplicates.contains( &"phf" )
    {
      duplicates.push( "phf" );
    }
  }

  if duplicates.is_empty()
  {
    CheckResult::passed()
  }
  else
  {
    let deps_list = duplicates.join( ", " );
    CheckResult::failed(
      "Duplicate dependencies",
      "Cargo.toml [dependencies] or [build-dependencies]",
      format!( "{} already provided by unilang", deps_list ),
      format!( "Remove {} from Cargo.toml", deps_list ),
      "https://docs.rs/unilang/latest/unilang/#anti-patterns"
    )
  }
}

#[cfg(test)]
mod tests
{
  use super::*;
  use assert_fs::prelude::*;

  #[test]
  fn test_no_duplicates_passes()
  {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child( "Cargo.toml" ).write_str(
r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"

[dependencies]
unilang = "0.30"
"#
    ).unwrap();

    let result = check_duplicate_deps( temp.path() );
    assert!( result.passed );
  }

  #[test]
  fn test_serde_yaml_duplicate_fails()
  {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child( "Cargo.toml" ).write_str(
r#"[package]
name = "test"
version = "0.1.0"

[dependencies]
unilang = "0.30"
serde_yaml = "0.9"
"#
    ).unwrap();

    let result = check_duplicate_deps( temp.path() );
    assert!( !result.passed );
    assert!( result.issue.contains( "serde_yaml" ) );
  }

  #[test]
  fn test_multiple_duplicates_fails()
  {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child( "Cargo.toml" ).write_str(
r#"[dependencies]
unilang = "0.30"
serde_yaml = "0.9"
walkdir = "2.0"
phf = "0.11"
"#
    ).unwrap();

    let result = check_duplicate_deps( temp.path() );
    assert!( !result.passed );
    assert!( result.issue.contains( "serde_yaml" ) );
    assert!( result.issue.contains( "walkdir" ) );
    assert!( result.issue.contains( "phf" ) );
  }
}
