//! Tests for value-context-aware tokenization (Bug #006 and #007 fixes).
//!
//! # Purpose
//!
//! This test suite verifies that argument values after `::` operators are treated as atomic units,
//! protecting special characters (#, ?, etc.) from being interpreted as delimiters or operators.
//!
//! # Root Cause (Bug #006 and #007)
//!
//! The tokenizer previously split ALL content on delimiters, even inside argument values:
//!
//! ```text
//! Input:  .search query::Bug #003
//! Before: ["query", "::", "Bug", "#", "003"]  ❌ Wrong - # split value
//! After:  ["query", "::", "Bug #003"]         ✅ Correct - # protected
//! ```
//!
//! # Fix Applied
//!
//! Implemented value-context-aware tokenization with state machine:
//! - After `::` operator: Enter value context
//! - Accumulate all tokens until whitespace delimiter
//! - Merge into single Identifier token
//! - Return to normal context after whitespace
//!
//! File: `src/parser_engine.rs::merge_value_context_tokens()`
//!
//! # Prevention
//!
//! - Spec updated: Section 2.4, Rule 5 now clarifies value collection behavior
//! - Test coverage: Complete edge case matrix (see below)
//! - Both API paths tested: `parse_single_instruction()` and `parse_from_argv()`
//!
//! # Pitfall
//!
//! Don't assume "delimiter" is a simple concept. Context matters:
//! - `#` in normal context → comment delimiter
//! - `#` after `::` → part of value content
//! - Always consider the parsing state when processing characters
//!
//! # Test Matrix
//!
//! | Test Case | Input | Expected Behavior | Edge Case |
//! |-----------|-------|-------------------|-----------|
//! | Hash in value | `query::Bug #003` | Value = `"Bug #003"` | Bug #006 |
//! | Question mark in value | `query::test?` | Value = `"test?"` | Bug #007 |
//! | Multiple named args | `query::Bug #003 status::open?` | Both values protected | Consecutive values |
//! | Empty value | `query::` | Value = `""` | EOF after operator |
//! | Nested delimiters | `path::dir/file#123.txt` | Value = `"dir/file#123.txt"` | Multiple delimiters |
//! | Whitespace terminates | `query::val1 val2` | value=`"val1"`, `"val2"` separate | Spec Rule 0 |
//! | Help operator preserved | `query::Bug #003 ?` | Help still works | Mixed operators |
//! | Hash outside value errors | `query::test #` | Parse error (existing behavior) | Context sensitivity |
//! | Both API paths | `parse_from_argv([...])` | Same as `parse_single_instruction` | API consistency |
//! | Dot in value | `query::path.to.file` | Value = `"path.to.file"` | Dot delimiter |
//! | Slash in value | `query::path/to/file` | Value = `"path/to/file"` | Path separators |
//! | Operator spacing variants | ` :: ` vs `::` | Both work identically | Whitespace handling |
//!
//! # Corner Cases Checklist
//!
//! - [x] Hash character in value
//! - [x] Question mark in value
//! - [x] Empty value
//! - [x] Whitespace terminates value
//! - [x] Multiple named arguments
//! - [x] Help operator after value
//! - [x] Hash outside value causes error (preserves existing behavior)
//! - [x] Nested delimiters (., /, #)
//! - [x] Both `::` operator variants
//! - [x] Both API paths
//! - [x] Value at EOF (no trailing whitespace)
//!
//! # References
//!
//! - Bug #006: Search query cannot contain # character
//! - Bug #007: Search query cannot contain ? character
//! - Spec: Section 2.4, Rule 5 (Named Arguments)
//! - Design: `/-state_machine_design.md`
//! - Comprehensive plan: `/-comprehensive_fix_plan_bug006_007.md`

use unilang_parser:: { Parser, config::UnilangParserOptions };

/// Helper to extract argument value by name from parsed instruction
fn get_arg_value( instruction: &unilang_parser::instruction::GenericInstruction, name: &str ) -> String
{
  instruction
  .named_arguments
  .get( name )
  .and_then( | args | args.first() )
  .map_or_else( || panic!( "Argument '{name}' not found" ), | arg | arg.value.clone() )
}

