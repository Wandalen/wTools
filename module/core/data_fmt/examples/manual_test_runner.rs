//! Manual test runner for Unicode display width corner cases.
//!
//! Executes high-priority manual test cases for Unicode handling including:
//! - Emoji with skin tone modifiers
//! - Zero-width joiners (ZWJ sequences)
//! - Regional indicator pairs (flag emoji)
//! - Mixed-width strings (ASCII + CJK + emoji)
//! - Edge cases (empty strings, full-width punctuation, etc.)
//!
//! Run with: `cargo run --example manual_test_runner`

use data_fmt::{ pad_to_width, truncate_cell };
use unicode_width::UnicodeWidthStr;

fn test_case( name : &str, text : &str, target_width : usize )
{
  println!( "\n=== {name} ===" );
  let char_count = text.chars().count();
  println!( "Input: '{text}' ({char_count} chars)" );
  let text_width = UnicodeWidthStr::width( text );
  println!( "Display width: {text_width}" );

  let padded = pad_to_width( text, target_width, false );
  let padded_width = UnicodeWidthStr::width( padded.as_str() );

  println!( "Padded to {target_width} chars: '{padded}'" );
  println!( "Actual display width: {padded_width}" );

  if padded_width == target_width
  {
    println!( "✓ PASS: Display width matches target" );
  }
  else
  {
    println!( "✗ FAIL: Display width {padded_width} != target {target_width}" );
  }
}

fn main()
{
  println!( "=== MANUAL UNICODE DISPLAY WIDTH TESTS ===" );

  // P0-1: Emoji with skin tone modifiers
  test_case(
    "Emoji with skin tone modifier",
    "👋🏽",
    10
  );

  // P0-2: Emoji ZWJ sequences (family emoji)
  test_case(
    "Family emoji (ZWJ sequence)",
    "👨‍👩‍👧‍👦",
    10
  );

  // P0-3: Regional indicator pairs (flag emoji)
  test_case(
    "Flag emoji (US flag)",
    "🇺🇸",
    10
  );

  // P0-4: Mixed-width strings
  test_case(
    "Mixed ASCII + CJK + emoji",
    "Hello日本🎉",
    20
  );

  // P0-5: Empty string
  test_case(
    "Empty string",
    "",
    10
  );

  // P0-6: Padding when exceeding width
  test_case(
    "String already exceeds target width",
    "This is a very long string",
    10
  );

  // P1-1: Full-width punctuation
  test_case(
    "Full-width punctuation",
    "Hello！World？",
    20
  );

  // P1-2: Very long string
  test_case(
    "Very long string (100+ chars)",
    &"a".repeat( 120 ),
    50
  );

  // P1-3: String with only zero-width characters
  test_case(
    "Only zero-width characters",
    "\u{0301}\u{0302}\u{0303}",
    10
  );

  // Additional edge case: Right alignment test
  println!( "\n=== RIGHT ALIGNMENT TEST ===" );
  let ascii = "Hello";
  let cjk = "日本語";

  let padded_ascii = pad_to_width( ascii, 15, true ); // right align
  let padded_cjk = pad_to_width( cjk, 15, true );

  println!( "ASCII right-aligned: '{}' (width: {})", padded_ascii, UnicodeWidthStr::width( padded_ascii.as_str() ) );
  println!( "CJK right-aligned:   '{}' (width: {})", padded_cjk, UnicodeWidthStr::width( padded_cjk.as_str() ) );

  // Truncation test
  println!( "\n=== TRUNCATION WITH UNICODE ===" );
  let long_cjk = "日本語日本語日本語";
  let truncated = truncate_cell( long_cjk, 10, "..." );

  println!( "Original: '{}' ({} display width)", long_cjk, UnicodeWidthStr::width( long_cjk ) );
  println!( "Truncated to 10: '{}' ({} display width)", truncated, UnicodeWidthStr::width( truncated.as_str() ) );
}
