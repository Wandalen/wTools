//! Diagnostic test suite for ISSUE-CMD-PATH
//!
//! ## Root Cause
//!
//! The command path parser (`parse_command_path` at parser_engine.rs:385-404) consumed
//! identifiers without checking if they were followed by `::`, which per spec.md:193
//! indicates the start of a named argument, NOT a command path segment.
//!
//! **Execution Flow (Before Fix):**
//! 1. Input: `"cmd::value"`
//! 2. Tokenizer produces: `["cmd", "::", "value"]` (correct)
//! 3. Command path parser sees `"cmd"` (Identifier)
//! 4. Adds `"cmd"` to `command_path` without lookahead
//! 5. Consumes the token, breaks (no dot follows)
//! 6. Argument parser receives `["::", "value"]`
//! 7. ERROR: "Named argument operator '::' cannot appear by itself"
//!
//! The parser violated spec.md:193 which mandates `::` ends command path parsing.
//!
//! ## Why Not Caught
//!
//! **Test Coverage Gap:** All 100+ existing tests used command paths (`.cmd arg::val`),
//! but zero tests covered named-only arguments without command paths. The spec allows
//! optional command paths (spec.md:173), but this edge case wasnt tested.
//!
//! **Misleading Error:** The error message blamed the tokenizer ("operator cannot appear
//! by itself"), leading investigation to `strs_tools` instead of `unilang_parser`'s command
//! path logic.
//!
//! **Common Usage Pattern:** Most real-world usage includes command paths, so the bug
//! only surfaced with the rare pattern of named-only arguments.
//!
//! ## Fix Applied
//!
//! Added lookahead in command path parser (parser_engine.rs:407-428):
//!
//! ```rust
//! // Clone iterator to look at next item without consuming current
//! let mut lookahead_iter = items_iter.clone();
//! lookahead_iter.next(); // Skip current item
//!
//! if let Some( next_item ) = lookahead_iter.peek() {
//!   let is_named_arg_operator = match &next_item.kind {
//!     UnilangTokenKind::Operator( op ) => *op == "::" || *op == " :: ",
//!     _ => false,
//!   };
//!
//!   if is_named_arg_operator {
//!     break; // Don't consume, let argument parser handle it
//!   }
//! }
//! ```
//!
//! **Pattern Source:** Copied from argument parser (lines 955-963) which already
//! implements this lookahead correctly.
//!
//! ## Prevention
//!
//! **Test Matrix Expansion:** Added 12 comprehensive tests covering:
//! - Named-only args (basic, with spaces, with quotes, empty values)
//! - Multiple named args without command
//! - Command path + named args (regression)
//! - Error detection (truly orphaned `::`)
//! - API consistency (both parse paths)
//!
//! **Spec Rule Coverage:** Every spec rule now has at least one positive test,
//! including rarely-used features like optional command paths.
//!
//! ## Pitfall
//!
//! **Iterator Lookahead Pattern:** The outer loop uses `peek()`, so calling `peek()`
//! again returns the SAME item, not the next one. Must clone the iterator and call
//! `next()` on the clone before peeking to see the truly next token.
//!
//! **Wrong Pattern (Returns current item):**
//! ```rust
//! while let Some(item) = iter.peek() {
//!   if let Some(next) = iter.peek() { } // ❌ Returns 'item' again!
//! }
//! ```
//!
//! **Correct Pattern (Returns next item):**
//! ```rust
//! while let Some(item) = iter.peek() {
//!   let mut lookahead = iter.clone();
//!   lookahead.next(); // Skip current
//!   if let Some(next) = lookahead.peek() { } // ✅ Returns truly next item
//! }
//! ```
//!
//! **Both Operator Variants:** The tokenizer produces TWO variants based on whitespace:
//! `"::"` (no spaces) and `" :: "` (with spaces). The fix MUST check both, as the
//! config (line 37) includes both forms in the operators list.

use unilang_parser::{ Parser, UnilangParserOptions };

/// Test 1: Simple case without quotes (should work if bug fixed)
#[test]
fn diagnostic_simple_named_arg_no_quotes()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( "cmd::value" );

  match &result {
    Ok( inst ) => {
      println!( "✅ Parsed successfully" );
      assert!( inst.command_path_slices.is_empty(), "Command path should be empty" );
      assert!( inst.named_arguments.contains_key( "cmd" ), "Should have 'cmd' named arg" );
    }
    Err( e ) => {
      println!( "❌ Parse failed: {e:?}" );
      panic!( "Should parse successfully after fix" );
    }
  }
}

/// Test 2: With command path AND named argument
#[test]
fn diagnostic_command_plus_named_arg()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( ".test arg::value" );

  match &result {
    Ok( inst ) => {
      println!( "✅ Parsed successfully" );
      assert_eq!( inst.command_path_slices, vec!["test"], "Command path should be 'test'" );
      assert!( inst.named_arguments.contains_key( "arg" ), "Should have 'arg' named argument" );
    }
    Err( e ) => {
      println!( "❌ Parse failed: {e:?}" );
      panic!( "This should work" );
    }
  }
}

/// Test 3: The escaped quotes case (original failing test)
#[test]
fn diagnostic_escaped_quotes_shows_same_bug()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( r#"cmd::"value with \"inner\" quotes""# );

  match &result {
    Ok( inst ) => {
      println!( "✅ Parsed successfully" );
      assert!( inst.command_path_slices.is_empty() );
    }
    Err( e ) => {
      println!( "❌ Parse failed: {e:?}" );
      assert!( format!( "{e:?}" ).contains( "cannot appear by itself" ) );
      panic!( "Should work after fix" );
    }
  }
}

