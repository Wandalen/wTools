//! Test for JSON parser bug with Unicode surrogate pairs
//!
//! ## Bug Reproducer (issue-001)
//!
//! ### Root Cause
//! The JSON parser in `src/json.rs:288-289` does not handle UTF-16 surrogate pairs
//! correctly. When parsing `\uXXXX` escape sequences, it attempts to convert each
//! sequence directly to a `char` using `char::from_u32()`. This fails for high
//! surrogates (U+D800 to U+DBFF) because they are not valid standalone Unicode
//! code points.
//!
//! For characters outside the Basic Multilingual Plane (code points > U+FFFF),
//! JSON represents them as **surrogate pairs** - two consecutive `\uXXXX` sequences:
//! - High surrogate: `\uD800` to `\uDBFF`
//! - Low surrogate: `\uDC00` to `\uDFFF`
//!
//! Example: 😀 (U+1F600) is encoded as `\uD83D\uDE00`
//!
//! The parser needs to:
//! 1. Detect high surrogate (D800-DBFF)
//! 2. Read next `\uXXXX` sequence (must be low surrogate DC00-DFFF)
//! 3. Combine: `codepoint = 0x10000 + ((high & 0x3FF) << 10) + (low & 0x3FF)`
//! 4. Convert combined code point to `char`
//!
//! ### Why Not Caught
//! All existing JSON parser tests used either:
//! - ASCII-only JSON strings
//! - Raw Unicode characters (not JSON escape sequences)
//!
//! Real Claude Code conversation data contains emojis and symbols that are
//! JSON-encoded as surrogate pairs, exposing this bug.
//!
//! Existing test in `json_multibyte_bug.rs` tests raw emoji `"🦀"` but not the
//! JSON-escaped form `"\uD83E\uDD80"`.
//!
//! ### Fix Applied
//! Modified `parse_unicode_escape()` in `src/json.rs` to:
//! 1. Detect high surrogates (D800-DBFF)
//! 2. Check if followed by `\uXXXX` sequence with low surrogate (DC00-DFFF)
//! 3. If properly paired, combine using formula: `0x10000 + ((high & 0x3FF) << 10) + (low & 0x3FF)`
//! 4. If unpaired (not followed by low surrogate), replace with U+FFFD (replacement character)
//! 5. Also handle unpaired low surrogates (DC00-DFFF) by replacing with U+FFFD
//!
//! This lenient approach matches Python's json module behavior and allows parsing real-world
//! JSON that may contain invalid surrogates, preventing data loss on actual user files
//!
//! ### Prevention
//! - Add test cases with JSON escape sequences for non-BMP characters
//! - Test with actual JSON-encoded emojis (surrogate pairs)
//! - Test surrogate pair edge cases (properly paired, unpaired high, unpaired low)
//! - Test unpaired surrogates mixed with other content (real-world case)
//! - Run parser against real Claude Code session files with varied Unicode content
//!
//! ### Pitfall
//! **UTF-16 surrogates in JSON:** Properly paired surrogates (high D800-DBFF followed by
//! low DC00-DFFF) represent valid non-BMP characters. However, unpaired surrogates are
//! technically invalid but appear in real-world JSON. **Design decision:** Reject unpaired
//! surrogates (strict, spec-compliant) vs. replace with U+FFFD (lenient, compatible with
//! Python/real data). We chose lenient to avoid data loss. Always handle both cases: check
//! if high surrogate is followed by low surrogate, and have fallback for unpaired case.

// test_kind: bug_reproducer(issue-001)
#[test]
fn test_surrogate_pair_emoji_grinning_face()
{
  use claude_storage_core::parse_json;

  // Minimal case: JSON with emoji as surrogate pair
  // 😀 (GRINNING FACE) = U+1F600 = \uD83D\uDE00
  let json = r#"{"text":"\uD83D\uDE00"}"#;

  let result = parse_json( json );
  assert!(
    result.is_ok(),
    "Failed to parse JSON with surrogate pair: {:?}",
    result.err()
  );

  let value = result.unwrap();
  let obj = value.as_object().expect( "should be object" );

  assert_eq!(
    obj.get( "text" ).and_then( | v | v.as_str() ),
    Some( "😀" ),
    "Surrogate pair should decode to correct emoji"
  );
}

// test_kind: bug_reproducer(issue-001)
#[test]
fn test_surrogate_pair_multiple_emojis()
{
  use claude_storage_core::parse_json;

  // Multiple emojis in one string
  // 😀 = \uD83D\uDE00
  // 🎉 = \uD83C\uDF89
  let json = r#"{"emojis":"\uD83D\uDE00 test \uD83C\uDF89"}"#;

  let result = parse_json( json );
  assert!(
    result.is_ok(),
    "Failed to parse multiple surrogate pairs: {:?}",
    result.err()
  );

  let value = result.unwrap();
  let obj = value.as_object().expect( "should be object" );

  assert_eq!(
    obj.get( "emojis" ).and_then( | v | v.as_str() ),
    Some( "😀 test 🎉" ),
    "Multiple surrogate pairs should decode correctly"
  );
}

