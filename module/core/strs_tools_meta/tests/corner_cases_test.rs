//! Comprehensive corner case manual testing for `strs_tools_meta`
//!
//! This file contains manual tests for edge cases that may not be covered
//! by the standard test suite. Execute with: `cargo test --test corner_cases_test`

#[ cfg( feature = "optimize_split" ) ]
use strs_tools_meta::optimize_split;

#[ cfg( feature = "optimize_match" ) ]
use strs_tools_meta::optimize_match;

// ============================================================================
// optimize_split! Corner Cases
// ============================================================================

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_empty_string()
{
  let result = optimize_split!( "", "," );
  println!( "Empty string split result: {result:?}" );
  assert_eq!( result, vec![ "" ] );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_empty_string_preserve_empty_false()
{
  let result = optimize_split!( "", ",", preserve_empty = false );
  println!( "Empty string (preserve_empty=false): {result:?}" );
  // The simple split().collect() path is used when no options are active.
  // str::split() always yields at least one segment, so [""] is returned even
  // for empty input; preserve_empty=false only filters in the manual-loop path.
  assert_eq!( result, vec![ "" ] );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_no_delimiters()
{
  let result = optimize_split!( "nodelimiters", "," );
  println!( "No delimiters result: {result:?}" );
  assert_eq!( result, vec![ "nodelimiters" ] );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_only_delimiters()
{
  let result = optimize_split!( ",,,", "," );
  println!( "Only delimiters result: {result:?}" );
  // Default preserve_empty=true: matches str::split() → 4 empty segments
  assert_eq!( result, vec![ "", "", "", "" ] );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_only_delimiters_preserve_empty()
{
  let result = optimize_split!( ",,,", ",", preserve_empty = true );
  println!( "Only delimiters (preserve_empty): {result:?}" );
  assert_eq!( result, vec![ "", "", "", "" ] );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_delimiter_at_start()
{
  let result = optimize_split!( ",abc", "," );
  println!( "Delimiter at start: {result:?}" );
  // Default preserve_empty=true: leading delimiter produces empty first segment
  assert_eq!( result, vec![ "", "abc" ] );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_delimiter_at_end()
{
  let result = optimize_split!( "abc,", "," );
  println!( "Delimiter at end: {result:?}" );
  // Default preserve_empty=true: trailing delimiter produces empty last segment
  assert_eq!( result, vec![ "abc", "" ] );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_consecutive_delimiters()
{
  let result = optimize_split!( "a,,b", "," );
  println!( "Consecutive delimiters: {result:?}" );
  // Default preserve_empty=true: consecutive delimiters produce empty middle segment
  assert_eq!( result, vec![ "a", "", "b" ] );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_consecutive_delimiters_preserve_empty()
{
  let result = optimize_split!( "a,,b", ",", preserve_empty = true );
  println!( "Consecutive delimiters (preserve_empty): {result:?}" );
  assert_eq!( result, vec![ "a", "", "b" ] );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_unicode_input()
{
  let result = optimize_split!( "café,naïve", "," );
  println!( "Unicode input: {result:?}" );
  assert_eq!( result, vec![ "café", "naïve" ] );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_multibyte_utf8_emoji()
{
  let result = optimize_split!( "😀,😁,😂", "," );
  println!( "Multi-byte UTF-8 emoji: {result:?}" );
  assert_eq!( result, vec![ "😀", "😁", "😂" ] );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_overlapping_delimiters()
{
  let result = optimize_split!( "aabaabc", [ "a", "ab", "abc" ] );
  println!( "Overlapping delimiters: {result:?}" );
  // ComplexPattern regex tries alternates left-to-right; "a" wins over "ab"/"abc"
  // "a" is a delimiter — no segment should equal standalone "a"
  assert!( result.iter().all( | s | *s != "a" ) );
  // "b"/"bc" appear between "a"s and must survive in some segment
  assert!( result.iter().any( | s | s.contains( 'b' ) ) );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_whitespace_delimiters()
{
  let result = optimize_split!( "a\nb\tc", [ "\n", "\t" ] );
  println!( "Whitespace delimiters: {result:?}" );
  assert_eq!( result, vec![ "a", "b", "c" ] );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_many_delimiters_threshold()
{
  // Test with exactly 8 delimiters (threshold) — stays in MultipleCharDelimiters path
  let result = optimize_split!( "a1b2c3d4e5f6g7h", [ "1", "2", "3", "4", "5", "6", "7", "8" ] );
  println!( "8 delimiters (threshold): {result:?}" );
  // "8" not in source; splits on "1"-"7" only → 8 segments
  assert_eq!( result, vec![ "a", "b", "c", "d", "e", "f", "g", "h" ] );
}

#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn corner_many_delimiters_over_threshold()
{
  // Test with 9 delimiters (over threshold → ComplexPattern path)
  let result = optimize_split!( "a1b2c3d4e5f6g7h8i", [ "1", "2", "3", "4", "5", "6", "7", "8", "9" ] );
  println!( "9 delimiters (over threshold): {result:?}" );
  // All digits 1-8 present in source; "9" absent; ComplexPattern → 9 segments
  assert_eq!( result, vec![ "a", "b", "c", "d", "e", "f", "g", "h", "i" ] );
}

// ============================================================================
// optimize_match! Corner Cases
// ============================================================================

#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn corner_match_empty_input()
{
  let result = optimize_match!( "", "test" );
  println!( "Match empty input: {result:?}" );
  assert_eq!( result, None );
}

#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn corner_match_empty_pattern()
{
  let result = optimize_match!( "test", "" );
  println!( "Match empty pattern: {result:?}" );
  // Matches Rust str::find("") behavior: empty pattern matches at byte position 0
  assert_eq!( result, Some( 0 ) );
}

#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn corner_match_no_match()
{
  let result = optimize_match!( "hello", "world" );
  println!( "No match: {result:?}" );
  assert_eq!( result, None );
}

#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn corner_match_at_start()
{
  let result = optimize_match!( "prefix_value", "prefix" );
  println!( "Match at start: {result:?}" );
  assert_eq!( result, Some( 0 ) );
}

#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn corner_match_at_end()
{
  let result = optimize_match!( "value_suffix", "suffix" );
  println!( "Match at end: {result:?}" );
  assert!( result.is_some() );
}

#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn corner_match_in_middle()
{
  let result = optimize_match!( "pre_mid_post", "mid" );
  println!( "Match in middle: {result:?}" );
  assert!( result.is_some() );
}

#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn corner_match_multiple_occurrences()
{
  let result = optimize_match!( "aaaa", "a" );
  println!( "Multiple occurrences: {result:?}" );
  // Should return first match (index 0)
  assert_eq!( result, Some( 0 ) );
}

#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn corner_match_unicode()
{
  let result = optimize_match!( "café in Paris", "café" );
  println!( "Unicode match: {result:?}" );
  assert_eq!( result, Some( 0 ) );
}

#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn corner_match_multibyte_utf8_emoji()
{
  let result = optimize_match!( "Hello 😀 world", "😀" );
  println!( "Multi-byte UTF-8 emoji match: {result:?}" );
  assert!( result.is_some() );
}

#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn corner_match_overlapping_patterns()
{
  let result = optimize_match!( "testing", [ "test", "testing" ] );
  println!( "Overlapping patterns: {result:?}" );
  // Should return first match (which pattern?)
  assert_eq!( result, Some( 0 ) );
}

#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn corner_match_many_patterns_threshold()
{
  // Test with exactly 16 patterns (threshold) — stays in TrieBasedMatch path
  let result = optimize_match!( "test08data", [ "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15", "16" ] );
  println!( "16 patterns (threshold): {result:?}" );
  // "08" appears at byte position 4 in "test08data"
  assert_eq!( result, Some( 4 ) );
}

#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn corner_match_many_patterns_over_threshold()
{
  // Test with 17 patterns (over threshold → SequentialMatch path)
  let result = optimize_match!( "test17data", [ "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17" ] );
  println!( "17 patterns (over threshold): {result:?}" );
  // "17" appears at byte position 4 in "test17data"
  assert_eq!( result, Some( 4 ) );
}

#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn corner_match_longest_match_strategy()
{
  let result = optimize_match!( "testing", [ "test", "testing" ], strategy = "longest_match" );
  println!( "Longest match strategy: {result:?}" );
  // strategy param is a no-op (see docs/invariant/003_strategy_param_no_op.md)
  // both "test" and "testing" match at position 0; first-pattern order → Some(0)
  assert_eq!( result, Some( 0 ) );
}
