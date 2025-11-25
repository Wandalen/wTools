//! Check for custom build.rs that duplicates unilang functionality

use super::CheckResult;
use std::{ fs, path::Path };

/// Check if project has custom build.rs with unilang keywords
pub fn check_build_rs( project_path : &Path ) -> CheckResult
{
  let build_rs_path = project_path.join( "build.rs" );

  // Check if build.rs exists
  if !build_rs_path.exists()
  {
    return CheckResult::passed();
  }

  // Read build.rs content
  let content = match fs::read_to_string( &build_rs_path )
  {
    Ok( c ) => c,
    Err( _ ) => return CheckResult::passed(), // Cannot read, skip check
  };

  // Check for unilang-related keywords
  let has_yaml_parsing = content.contains( "serde_yaml" )
    || content.contains( "yaml" )
    || content.contains( "commands.yaml" );

  let has_phf_generation = content.contains( "phf_codegen" )
    || content.contains( "phf" );

  let has_walkdir = content.contains( "walkdir" );

  if has_yaml_parsing || has_phf_generation || has_walkdir
  {
    let line_count = content.lines().count();
    CheckResult::failed(
      "Custom build.rs found",
      format!( "./build.rs ({} lines)", line_count ),
      "Duplicates unilang's built-in build system",
      "Delete build.rs - unilang provides this automatically",
      "https://docs.rs/unilang/latest/unilang/#anti-patterns"
    )
  }
  else
  {
    // build.rs exists but doesn't seem to duplicate unilang functionality
    CheckResult::passed()
  }
}

#[cfg(test)]
mod tests
{
  use super::*;
  use assert_fs::prelude::*;

  #[test]
  fn test_no_build_rs_passes()
  {
    let temp = assert_fs::TempDir::new().unwrap();
    let result = check_build_rs( temp.path() );
    assert!( result.passed );
  }

  #[test]
  fn test_build_rs_with_yaml_fails()
  {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child( "build.rs" ).write_str( "fn main() { let yaml = serde_yaml::from_str(); }" ).unwrap();

    let result = check_build_rs( temp.path() );
    assert!( !result.passed );
    assert!( result.issue_type.contains( "Custom build.rs" ) );
  }

  #[test]
  fn test_build_rs_with_phf_fails()
  {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child( "build.rs" ).write_str( "fn main() { phf_codegen::Map::new(); }" ).unwrap();

    let result = check_build_rs( temp.path() );
    assert!( !result.passed );
  }

  #[test]
  fn test_build_rs_without_unilang_stuff_passes()
  {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.child( "build.rs" ).write_str( "fn main() { println!(\"cargo:rerun-if-changed=build.rs\"); }" ).unwrap();

    let result = check_build_rs( temp.path() );
    assert!( result.passed );
  }
}
