//! Reproduction test for argv multi-word parameter bug
//!
//! This test demonstrates the critical bug where multi-word parameter values
//! are incorrectly parsed when passed via argv (as from shell command line).
//!
//! ## Bug Description
//!
//! When a user types: `mycli .cmd param::"value with spaces"`
//! Bash removes quotes and passes: `["mycli", ".cmd", "param::value with spaces"]`
//! The parser should treat "value with spaces" as a single parameter value.
//!
//! **Currently:** The parser reconstructs quotes but then re-parses, breaking the value.
//! **Expected:** The parser should preserve the argv token boundaries.

use unilang_parser::{ Parser, UnilangParserOptions };

/// Test Case 1: Single-word parameter (baseline - should work)
#[test]
fn test_argv_single_word_parameter()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .video.search query::rust
  // Bash outputs: [".video.search", "query::rust"]
  let result = parser.parse_from_argv( &[
    ".video.search".to_string(),
    "query::rust".to_string(),
  ]);

  assert!( result.is_ok(), "Single-word parameter should parse successfully" );
  let instruction = result.unwrap();

  // Check command name
  assert_eq!( instruction.command_path_slices.len(), 2 );
  assert_eq!( instruction.command_path_slices[ 0 ], "video" );
  assert_eq!( instruction.command_path_slices[ 1 ], "search" );

  // Check parameter
  let query_values = instruction.named_arguments.get( "query" );
  assert!( query_values.is_some(), "query parameter should exist" );

  let query_args = query_values.unwrap();
  assert_eq!( query_args.len(), 1, "query should have one value" );
  assert_eq!( query_args[ 0 ].value, "rust", "query value should be 'rust'" );
}

/// Test Case 2: Multi-word parameter WITHOUT outer shell quotes (THE BUG)
#[test]
fn test_argv_multiword_parameter_no_shell_quotes()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .video.search query::"llm rust"
  // Bash processing: Removes the double quotes around "llm rust"
  // Bash outputs: [".video.search", "query::llm rust"] ← ONE TOKEN, NO QUOTES IN STRING
  let result = parser.parse_from_argv( &[
    ".video.search".to_string(),
    "query::llm rust".to_string(),  // ← Shell combined this into one token
  ]);

  assert!( result.is_ok(), "Multi-word parameter should parse successfully" );
  let instruction = result.unwrap();

  // Check command name
  assert_eq!( instruction.command_path_slices.len(), 2 );

  // Check parameter - THIS IS WHERE THE BUG MANIFESTS
  let query_values = instruction.named_arguments.get( "query" );
  assert!(
    query_values.is_some(),
    "query parameter should exist (BUG: might be split into multiple params)"
  );

  let query_args = query_values.unwrap();
  assert_eq!(
    query_args.len(),
    1,
    "query should have ONE value (BUG: might have multiple or orphaned tokens)"
  );

  assert_eq!(
    query_args[ 0 ].value,
    "llm rust",
    "query value should be 'llm rust' as a complete string (BUG: might only be 'llm')"
  );
}

/// Test Case 3: Multi-word parameter WITH shell quotes preserved
#[test]
fn test_argv_multiword_parameter_with_shell_quotes_preserved()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .video.search 'query::"llm rust"'
  // Bash processing: Outer single quotes tell bash to preserve inner quotes
  // Bash outputs: [".video.search", 'query::"llm rust"'] ← Quotes PRESERVED
  let result = parser.parse_from_argv( &[
    ".video.search".to_string(),
    "query::\"llm rust\"".to_string(),  // ← Quotes in the string
  ]);

  assert!( result.is_ok(), "Multi-word with preserved quotes should parse" );
  let instruction = result.unwrap();

  let query_values = instruction.named_arguments.get( "query" );
  assert!( query_values.is_some(), "query parameter should exist" );

  let query_args = query_values.unwrap();
  assert_eq!( query_args.len(), 1, "query should have one value" );
  assert_eq!(
    query_args[ 0 ].value,
    "llm rust",
    "query value should be 'llm rust' (quotes stripped)"
  );
}

/// Test Case 4: Multiple parameters, one multi-word
#[test]
fn test_argv_multiple_params_one_multiword()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .video.search query::"llm rust" limit::10
  // Bash outputs: [".video.search", "query::llm rust", "limit::10"]
  let result = parser.parse_from_argv( &[
    ".video.search".to_string(),
    "query::llm rust".to_string(),
    "limit::10".to_string(),
  ]);

  assert!( result.is_ok(), "Multiple params with one multi-word should parse" );
  let instruction = result.unwrap();

  // Check query parameter
  let query_values = instruction.named_arguments.get( "query" );
  assert!( query_values.is_some(), "query parameter should exist" );
  assert_eq!( query_values.unwrap()[ 0 ].value, "llm rust", "query should be 'llm rust'" );

  // Check limit parameter
  let limit_values = instruction.named_arguments.get( "limit" );
  assert!( limit_values.is_some(), "limit parameter should exist" );
  assert_eq!( limit_values.unwrap()[ 0 ].value, "10", "limit should be '10'" );
}

