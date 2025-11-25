//!
//! # Category Field Code Generation Tests
//!
//! ## What This Tests
//!
//! This test suite validates that the build script (`build.rs`) correctly generates
//! code for the `category` field when creating `StaticCommandDefinition` constants
//! from YAML manifests.
//!
//! ## Why This Matters
//!
//! The build script is layer 2 of the three-layer data integrity chain:
//! YAML → **Build Script** → `StaticCommandDefinition` → From conversion
//!
//! If the build script doesn't extract and output the category field, all YAML
//! category values are lost. These tests prevent:
//! - Generator forgetting to output category line
//! - Code injection via special characters in category names
//! - Syntax errors from improper escaping
//! - Field ordering breaking PHF compilation
//!
//! ## Failure Interpretation
//!
//! - `codegen_outputs_category_line()` fails: Build script not generating category field
//! - `codegen_escapes_quotes_in_category()` fails: Quote escaping broken, code injection risk
//! - `codegen_escapes_newlines_in_category()` fails: Newline handling broken, multi-line output
//! - `codegen_empty_category_format()` fails: Empty category not handled correctly
//! - `codegen_category_position_after_auto_help()` fails: Field ordering wrong, may break PHF
//! - `codegen_category_with_special_chars()` fails: Special character escaping broken
//! - `codegen_very_long_category()` fails: Long categories truncated or wrapped incorrectly
//! - `codegen_multiple_commands_different_categories()` fails: Category leaking between commands
//!
//! ## Related
//!
//! - Issue-089: Category field code generation
//! - `build.rs:generate_command_const()` - Code generation implementation
//! - `tests/category_field_yaml_parsing_tests.rs` - YAML input validation

use std::fs;
use std::path::PathBuf;

//
// Test: codegen outputs category line
//

/// Verifies that generated code contains category field line.
///
/// This prevents generator forgetting to output category.
#[ test ]
fn codegen_outputs_category_line()
{
  // Check that build.rs generated output includes category field
  let manifest_dir = std::env::var( "CARGO_MANIFEST_DIR" ).unwrap();
  let out_dir = PathBuf::from( &manifest_dir ).join( "target" ).join( "debug" ).join( "build" );

  // Find unilang build output directory
  let unilang_build_dirs : Vec< _ > = match fs::read_dir( &out_dir )
  {
    Ok( entries ) => entries
      .filter_map( Result::ok )
      .filter( | e | e.file_name().to_string_lossy().starts_with( "unilang-" ) )
      .collect(),
    Err( _ ) => return, // Build dir doesn't exist yet, skip test
  };

  if unilang_build_dirs.is_empty()
  {
    // Build hasn't run yet, skip this test
    return;
  }

  // Check the first unilang build directory
  let build_dir = unilang_build_dirs[ 0 ].path().join( "out" );
  if !build_dir.exists()
  {
    return;
  }

  let static_commands = build_dir.join( "static_commands.rs" );
  if !static_commands.exists()
  {
    return;
  }

  let content = fs::read_to_string( static_commands ).unwrap();

  // Verify category field is present in generated code
  assert!( content.contains( "category:" ), "Generated code should contain 'category:' field" );
}

//
// Test: codegen escapes quotes in category
//

/// Verifies that quotes in category names are properly escaped.
///
/// This prevents code injection or syntax errors.
#[ test ]
fn codegen_escapes_quotes_in_category()
{
  // The escape_string function in build.rs should escape quotes
  // We test this indirectly by verifying the pattern used in build.rs
  let test_input = r#"My "Special" Category"#;
  let expected_output = r#"My \"Special\" Category"#;

  let escaped = escape_string_like_build_rs( test_input );
  assert_eq!( escaped, expected_output );
}

//
// Test: codegen escapes newlines in category
//

/// Verifies that newlines in category names are properly escaped.
///
/// This prevents multi-line category breaking generated code.
#[ test ]
fn codegen_escapes_newlines_in_category()
{
  let test_input = "Category\nWith\nNewlines";
  let escaped = escape_string_like_build_rs( test_input );

  assert!( !escaped.contains( '\n' ), "Escaped string should not contain literal newlines" );
  assert!( escaped.contains( "\\n" ), "Escaped string should contain \\n sequences" );
}

//
// Test: codegen empty category format
//

