//! Integration tests for quoted value parsing in CLI arguments.
//!
//! This module tests the critical parsing issue where quoted multi-word values
//! are not handled correctly without outer shell quotes.
//!
//! Problem: `query::"llm rust"` should parse as a single argument with value "llm rust"
//! Currently fails and needs to be wrapped in additional quotes.

use assert_cmd::Command;
use predicates::prelude::*;

#[ test ]
fn test_quoted_multiword_value_parsing_reproduction()
{
  // This test reproduces the critical parsing issue
  // The command: .video.search query::"llm rust"
  // Should parse query as "llm rust" (without the quotes)
  // Currently this fails because the parser treats it as:
  // - query::llm (first argument)
  // - rust (second argument, incorrectly parsed)

  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.args( vec![ ".video.search", r#"query::"llm rust""# ] );

  // The command should succeed and process the query as "llm rust"
  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: llm rust" ) );
}

#[ test ]
fn test_quoted_multiword_value_with_various_quotes()
{
  // Test different quote scenarios that should all work
  let test_cases = vec![
    (r#"query::"hello world""#, "hello world"),
    (r#"query::"multi word query""#, "multi word query"),
    (r#"query::"rust programming language""#, "rust programming language"),
    (r#"query::rust title::"My Amazing Video""#, "My Amazing Video"),
  ];

  for (input_arg, expected_value) in test_cases {
    let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
    cmd.args( vec![ ".video.search", input_arg ] );

    cmd
      .assert()
      .success()
      .stdout( predicate::str::contains( expected_value ) );
  }
}

#[ test ]
fn test_single_word_values_still_work()
{
  // Ensure we don't break existing single-word value parsing
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.args( vec![ ".video.search", "query::rust" ] );

  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: rust" ) );
}

#[ test ]
fn test_unquoted_multiword_handling()
{
  // This now works with the fix for unquoted multi-word parsing
  // query::hello world (without quotes) should be parsed as "hello world"
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.args( vec![ ".video.search", "query::hello", "world" ] );

  // This should now work correctly and parse as "hello world"
  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: hello world" ) );
}