/// Test 4: Prove `strs_tools` works correctly
#[test]
fn diagnostic_strs_tools_is_correct()
{
  use strs_tools::string::split;

  let input = r#"cmd::"value with \"inner\" quotes""#;
  let result: Vec<_> = split::split()
    .src( input )
    .delimeters( &[ "::", " ", "\t", "\n", "\r", "#" ] )
    .quoting( true )
    .preserving_empty( false )
    .perform()
    .collect();

  assert_eq!( result.len(), 3, "Should produce 3 tokens" );
  assert_eq!( result[0].string, "cmd" );
  assert_eq!( result[1].string, "::" );
  assert_eq!( result[2].string, r#"value with "inner" quotes"# );
  println!( "✅ strs_tools correctly handles escaped quotes!" );
}

/// Test 5: Named argument with space-separated operator
// test_kind: bug_reproducer(issue-cmd-path)
#[test]
fn test_named_arg_with_space_operator()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( "arg :: value" );

  assert!( result.is_ok(), "Should parse named arg with space operator" );
  let inst = result.unwrap();
  assert!( inst.command_path_slices.is_empty() );
  assert!( inst.named_arguments.contains_key( "arg" ) );
  assert_eq!( inst.named_arguments.get( "arg" ).unwrap()[0].value, "value" );
}

/// Test 6: Multiple named arguments without command path
// test_kind: bug_reproducer(issue-cmd-path)
#[test]
fn test_multiple_named_args_no_command()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( "arg1::val1 arg2::val2 arg3::val3" );

  assert!( result.is_ok(), "Should parse multiple named args without command" );
  let inst = result.unwrap();
  assert!( inst.command_path_slices.is_empty() );
  assert_eq!( inst.named_arguments.len(), 3 );
  assert_eq!( inst.named_arguments.get( "arg1" ).unwrap()[0].value, "val1" );
  assert_eq!( inst.named_arguments.get( "arg2" ).unwrap()[0].value, "val2" );
  assert_eq!( inst.named_arguments.get( "arg3" ).unwrap()[0].value, "val3" );
}

/// Test 7: Named argument with quoted multi-word value
// test_kind: bug_reproducer(issue-cmd-path)
#[test]
fn test_named_arg_quoted_value()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( r#"path::"my file.txt""# );

  assert!( result.is_ok(), "Should parse named arg with quoted value" );
  let inst = result.unwrap();
  assert!( inst.command_path_slices.is_empty() );
  assert!( inst.named_arguments.contains_key( "path" ) );
  let value = &inst.named_arguments.get( "path" ).unwrap()[0].value;
  assert!( value.contains( "my file.txt" ) );
}

/// Test 8: Named argument with empty value
// test_kind: bug_reproducer(issue-cmd-path)
#[test]
fn test_named_arg_empty_value()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( r#"arg::"""# );

  assert!( result.is_ok(), "Should parse named arg with empty quoted value" );
  let inst = result.unwrap();
  assert!( inst.command_path_slices.is_empty() );
  assert!( inst.named_arguments.contains_key( "arg" ) );
}

/// Test 9: Dotted command path with named argument (regression test)
// test_kind: regression_prevention(issue-cmd-path)
#[test]
fn test_dotted_path_plus_named()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( ".files.copy src::source.txt dst::dest.txt" );

  assert!( result.is_ok(), "Should parse dotted path with named args" );
  let inst = result.unwrap();
  assert_eq!( inst.command_path_slices, vec!["files", "copy"] );
  assert_eq!( inst.named_arguments.len(), 2 );
  assert!( inst.named_arguments.contains_key( "src" ) );
  assert!( inst.named_arguments.contains_key( "dst" ) );
}

/// Test 10: Both API paths produce same result (consistency)
// test_kind: consistency_check(issue-cmd-path)
#[test]
fn test_api_path_consistency()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let result_argv = parser.parse_from_argv( &[ "arg::value".to_string() ] );
  let result_direct = parser.parse_single_instruction( "arg::value" );

  assert!( result_argv.is_ok(), "parse_from_argv should succeed" );
  assert!( result_direct.is_ok(), "parse_single_instruction should succeed" );

  let inst_argv = result_argv.unwrap();
  let inst_direct = result_direct.unwrap();
  assert_eq!( inst_argv.command_path_slices, inst_direct.command_path_slices );
  assert_eq!( inst_argv.named_arguments.len(), inst_direct.named_arguments.len() );
}

/// Test 11: Invalid pattern - orphaned :: should still error
// test_kind: error_detection(issue-cmd-path)
#[test]
fn test_truly_orphaned_operator_errors()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( "::" );

  assert!( result.is_err(), "Truly orphaned :: operator should produce error" );
  if let Err( e ) = result {
    assert!( format!( "{e:?}" ).contains( "cannot appear by itself" ) );
  }
}

/// Test 12: Command path followed by orphaned :: should error
// test_kind: error_detection(issue-cmd-path)
#[test]
fn test_command_path_then_orphaned_operator()
{
  let parser = Parser::new( UnilangParserOptions::default() );
  let result = parser.parse_single_instruction( ".cmd ::" );

  assert!( result.is_err(), "Command path followed by orphaned :: should error" );
}
