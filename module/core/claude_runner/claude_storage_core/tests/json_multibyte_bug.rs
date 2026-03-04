//! Test for JSON parser bug with multi-byte UTF-8 characters
//!
//! ## Bug Reproducer (bug-1)
//!
//! ### Root Cause
//! The JSON parser in `src/json.rs` inconsistently mixes byte indices and character
//! indices:
//! - `self.position` is used as BOTH a byte index (in `consume_str` via slicing)
//!   AND a character index (in `peek()` via `chars().nth()`)
//! - When multi-byte UTF-8 characters are encountered (like em-dash U+2014 which
//!   is 3 bytes), byte position and character position diverge
//! - This causes `chars().nth(position)` to return the wrong character after any
//!   multi-byte character
//!
//! ### Why Not Caught
//! All existing parser tests used ASCII-only JSON. Real Claude Code conversation
//! data contains Unicode characters (em-dash, smart quotes, emoji, etc.) which
//! exposed this bug.
//!
//! ### Fix Applied
//! Modified parser to use byte-oriented indexing consistently:
//! - `peek()`: Use byte slicing + `chars().next()` instead of `chars().nth()`
//! - `advance()`: Advance by actual UTF-8 byte length of character
//! - `skip_whitespace()`: Use byte slicing consistently
//! - `consume_str()`: Already correct (uses byte slicing)
//!
//! ### Prevention
//! - Add test cases with multi-byte UTF-8 characters to parser test suite
//! - Always test with real-world data that contains Unicode
//! - Use byte-oriented indexing consistently when working with UTF-8 strings
//!
//! ### Pitfall
//! **Never mix byte indices and character indices.** Rust strings are UTF-8 byte
//! sequences. `str[index]` is byte-indexed, `chars().nth(n)` is char-indexed.
//! These are DIFFERENT for non-ASCII text. Choose one indexing scheme and use it
//! consistently throughout the parser.

#[test]
fn bug_reproducer_unicode_em_dash()
{
  use claude_storage_core::parse_json;

  // Minimal case: em-dash in string followed by null value
  // Em-dash (—) is U+2014, encoded as 3 bytes in UTF-8: E2 80 94
  let json = r#"{"text":"extension of you—no reference","value":null}"#;

  let result = parse_json( json );
  assert!( result.is_ok(), "Failed to parse JSON with em-dash: {:?}", result.err() );

  let value = result.unwrap();
  let obj = value.as_object().expect( "should be object" );

  assert_eq!( obj.get( "text" ).and_then( | v | v.as_str() ),
    Some( "extension of you—no reference" ) );
}

#[test]
fn bug_reproducer_full_claude_message()
{
  use claude_storage_core::parse_json;

  // Actual failing JSON from Claude Code storage (truncated for test)
  let json = r#"{"text":"I'm an extension of you—no self-reference to AI","stop_reason":null}"#;

  let result = parse_json( json );
  assert!( result.is_ok(), "Parser should handle em-dash correctly: {:?}", result.err() );

  let value = result.unwrap();
  let obj = value.as_object().expect( "should be object" );

  // Verify null value parses correctly after multibyte character
  assert_eq!( obj.get( "stop_reason" ), Some( &claude_storage_core::JsonValue::Null ) );
}

#[test]
fn test_various_unicode_characters()
{
  use claude_storage_core::parse_json;

  // Test various multi-byte UTF-8 characters
  let test_cases =
  [
    r#"{"emoji":"🦀","value":123}"#,  // 4-byte emoji
    r#"{"quote":"He said \"hello\"","done":true}"#,  // Escaped quotes
    r#"{"mixed":"café—résumé","null":null}"#,  // Accents + em-dash
  ];

  for (i, json) in test_cases.iter().enumerate()
  {
    let result = parse_json( json );
    assert!( result.is_ok(), "Test case {} failed: {:?}", i, result.err() );
  }
}
