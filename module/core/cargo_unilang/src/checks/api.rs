//! Check for deprecated API usage

use super::CheckResult;
use std::{ fs, path::Path };
use walkdir::WalkDir;

/// Check if project uses deprecated CommandRegistry::new()
pub fn check_deprecated_api( project_path : &Path ) -> CheckResult
{
  let src_dir = project_path.join( "src" );

  if !src_dir.exists()
  {
    return CheckResult::passed();
  }

  // Walk through all .rs files in src/
  for entry in WalkDir::new( &src_dir )
    .into_iter()
    .filter_map( |e| e.ok() )
    .filter( |e| e.path().extension().map( |ext| ext == "rs" ).unwrap_or( false ) )
  {
    let content = match fs::read_to_string( entry.path() )
    {
      Ok( c ) => c,
      Err( _ ) => continue,
    };

    // Check for deprecated API
    if content.contains( "CommandRegistry::new()" )
    {
      let relative_path = entry.path()
        .strip_prefix( project_path )
        .unwrap_or( entry.path() )
        .display()
        .to_string();

      return CheckResult::failed(
        "Deprecated API usage",
        relative_path,
        "CommandRegistry::new() is deprecated",
        "Use CommandRegistry::with_static_commands() instead",
        "https://docs.rs/unilang/latest/unilang/struct.CommandRegistry.html"
      );
    }
  }

  CheckResult::passed()
}

#[cfg(test)]
mod tests
{
  use super::*;
  use assert_fs::prelude::*;

  #[test]
  fn test_no_deprecated_api_passes()
  {
    let temp = assert_fs::TempDir::new().unwrap();
    let src = temp.child( "src" );
    src.create_dir_all().unwrap();
    src.child( "main.rs" ).write_str(
r"use unilang::prelude::*;
fn main() {
  let registry = CommandRegistry::with_static_commands();
}
"
    ).unwrap();

    let result = check_deprecated_api( temp.path() );
    assert!( result.passed );
  }

  #[test]
  fn test_deprecated_new_fails()
  {
    let temp = assert_fs::TempDir::new().unwrap();
    let src = temp.child( "src" );
    src.create_dir_all().unwrap();
    src.child( "main.rs" ).write_str(
r"use unilang::prelude::*;
fn main() {
  let registry = CommandRegistry::new();
}
"
    ).unwrap();

    let result = check_deprecated_api( temp.path() );
    assert!( !result.passed );
    assert!( result.issue_type.contains( "Deprecated API" ) );
    assert!( result.fix.contains( "with_static_commands" ) );
  }

  #[test]
  fn test_check_in_subdirectory()
  {
    let temp = assert_fs::TempDir::new().unwrap();
    let src = temp.child( "src/commands" );
    src.create_dir_all().unwrap();
    src.child( "mod.rs" ).write_str(
      "let x = CommandRegistry::new();"
    ).unwrap();

    let result = check_deprecated_api( temp.path() );
    assert!( !result.passed );
  }
}