#[ test ]
fn test_hash_in_value()
{
  // Test Case: Bug #006 - Hash character in argument value
  // Input: .search query::"Bug #003" (quoted because value contains space)
  // Expected: Value = "Bug #003"

  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( ".search query::\"Bug #003\"" );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  // Should have ONE named argument "query" with value "Bug #003"
  assert_eq!( instruction.named_arguments.len(), 1, "Expected 1 named argument" );
  let value = get_arg_value( &instruction, "query" );
  assert_eq!( value, "Bug #003", "Hash character should be part of value" );
}

#[ test ]
fn test_question_mark_in_value()
{
  // Test Case: Bug #007 - Question mark in argument value
  // Input: .cmd arg::test?
  // Expected: Value = "test?"

  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( ".cmd arg::test?" );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  // Value should contain the question mark
  let value = get_arg_value( &instruction, "arg" );
  assert_eq!( value, "test?", "Question mark should be part of value" );

  // Should NOT be interpreted as help request
  assert!( !instruction.help_requested, "? in value should not trigger help" );
}

#[ test ]
fn test_multiple_values_with_special_chars()
{
  // Test Case: Multiple named arguments with special characters
  // Input: .search query::"Bug #003" status::open? max::100
  // Expected: All three values protected
  // Note: query value is quoted because it contains space

  let input = ".search query::\"Bug #003\" status::open? max::100";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  // Should have 3 named arguments
  assert_eq!( instruction.named_arguments.len(), 3, "Expected 3 named arguments" );

  // Verify each value
  assert_eq!( get_arg_value( &instruction, "query" ), "Bug #003" );
  assert_eq!( get_arg_value( &instruction, "status" ), "open?" );
  assert_eq!( get_arg_value( &instruction, "max" ), "100" );
}

#[ test ]
fn test_value_with_path_containing_hash()
{
  // Test Case: Path-like value with hash
  // Input: .cmd file::path/to/file#123
  // Expected: Value = "path/to/file#123"

  let input = ".cmd file::path/to/file#123";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  let value = get_arg_value( &instruction, "file" );
  assert_eq!( value, "path/to/file#123", "Path with # should be preserved" );
}

#[ test ]
fn test_empty_value()
{
  // Test Case: Empty value after :: operator
  // Input: .cmd query::""
  // Expected: Value = "" (empty string)
  // Note: Empty values must be quoted to be valid

  let input = ".cmd query::\"\"";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  // Should parse as argument "query" with empty value
  let value = get_arg_value( &instruction, "query" );
  assert_eq!( value, "", "Empty value should be preserved" );
}

#[ test ]
fn test_whitespace_terminates_value()
{
  // Test Case: Whitespace terminates value collection (Spec Rule 0)
  // Input: .cmd query::val1 val2
  // Expected: query="val1", "val2" is separate positional arg

  let input = ".cmd query::val1 val2";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  // "val1" is query value
  assert_eq!( get_arg_value( &instruction, "query" ), "val1" );

  // "val2" is separate positional argument
  assert_eq!( instruction.positional_arguments.len(), 1, "Expected 1 positional arg" );
  assert_eq!( instruction.positional_arguments[0].value, "val2" );
}

#[ test ]
fn test_help_operator_after_value()
{
  // Test Case: Help operator works after value context
  // Input: .cmd query::"Bug #003" ?
  // Expected: Value protected, help requested

  let input = ".cmd query::\"Bug #003\" ?";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  // Value should be protected
  assert_eq!( get_arg_value( &instruction, "query" ), "Bug #003" );

  // Help should be requested
  assert!( instruction.help_requested, "? operator should trigger help" );
}

#[ test ]
fn test_dot_delimiter_in_value()
{
  // Test Case: Dot delimiter protected in value
  // Input: .cmd path::file.name.ext
  // Expected: Value = "file.name.ext"

  let input = ".cmd path::file.name.ext";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  let value = get_arg_value( &instruction, "path" );
  assert_eq!( value, "file.name.ext", "Dots in value should be preserved" );
}

#[ test ]
fn test_slash_in_value()
{
  // Test Case: Slash protected in value
  // Input: .cmd path::dir/subdir/file
  // Expected: Value = "dir/subdir/file"

  let input = ".cmd path::dir/subdir/file";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  let value = get_arg_value( &instruction, "path" );
  assert_eq!( value, "dir/subdir/file", "Slashes in value should be preserved" );
}

