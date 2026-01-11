//! Regression Test: Unix-style Flag Syntax Rejection
//!
//! ## Root Cause
//!
//! Three example files (`03_complex_argument_patterns.rs`, `04_multiple_instructions.rs`,
//! and `unilang_parser_basic.rs`) incorrectly used Unix-style `--flag` syntax (e.g.,
//! `--verbose`, `--dry-run`, `--binary`) which is not supported by the unilang parser.
//! The parser expects `key::value` syntax for all named arguments (e.g., `verbose :: true`).
//!
//! This bug occurred because example authors assumed compatibility with traditional
//! Unix CLI flag conventions without checking the parser specification.
//!
//! ## Why Not Caught
//!
//! The examples were not tested as part of the automated test suite. Manual testing
//! of examples was not part of the development workflow, allowing syntax errors to
//! persist in documentation code.
//!
//! ## Fix Applied
//!
//! Converted all `--flag` syntax instances to proper `key::value` syntax:
//! - `--verbose` → `verbose :: true`
//! - `--dry-run` → `dry_run :: true`
//! - `--binary` → `binary :: true`
//!
//! Updated example logic that checked for flags in positional arguments to instead
//! check named arguments map.
//!
//! ## Prevention
//!
//! This test ensures the parser continues to reject `--flag` syntax with a clear
//! error message. Future example authors will encounter this test failure if they
//! attempt to use unsupported flag syntax.
//!
//! Additionally, example compilation/execution should be added to CI pipeline to
//! catch syntax errors in documentation code.
//!
//! ## Pitfall
//!
//! Unix-style flags are a pervasive CLI convention, making them an attractive
//! but incorrect choice for unilang syntax. The parser's error message "Unexpected
//! token '--flag' in arguments" clearly indicates the syntax is not supported,
//! but example code serves as implicit documentation and must be accurate.
//!
//! Future consideration: If flag-style syntax is a frequent user request, the
//! parser specification could be extended to support it as syntactic sugar for
//! `flag_name :: true`, but this would require specification changes and careful
//! backward compatibility consideration.

use unilang_parser:: { Parser, UnilangParserOptions };

#[ test ]
fn flag_syntax_rejection_verbose()
{
  let parser = Parser ::new( UnilangParserOptions ::default() );

  // Attempt to parse command with --verbose flag
  let result = parser.parse_single_instruction( "command.run --verbose" );

  assert!( result.is_err(), "Parser should reject --verbose flag syntax" );

  let error = result.unwrap_err();
  let error_msg = format!( "{error:?}" );

  assert!(
    error_msg.contains( "--verbose" ) || error_msg.contains( "Unexpected token" ),
    "Error message should mention the problematic token: {error_msg}"
  );
}

#[ test ]
fn flag_syntax_rejection_dry_run()
{
  let parser = Parser ::new( UnilangParserOptions ::default() );

  // Attempt to parse command with --dry-run flag (hyphenated)
  let result = parser.parse_single_instruction( "deploy.staging --dry-run" );

  assert!( result.is_err(), "Parser should reject --dry-run flag syntax" );

  let error = result.unwrap_err();
  let error_msg = format!( "{error:?}" );

  assert!(
    error_msg.contains( "--dry-run" ) || error_msg.contains( "Unexpected token" ),
    "Error message should mention the problematic token: {error_msg}"
  );
}

#[ test ]
fn flag_syntax_rejection_binary()
{
  let parser = Parser ::new( UnilangParserOptions ::default() );

  // Attempt to parse command with --binary flag
  let result = parser.parse_single_instruction( "file.read path :: \"/etc/hosts\" --binary" );

  assert!( result.is_err(), "Parser should reject --binary flag syntax" );

  let error = result.unwrap_err();
  let error_msg = format!( "{error:?}" );

  assert!(
    error_msg.contains( "--binary" ) || error_msg.contains( "Unexpected token" ),
    "Error message should mention the problematic token: {error_msg}"
  );
}

#[ test ]
fn correct_boolean_syntax_works()
{
  let parser = Parser ::new( UnilangParserOptions ::default() );

  // Verify the correct syntax works
  let result = parser.parse_single_instruction( "command.run verbose :: true dry_run :: true" );

  assert!( result.is_ok(), "Parser should accept key::value syntax for boolean flags" );

  let instruction = result.unwrap();

  // Verify named arguments are parsed correctly
  assert!( instruction.named_arguments.contains_key( "verbose" ) );
  assert!( instruction.named_arguments.contains_key( "dry_run" ) );

  assert_eq!( instruction.named_arguments.get( "verbose" ).unwrap()[0].value, "true" );
  assert_eq!( instruction.named_arguments.get( "dry_run" ).unwrap()[0].value, "true" );
}

#[ test ]
fn mixed_correct_and_incorrect_syntax()
{
  let parser = Parser ::new( UnilangParserOptions ::default() );

  // Command with both correct named arg and incorrect flag
  let result = parser.parse_single_instruction( "server.deploy config :: \"/etc/app.conf\" --verbose" );

  assert!( result.is_err(), "Parser should reject command even with one invalid flag" );

  let error = result.unwrap_err();
  let error_msg = format!( "{error:?}" );

  assert!(
    error_msg.contains( "--verbose" ) || error_msg.contains( "Unexpected token" ),
    "Error should clearly identify the problematic flag: {error_msg}"
  );
}
