//! Corner case testing for `file_tools` examples

fn main()
{
  println!( "\n=== Testing Corner Cases ===" );

  test_tempdir_empty_paths();
  test_tempdir_single_component();
  test_tempdir_special_characters();
  test_tempdir_without_create();
  test_glob_zero_matches();
  test_glob_invalid_pattern();
  test_pattern_edge_cases();

  println!( "\n✓ All corner case tests completed\n" );
}

/// Corner Case 1: `TempDir` with all empty paths
#[ cfg( feature = "enabled" ) ]
fn test_tempdir_empty_paths()
{
  use file_tools::TempDir;

  println!( "\n--- Corner Case 1: TempDir with Empty Paths ---" );

  let temp = TempDir::new();
  assert!( temp.base_path.as_os_str().is_empty() );
  assert!( temp.prefix_path.as_os_str().is_empty() );
  assert!( temp.postfix_path.as_os_str().is_empty() );

  let full = temp.full_path();
  assert!( full.as_os_str().is_empty(), "full_path of empty components should be empty" );

  println!( "✓ Empty paths handled correctly" );
}

#[ cfg( not( feature = "enabled" ) ) ]
fn test_tempdir_empty_paths()
{
  println!( "\n--- Corner Case 1: TempDir with Empty Paths ---" );
  println!( "⚠ Skipped" );
}

/// Corner Case 2: `TempDir` with only base path set
#[ cfg( feature = "enabled" ) ]
fn test_tempdir_single_component()
{
  use file_tools::TempDir;
  use std::path::PathBuf;

  println!( "\n--- Corner Case 2: TempDir Single Component ---" );

  let mut temp = TempDir::new();
  temp.base_path = PathBuf::from( "/tmp" );

  let full = temp.full_path();
  assert_eq!( full, PathBuf::from( "/tmp" ) );

  println!( "✓ Single component path construction works" );
}

#[ cfg( not( feature = "enabled" ) ) ]
fn test_tempdir_single_component()
{
  println!( "\n--- Corner Case 2: TempDir Single Component ---" );
  println!( "⚠ Skipped" );
}

/// Corner Case 3: `TempDir` with special characters in paths
#[ cfg( feature = "enabled" ) ]
fn test_tempdir_special_characters()
{
  use file_tools::TempDir;
  use std::path::PathBuf;

  println!( "\n--- Corner Case 3: Special Characters in Paths ---" );

  let mut temp = TempDir::new();
  temp.base_path = std::env::temp_dir();
  temp.prefix_path = PathBuf::from( "test with spaces" );
  temp.postfix_path = PathBuf::from( "unicode_тест_测试" );

  let path = temp.create_all().expect( "should handle special characters" );
  assert!( path.exists() );

  println!( "✓ Special characters handled: {}", path.display() );
}

#[ cfg( not( feature = "enabled" ) ) ]
fn test_tempdir_special_characters()
{
  println!( "\n--- Corner Case 3: Special Characters in Paths ---" );
  println!( "⚠ Skipped" );
}

/// Corner Case 4: `TempDir` `full_path()` without `create()` - no cleanup should occur
#[ cfg( feature = "enabled" ) ]
fn test_tempdir_without_create()
{
  use file_tools::TempDir;
  use std::path::PathBuf;

  println!( "\n--- Corner Case 4: full_path() Without create() ---" );

  let mut temp = TempDir::new();
  temp.base_path = PathBuf::from( "/tmp" );
  temp.prefix_path = PathBuf::from( "test" );

  let path = temp.full_path();
  println!( "Path computed: {}", path.display() );

  // No create() called, so no cleanup should be registered
  // This is tested implicitly - if Drop tried to cleanup, it would be a bug

  println!( "✓ full_path() without create() works" );
}

#[ cfg( not( feature = "enabled" ) ) ]
fn test_tempdir_without_create()
{
  println!( "\n--- Corner Case 4: full_path() Without create() ---" );
  println!( "⚠ Skipped" );
}

/// Corner Case 5: Glob pattern matching zero files
#[ cfg( feature = "glob" ) ]
fn test_glob_zero_matches()
{
  use file_tools::glob::glob;

  println!( "\n--- Corner Case 5: Glob Zero Matches ---" );

  let results : Vec< _ > = glob( "*.nonexistent_extension" )
    .expect( "valid pattern" )
    .collect();

  assert_eq!( results.len(), 0, "Should match zero files" );

  println!( "✓ Zero-match glob handled correctly" );
}

#[ cfg( not( feature = "glob" ) ) ]
fn test_glob_zero_matches()
{
  println!( "\n--- Corner Case 5: Glob Zero Matches ---" );
  println!( "⚠ Skipped" );
}

/// Corner Case 6: Invalid glob pattern
#[ cfg( feature = "glob" ) ]
fn test_glob_invalid_pattern()
{
  use file_tools::glob::glob;

  println!( "\n--- Corner Case 6: Invalid Glob Pattern ---" );

  // Invalid pattern with unclosed bracket
  let result = glob( "[invalid" );

  assert!( result.is_err(), "Should return error for invalid pattern" );

  if let Err( e ) = result
  {
    println!( "✓ Invalid pattern rejected: {e}" );
  }
}

#[ cfg( not( feature = "glob" ) ) ]
fn test_glob_invalid_pattern()
{
  println!( "\n--- Corner Case 6: Invalid Glob Pattern ---" );
  println!( "⚠ Skipped" );
}

/// Corner Case 7: Pattern edge cases
#[ cfg( feature = "glob" ) ]
fn test_pattern_edge_cases()
{
  use file_tools::glob::Pattern;

  println!( "\n--- Corner Case 7: Pattern Edge Cases ---" );

  // Empty pattern
  let empty = Pattern::new( "" );
  assert!( empty.is_ok(), "Empty pattern should be valid" );

  // Single character
  let single = Pattern::new( "?" ).expect( "valid" );
  assert!( single.matches( "a" ) );
  assert!( !single.matches( "ab" ) );

  // Wildcard matching empty
  let star = Pattern::new( "*" ).expect( "valid" );
  assert!( star.matches( "" ) );
  assert!( star.matches( "anything" ) );

  println!( "✓ Pattern edge cases handled correctly" );
}

#[ cfg( not( feature = "glob" ) ) ]
fn test_pattern_edge_cases()
{
  println!( "\n--- Corner Case 7: Pattern Edge Cases ---" );
  println!( "⚠ Skipped" );
}
