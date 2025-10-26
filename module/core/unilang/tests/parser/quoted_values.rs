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
  // Shell removes quotes and passes argv: ["query::llm rust"]
  // Parser should parse query as "llm rust" (without the quotes)

  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.args( vec![ ".video.search", "query::llm rust" ] );

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
  // Shell removes quotes, so we pass the values without them
  let test_cases = vec![
    ("query::hello world", "hello world"),
    ("query::multi word query", "multi word query"),
    ("query::rust programming language", "rust programming language"),
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
fn test_multiple_multiword_params()
{
  // Test multiple parameters with multiword values
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.args( vec![ ".video.search", "query::rust language", "title::My Amazing Video" ] );

  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: rust language" ) )
    .stdout( predicate::str::contains( "Title: My Amazing Video" ) );
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
  // Tests correct parsing behavior for separate arguments
  // query::hello world (as separate arguments) should parse as query="hello" and title="world"
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();
  cmd.args( vec![ ".video.search", "query::hello", "title::world" ] );

  // This should correctly parse as separate named arguments
  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: hello" ) )
    .stdout( predicate::str::contains( "Title: world" ) );
}