/// Test Case 5: Multi-word split across argv elements (shell removed quotes)
#[test]
fn test_argv_multiword_split_across_elements()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .cmd param::word1 word2
  // Bash outputs: [".cmd", "param::word1", "word2"]
  // Parser should combine "word1" + "word2" until seeing next :: or .
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "param::word1".to_string(),
    "word2".to_string(),
  ]);

  assert!( result.is_ok(), "Split multi-word should parse" );
  let instruction = result.unwrap();

  let param_values = instruction.named_arguments.get( "param" );
  assert!( param_values.is_some(), "param should exist" );
  assert_eq!(
    param_values.unwrap()[ 0 ].value,
    "word1 word2",
    "Parser should combine consecutive tokens into multi-word value"
  );
}

/// Test Case 6: Multi-word with special characters
#[test]
fn test_argv_multiword_with_special_chars()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .cmd path::"/My Documents/file.txt"
  // Bash outputs: [".cmd", "path::/My Documents/file.txt"]
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "path::/My Documents/file.txt".to_string(),
  ]);

  assert!( result.is_ok(), "Path with spaces should parse" );
  let instruction = result.unwrap();

  let path_values = instruction.named_arguments.get( "path" );
  assert!( path_values.is_some(), "path should exist" );
  assert_eq!(
    path_values.unwrap()[ 0 ].value,
    "/My Documents/file.txt",
    "Path should preserve spaces"
  );
}

/// Test Case 7: Empty string value
#[test]
fn test_argv_empty_string_value()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .cmd param::""
  // Bash outputs: [".cmd", "param::"]
  let result = parser.parse_from_argv( &[
    ".cmd".to_string(),
    "param::".to_string(),
  ]);

  assert!( result.is_ok(), "Empty string should parse" );
  let instruction = result.unwrap();

  let param_values = instruction.named_arguments.get( "param" );
  assert!( param_values.is_some(), "param should exist even if empty" );
  assert_eq!( param_values.unwrap()[ 0 ].value, "", "Value should be empty string" );
}

/// Test Case 8: Command with shell command as parameter (real-world case)
#[test]
fn test_argv_shell_command_as_parameter()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: w3 .crates.for.each cmd::"cargo build --release"
  // Bash outputs: [".crates.for.each", "cmd::cargo build --release"]
  let result = parser.parse_from_argv( &[
    ".crates.for.each".to_string(),
    "cmd::cargo build --release".to_string(),
  ]);

  assert!( result.is_ok(), "Shell command as param should parse" );
  let instruction = result.unwrap();

  let cmd_values = instruction.named_arguments.get( "cmd" );
  assert!( cmd_values.is_some(), "cmd parameter should exist" );
  assert_eq!(
    cmd_values.unwrap()[ 0 ].value,
    "cargo build --release",
    "Command with flags should be preserved as single value"
  );
}

/// Test Case 9: Conflict scenario - String param with Integer param having short alias
#[test]
fn test_argv_conflict_string_vs_integer_with_alias()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // This reproduces the exact scenario from the bug report:
  // Command has:
  // - "cmd" parameter (String)
  // - "threads" parameter (Integer, alias "t")
  //
  // User types: .foreach cmd::"echo test"
  // Bash outputs: [".foreach", "cmd::echo test"]
  //
  // BUG: Parser splits "test" as separate token, matches "t" alias to "threads",
  // tries to parse "test" as Integer, fails with "invalid digit"

  let result = parser.parse_from_argv( &[
    ".foreach".to_string(),
    "cmd::echo test".to_string(),  // ← "test" should NOT become separate param
  ]);

  assert!( result.is_ok(), "Multi-word param should not trigger alias matching" );
  let instruction = result.unwrap();

  // Verify only cmd parameter exists
  let cmd_values = instruction.named_arguments.get( "cmd" );
  assert!( cmd_values.is_some(), "cmd parameter should exist" );
  assert_eq!(
    cmd_values.unwrap()[ 0 ].value,
    "echo test",
    "BUG: Parser might split this and match 'test' to 'threads' via alias 't'"
  );

  // Verify no "threads" or "t" parameter was created
  let threads_values = instruction.named_arguments.get( "threads" );
  assert!(
    threads_values.is_none(),
    "BUG: 'threads' param might exist if 'test' was incorrectly matched via alias 't'"
  );

  let t_values = instruction.named_arguments.get( "t" );
  assert!(
    t_values.is_none(),
    "BUG: 't' param might exist if 'test' was incorrectly matched as alias"
  );
}

/// Test Case 10: Value with equals sign (common in env vars)
#[test]
fn test_argv_value_with_equals()
{
  let parser = Parser::new( UnilangParserOptions::default() );

  // Simulates: mycli .env.set var::"PATH=/usr/bin:/bin"
  // Bash outputs: [".env.set", "var::PATH=/usr/bin:/bin"]
  let result = parser.parse_from_argv( &[
    ".env.set".to_string(),
    "var::PATH=/usr/bin:/bin".to_string(),
  ]);

  assert!( result.is_ok(), "Value with equals should parse" );
  let instruction = result.unwrap();

  let var_values = instruction.named_arguments.get( "var" );
  assert!( var_values.is_some() );
  assert_eq!(
    var_values.unwrap()[ 0 ].value,
    "PATH=/usr/bin:/bin",
    "Equals sign in value should be preserved"
  );
}