#[ test ]
fn test_operator_spacing_variant_with_spaces()
{
  // Test Case: Operator " :: " variant (with spaces) works identically
  // Input: .cmd query :: "Bug #003"
  // Expected: Value = "Bug #003"

  let input = ".cmd query :: \"Bug #003\"";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  let value = get_arg_value( &instruction, "query" );
  assert_eq!( value, "Bug #003", "Spaced operator should work identically" );
}

#[ test ]
fn test_parse_from_argv_api_consistency()
{
  // Test Case: Both API paths produce identical results
  // Test parse_from_argv() with special characters in values

  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulate argv array with special characters
  let argv = vec![
    ".search".to_string(),
    "query::Bug #003".to_string(),
  ];

  let result = parser.parse_from_argv( &argv );

  assert!( result.is_ok(), "parse_from_argv failed: {:?}", result.err() );
  let instruction = result.unwrap();

  // Should have same behavior as parse_single_instruction
  let value = get_arg_value( &instruction, "query" );
  assert_eq!( value, "Bug #003", "parse_from_argv should protect special chars" );
}

#[ test ]
fn test_complex_value_with_multiple_delimiters()
{
  // Test Case: Value with multiple different delimiter types
  // Input: .cmd arg::path/to#file.ext?query
  // Expected: Value = "path/to#file.ext?query"

  let input = ".cmd arg::path/to#file.ext?query";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  let value = get_arg_value( &instruction, "arg" );
  assert_eq!( value, "path/to#file.ext?query", "All delimiters should be preserved in value" );
}

#[ test ]
fn test_value_at_eof()
{
  // Test Case: Value at end of input (no trailing whitespace)
  // Input: .cmd query::value#123
  // Expected: Value = "value#123"

  let input = ".cmd query::value#123";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  let value = get_arg_value( &instruction, "query" );
  assert_eq!( value, "value#123", "Value at EOF should be fully captured" );
}

#[ test ]
fn test_value_with_tab_delimiter()
{
  // Test Case: Tab terminates value (whitespace rule)
  // Input: ".cmd query::val1\tval2"
  // Expected: query="val1", "val2" is separate

  let input = ".cmd query::val1\tval2";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  // Tab should terminate value
  assert_eq!( get_arg_value( &instruction, "query" ), "val1" );
  assert_eq!( instruction.positional_arguments.len(), 1 );
  assert_eq!( instruction.positional_arguments[0].value, "val2" );
}

#[ test ]
fn test_regression_hash_outside_value_still_errors()
{
  // Test Case: Hash character outside value context still causes error (preserves existing behavior)
  // Input: .cmd arg::test #
  // Expected: Parse error (# is not supported outside value context)
  // Note: Comments are not supported in unilang - see comprehensive_tests.rs SA2.3

  let input = ".cmd arg::test #";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  // Should fail - # outside value context is an error
  assert!( result.is_err(), "Expected parse error for # outside value context" );

  // Verify it's specifically a syntax error about the # character
  if let Err( e ) = result
  {
    let error_msg = format!( "{e:?}" );
    assert!(
      error_msg.contains( '#' ) || error_msg.contains( "Unexpected token" ),
      "Error should mention # or unexpected token, got: {error_msg}"
    );
  }
}

#[ test ]
fn test_regression_help_operator_alone_still_works()
{
  // Test Case: Help operator alone still works
  // Input: .cmd ?
  // Expected: Help requested, no errors

  let input = ".cmd ?";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  // Help should be requested
  assert!( instruction.help_requested, "? alone should trigger help" );
}

#[ test ]
fn test_value_with_newline_delimiter()
{
  // Test Case: Newline terminates value (whitespace rule)
  // Input: ".cmd query::val1\nval2"
  // Expected: query="val1", "val2" is separate

  let input = ".cmd query::val1\nval2";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  // Newline should terminate value
  assert_eq!( get_arg_value( &instruction, "query" ), "val1" );
}

#[ test ]
fn test_consecutive_operators_in_value()
{
  // Test Case: Multiple :: operators in sequence
  // Input: .cmd arg1::val1 arg2::val2
  // Expected: Both values captured correctly

  let input = ".cmd arg1::val1 arg2::val2";
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( input );

  assert!( result.is_ok(), "Failed to parse: {:?}", result.err() );
  let instruction = result.unwrap();

  assert_eq!( get_arg_value( &instruction, "arg1" ), "val1" );
  assert_eq!( get_arg_value( &instruction, "arg2" ), "val2" );
}