// test_kind: bug_reproducer(issue-001)
#[test]
fn test_surrogate_pair_mixed_with_regular_escapes()
{
  use claude_storage_core::parse_json;

  // Mix of regular escapes and surrogate pairs
  // \n = newline
  // 😀 = \uD83D\uDE00
  // \t = tab
  let json = r#"{"text":"Line1\nEmoji: \uD83D\uDE00\tDone"}"#;

  let result = parse_json( json );
  assert!(
    result.is_ok(),
    "Failed to parse mixed escapes and surrogate pairs: {:?}",
    result.err()
  );

  let value = result.unwrap();
  let obj = value.as_object().expect( "should be object" );

  assert_eq!(
    obj.get( "text" ).and_then( | v | v.as_str() ),
    Some( "Line1\nEmoji: 😀\tDone" ),
    "Mixed escapes should work correctly"
  );
}

// test_kind: bug_reproducer(issue-001)
#[test]
fn test_surrogate_pair_mathematical_symbols()
{
  use claude_storage_core::parse_json;

  // Mathematical Alphanumeric Symbols (outside BMP)
  // 𝕳 (MATHEMATICAL DOUBLE-STRUCK CAPITAL H) = U+1D573 = \uD835\uDD73
  let json = r#"{"symbol":"\uD835\uDD73"}"#;

  let result = parse_json( json );
  assert!(
    result.is_ok(),
    "Failed to parse mathematical symbol surrogate pair: {:?}",
    result.err()
  );

  let value = result.unwrap();
  let obj = value.as_object().expect( "should be object" );

  assert_eq!(
    obj.get( "symbol" ).and_then( | v | v.as_str() ),
    Some( "𝕳" ),
    "Mathematical symbol surrogate pair should decode correctly"
  );
}

#[test]
fn test_real_claude_session_with_emoji()
{
  use claude_storage_core::parse_json;

  // Simplified version of actual Claude Code session entry with emoji
  let json = r#"{"message":{"role":"user","content":[{"type":"text","text":"Great work! \uD83C\uDF89"}]}}"#;

  let result = parse_json( json );
  assert!(
    result.is_ok(),
    "Failed to parse real Claude session with emoji: {:?}",
    result.err()
  );
}

// test_kind: bug_reproducer(issue-001)
#[test]
fn test_unpaired_high_surrogate()
{
  use claude_storage_core::parse_json;

  // Unpaired high surrogate (not followed by low surrogate)
  // Real-world data may contain this - should replace with U+FFFD
  let json = r#"{"text":"\uD83D"}"#;

  let result = parse_json( json );
  assert!(
    result.is_ok(),
    "Should parse unpaired high surrogate: {:?}",
    result.err()
  );

  let value = result.unwrap();
  let obj = value.as_object().expect( "should be object" );

  // Should be replaced with replacement character
  assert_eq!(
    obj.get( "text" ).and_then( | v | v.as_str() ),
    Some( "\u{FFFD}" ),
    "Unpaired high surrogate should become replacement character"
  );
}

// test_kind: bug_reproducer(issue-001)
#[test]
fn test_unpaired_low_surrogate()
{
  use claude_storage_core::parse_json;

  // Unpaired low surrogate (without preceding high surrogate)
  let json = r#"{"text":"\uDE00"}"#;

  let result = parse_json( json );
  assert!(
    result.is_ok(),
    "Should parse unpaired low surrogate: {:?}",
    result.err()
  );

  let value = result.unwrap();
  let obj = value.as_object().expect( "should be object" );

  assert_eq!(
    obj.get( "text" ).and_then( | v | v.as_str() ),
    Some( "\u{FFFD}" ),
    "Unpaired low surrogate should become replacement character"
  );
}

// test_kind: bug_reproducer(issue-001)
#[test]
fn test_unpaired_surrogate_with_other_text()
{
  use claude_storage_core::parse_json;

  // Unpaired high surrogate followed by newline (real case from Claude Code data)
  let json = r#"{"text":"before\uD83D\nafter"}"#;

  let result = parse_json( json );
  assert!(
    result.is_ok(),
    "Should parse unpaired surrogate with other escapes: {:?}",
    result.err()
  );

  let value = result.unwrap();
  let obj = value.as_object().expect( "should be object" );

  assert_eq!(
    obj.get( "text" ).and_then( | v | v.as_str() ),
    Some( "before�\nafter" ),
    "Should handle unpaired surrogate in mixed content"
  );
}