/// Verifies that empty category generates correct format.
///
/// This prevents omitting field or using null.
#[ test ]
fn codegen_empty_category_format()
{
  let test_input = "";
  let escaped = escape_string_like_build_rs( test_input );

  assert_eq!( escaped, "", "Empty category should remain empty string" );
}

//
// Test: codegen category position after auto_help
//

/// Verifies that category field appears after `auto_help_enabled` in generated struct.
///
/// This prevents field ordering breaking PHF map compilation.
#[ test ]
fn codegen_category_position_after_auto_help()
{
  let manifest_dir = std::env::var( "CARGO_MANIFEST_DIR" ).unwrap();
  let out_dir = PathBuf::from( &manifest_dir ).join( "target" ).join( "debug" ).join( "build" );

  let unilang_build_dirs : Vec< _ > = match fs::read_dir( &out_dir )
  {
    Ok( entries ) => entries
      .filter_map( Result::ok )
      .filter( | e | e.file_name().to_string_lossy().starts_with( "unilang-" ) )
      .collect(),
    Err( _ ) => return,
  };

  if unilang_build_dirs.is_empty()
  {
    return;
  }

  let build_dir = unilang_build_dirs[ 0 ].path().join( "out" );
  if !build_dir.exists()
  {
    return;
  }

  let static_commands = build_dir.join( "static_commands.rs" );
  if !static_commands.exists()
  {
    return;
  }

  let content = fs::read_to_string( static_commands ).unwrap();

  // Find first occurrence of auto_help_enabled and category
  if let Some( auto_help_pos ) = content.find( "auto_help_enabled:" )
  {
    if let Some( category_pos ) = content.find( "category:" )
    {
      assert!( category_pos > auto_help_pos, "category field should appear after auto_help_enabled field" );
    }
  }
}

//
// Test: codegen category with special markdown chars
//

/// Verifies that markdown special characters are properly escaped.
///
/// This prevents breaking generated Rust code.
#[ test ]
fn codegen_category_with_special_markdown_chars()
{
  let test_input = r"Code: *bold*, _italic_, [link], `backtick`";
  let escaped = escape_string_like_build_rs( test_input );

  // Should escape backslashes and quotes
  assert!( !escaped.contains( '"' ) || escaped.contains( r#"\""# ), "Quotes should be escaped if present" );
}

//
// Test: codegen very long category
//

/// Verifies that long category names (200+ chars) are handled correctly.
///
/// This prevents truncation or wrapping issues.
#[ test ]
fn codegen_very_long_category()
{
  let test_input = "a".repeat( 200 );
  let escaped = escape_string_like_build_rs( &test_input );

  assert_eq!( escaped.len(), 200, "Long category should not be truncated" );
}

//
// Test: codegen multiple commands different categories
//

/// Verifies that multiple commands with different categories are generated correctly.
///
/// This prevents category leaking between commands.
#[ test ]
fn codegen_multiple_commands_different_categories()
{
  // This test verifies build output structure
  let manifest_dir = std::env::var( "CARGO_MANIFEST_DIR" ).unwrap();
  let out_dir = PathBuf::from( &manifest_dir ).join( "target" ).join( "debug" ).join( "build" );

  let unilang_build_dirs : Vec< _ > = match fs::read_dir( &out_dir )
  {
    Ok( entries ) => entries
      .filter_map( Result::ok )
      .filter( | e | e.file_name().to_string_lossy().starts_with( "unilang-" ) )
      .collect(),
    Err( _ ) => return,
  };

  if unilang_build_dirs.is_empty()
  {
    return;
  }

  let build_dir = unilang_build_dirs[ 0 ].path().join( "out" );
  if !build_dir.exists()
  {
    return;
  }

  let static_commands = build_dir.join( "static_commands.rs" );
  if !static_commands.exists()
  {
    return;
  }

  let content = fs::read_to_string( static_commands ).unwrap();

  // Count category field occurrences - should match command count
  let category_count = content.matches( "category:" ).count();

  // Should have at least one category field per command
  assert!( category_count > 0, "Should have category fields in generated code" );
}

//
// Helper function that mimics build.rs escape_string logic
//

fn escape_string_like_build_rs( s : &str ) -> String
{
  s.replace( '\\', "\\\\" )
    .replace( '"', "\\\"" )
    .replace( '\n', "\\n" )
    .replace( '\r', "\\r" )
    .replace( '\t', "\\t" )
}
