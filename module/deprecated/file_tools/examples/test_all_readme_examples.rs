//! Comprehensive test of ALL examples from readme.md

fn main()
{
  println!( "\n=== Testing All readme.md Examples ===" );

  test_tempdir_raii();
  test_glob_pattern_matching();
  test_recursive_glob();
  test_pattern_matching();

  println!( "\n✓ All readme.md examples completed successfully\n" );
}

/// Example 1: `TempDir` with RAII Cleanup (readme.md lines 23-37)
#[ cfg( feature = "enabled" ) ]
fn test_tempdir_raii()
{
  use file_tools::TempDir;
  use std::path::PathBuf;

  println!( "\n--- Test 1: TempDir with RAII Cleanup ---" );

  let mut temp = TempDir::new();
  temp.base_path = std::env::temp_dir();
  temp.prefix_path = PathBuf::from( "my_app" );
  temp.postfix_path = PathBuf::from( "session_1" );

  // Create directory (enables automatic cleanup)
  let path = temp.create_all().expect( "failed to create" );
  println!( "Created: {}", path.display() );
  assert!( path.is_dir(), "Directory should exist after creation" );

  // Verify the directory exists
  assert!( path.exists(), "Path should exist" );
  println!( "✓ Directory exists and is a directory" );

  // Directory is automatically removed when `temp` goes out of scope
}

#[ cfg( not( feature = "enabled" ) ) ]
fn test_tempdir_raii()
{
  println!( "\n--- Test 1: TempDir with RAII Cleanup ---" );
  println!( "⚠ Skipped (requires 'enabled' feature)" );
}

/// Example 2: Glob Pattern Matching (readme.md lines 41-52)
#[ cfg( feature = "glob" ) ]
fn test_glob_pattern_matching()
{
  use file_tools::glob::glob;

  println!( "\n--- Test 2: Glob Pattern Matching ---" );

  // Find all Rust files in current directory
  let mut found_count = 0;
  for path in glob( "*.rs" ).expect( "valid pattern" ).flatten()
  {
    println!( "Found: {}", path.display() );
    found_count += 1;
  }

  println!( "✓ Found {found_count} .rs files" );
}

#[ cfg( not( feature = "glob" ) ) ]
fn test_glob_pattern_matching()
{
  println!( "\n--- Test 2: Glob Pattern Matching ---" );
  println!( "⚠ Skipped (requires 'glob' feature)" );
}

/// Example 3: Recursive Glob (readme.md lines 56-67)
#[ cfg( feature = "glob" ) ]
fn test_recursive_glob()
{
  use file_tools::glob::glob;

  println!( "\n--- Test 3: Recursive Glob ---" );

  // Find all Rust files recursively
  let mut found_count = 0;
  for path in glob( "src/**/*.rs" ).expect( "valid pattern" ).flatten()
  {
    println!( "Found: {}", path.display() );
    found_count += 1;
  }

  println!( "✓ Found {found_count} .rs files recursively" );
}

#[ cfg( not( feature = "glob" ) ) ]
fn test_recursive_glob()
{
  println!( "\n--- Test 3: Recursive Glob ---" );
  println!( "⚠ Skipped (requires 'glob' feature)" );
}

/// Example 4: Pattern Matching (readme.md lines 71-78)
#[ cfg( feature = "glob" ) ]
fn test_pattern_matching()
{
  use file_tools::glob::Pattern;

  println!( "\n--- Test 4: Pattern Matching ---" );

  let pattern = Pattern::new( "*.rs" ).expect( "valid pattern" );

  assert!( pattern.matches( "lib.rs" ), "Should match lib.rs" );
  assert!( !pattern.matches( "Cargo.toml" ), "Should not match Cargo.toml" );

  println!( "✓ Pattern matching works correctly" );
}

#[ cfg( not( feature = "glob" ) ) ]
fn test_pattern_matching()
{
  println!( "\n--- Test 4: Pattern Matching ---" );
  println!( "⚠ Skipped (requires 'glob' feature)" );
}
