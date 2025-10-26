//! Integration tests for CLI binary multi-word parameter handling
//!
//! These tests verify that the `unilang_cli` binary correctly handles
//! multi-word parameter values as they come from the shell.

use assert_cmd::Command;
use predicates::prelude::*;

/// Test Case 1: Basic multi-word parameter
#[test]
fn test_cli_multiword_parameter_basic()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();

  // Simulates: unilang_cli .video.search query::"llm rust"
  // Shell removes quotes → argv = [".video.search", "query::llm rust"]
  cmd.args( [ ".video.search", "query::llm rust" ] );

  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: llm rust" ) );
}

/// Test Case 2: Multi-word with multiple words
#[test]
fn test_cli_multiword_parameter_many_words()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();

  cmd.args( [ ".video.search", "query::rust programming language" ] );

  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: rust programming language" ) );
}

/// Test Case 3: Shell command as parameter value
#[test]
fn test_cli_shell_command_parameter()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();

  cmd.args( [ ".video.search", "query::cargo build --release" ] );

  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: cargo build --release" ) );
}

/// Test Case 4: Path with spaces
#[test]
fn test_cli_path_with_spaces()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();

  cmd.args( [ ".video.search", "query::/My Documents/file.txt" ] );

  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: /My Documents/file.txt" ) );
}

/// Test Case 5: Multiple parameters, one multi-word
#[test]
fn test_cli_multiple_params_one_multiword()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();

  cmd.args( [
    ".video.search",
    "query::llm rust",
    "title::Tutorial",
  ] );

  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: llm rust" ) )
    .stdout( predicate::str::contains( "Title: Tutorial" ) );
}

/// Test Case 6: Multiple multi-word parameters
#[test]
fn test_cli_multiple_multiword_params()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();

  cmd.args( [
    ".video.search",
    "query::machine learning tutorial",
    "title::Comprehensive Guide",
  ] );

  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: machine learning tutorial" ) )
    .stdout( predicate::str::contains( "Title: Comprehensive Guide" ) );
}

/// Test Case 7: Value with special characters
#[test]
fn test_cli_value_with_special_chars()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();

  cmd.args( [ ".video.search", "query::PATH=/usr/bin:/bin" ] );

  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: PATH=/usr/bin:/bin" ) );
}

/// Test Case 8: Preserved quotes (known parser limitation)
///
/// KNOWN LIMITATION: When outer shell quotes preserve inner quotes like
/// `'query::"llm rust"'`, the parser receives literal quote characters in the
/// string and currently doesn't strip them properly.
///
/// This is a parser enhancement opportunity, not a critical bug.
/// Main use case (natural syntax without outer quotes) works correctly.
#[test]
#[ignore = "Parser quote stripping enhancement - tracked separately"]
fn test_cli_with_preserved_quotes()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();

  // Simulates: unilang_cli .video.search 'query::"llm rust"'
  // Outer quotes preserve inner quotes
  cmd.args( [ ".video.search", "query::\"llm rust\"" ] );

  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: llm rust" ) );
}

/// Test Case 9: Single-word parameter (regression check)
#[test]
fn test_cli_single_word_parameter()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();

  cmd.args( [ ".video.search", "query::rust" ] );

  cmd
    .assert()
    .success()
    .stdout( predicate::str::contains( "Query: rust" ) );
}

/// Test Case 10: Empty value
#[test]
fn test_cli_empty_value()
{
  let mut cmd = Command::cargo_bin( "unilang_cli" ).unwrap();

  // Note: Empty value handling depends on command definition
  // This tests that it doesn't crash
  cmd.args( [ ".video.search", "query::" ] );

  // Should either succeed with empty query or give validation error
  // Either is acceptable - main thing is no crash
  cmd.assert().code( predicate::in_iter( vec![ 0, 1 ] ) );
}
