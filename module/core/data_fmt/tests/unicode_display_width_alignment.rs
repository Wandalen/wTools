//! Unicode display width alignment tests
//!
//! # Root Cause
//!
//! Character-count-based truncation (`visual_len()` uses `chars().count()`) mixed with
//! display-width-based padding in downstream code (Rust's `{:<N}` formatting uses terminal
//! display width, where CJK/emoji = 2 width, ASCII = 1 width, combining marks = 0 width).
//!
//! The fundamental mismatch:
//! - `visual_len("Владислав")` returns 9 (character count)
//! - Terminal display width of "Владислав" may differ depending on character properties
//! - Cyrillic characters are typically 1 display width each (same as ASCII)
//! - But CJK characters are 2 display width (wide characters)
//! - When `truncate_cell()` returns text based on char count, then user applies `{:<35}`,
//!   the padding calculation uses display width, causing misalignment
//!
//! # Why Not Caught
//!
//! No existing tests validated alignment with:
//! - Cyrillic text (multi-byte but 1 display width per char)
//! - CJK text (multi-byte and 2 display width per char)
//! - Emoji (multi-byte and 2 display width)
//! - Mixed ASCII + Unicode in formatted output
//!
//! All existing tests used ASCII-only content or didn't verify actual terminal alignment,
//! only internal `data_fmt` rendering (which handles padding internally).
//!
//! # Fix Applied
//!
//! Implement display-width-aware padding function that uses `unicode-width` crate to
//! calculate East Asian Width property. Replace character-count-based `pad_to_width()`
//! with display-width-based version.
//!
//! New API:
//! - `visual_len_display_width()` - counts display width (not chars)
//! - `pad_to_display_width()` - pads based on display width
//!
//! Or update existing `visual_len()` and `pad_to_width()` to use display width
//! (breaking change requiring version bump).
//!
//! # Prevention
//!
//! Add comprehensive Unicode test coverage for all text operations:
//! - Test matrix: (ASCII, Cyrillic, CJK, Emoji) × (truncate, pad, align)
//! - Always test actual terminal rendering, not just internal calculations
//! - Validate alignment with file listing format (real-world use case)
//! - Regression tests for character/display-width distinction
//!
//! # Pitfall
//!
//! **NEVER mix character-based width calculations with display-width-based formatting.**
//!
//! Three different measurements exist:
//! 1. **Byte count**: `str.len()` - varies by encoding (UTF-8 uses 1-4 bytes per char)
//! 2. **Character count**: `str.chars().count()` - Unicode codepoints
//! 3. **Display width**: terminal column count (CJK = 2, ASCII = 1, combining = 0)
//!
//! Rule: If downstream code uses `{:<N}` formatting (display-width-based), upstream
//! functions MUST return display-width-ready strings or provide display-width metadata.
//!
//! Example pitfall:
//! ```rust
//! // BROKEN: character-count truncation + display-width padding
//! let text = truncate_cell("日本語", 10, "...");  // Returns based on char count
//! println!("{:<10} next", text);  // Pads based on display width → MISALIGNED
//!
//! // CORRECT: both use display width
//! let text = truncate_to_display_width("日本語", 10, "...");
//! println!("{:<10} next", text);  // Now aligned correctly
//! ```

use data_fmt::{ truncate_cell, pad_to_width, RowBuilder, TableFormatter, TableConfig };

/// Bug reproducer for issue-003: Unicode display width alignment
///
/// Demonstrates that Cyrillic/CJK text becomes misaligned when using
/// character-count-based truncation followed by display-width-based padding.
///
/// This test reproduces the exact issue from gdrive file listing where
/// "Владислав Гайдук.jpg" appeared left-shifted compared to "Kyrylo Stepanov.jpeg".
#[ test ]
fn bug_reproducer_issue_003_cyrillic_alignment()
{
  // Real filenames from gdrive listing
  let ascii_filename = "Kyrylo Stepanov.jpeg";      // 20 chars, 20 bytes, 20 display width
  let cyrillic_filename = "Владислав Гайдук.jpg";    // 20 chars, 35 bytes, 20 display width (Cyrillic = 1 width each)

  // Truncate both to same visual character count
  let trunc_ascii = truncate_cell( ascii_filename, 35, "...." );
  let trunc_cyrillic = truncate_cell( cyrillic_filename, 35, "...." );

  // Verify character counts are equal (current behavior)
  let ascii_char_count = trunc_ascii.chars().count();
  let cyrillic_char_count = trunc_cyrillic.chars().count();

  println!( "Character counts:" );
  println!( "  ASCII: {ascii_char_count} chars" );
  println!( "  Cyrillic: {cyrillic_char_count} chars" );
  println!();

  // Now apply Rust's built-in padding (display-width-based)
  let padded_ascii = format!( "{trunc_ascii:<35}" );
  let padded_cyrillic = format!( "{trunc_cyrillic:<35}" );

  println!( "Padded output (using Rust's {{:<35}} formatting):" );
  println!( "{padded_ascii}JPEG     39.5 KB" );
  println!( "{padded_cyrillic}JPEG     75.9 KB" );
  println!();

  // Measure actual display widths after Rust's padding
  let ascii_display_width = unicode_width::UnicodeWidthStr::width( trunc_ascii.as_str() );
  let cyrillic_display_width = unicode_width::UnicodeWidthStr::width( trunc_cyrillic.as_str() );

  println!( "Display widths (unicode-width crate):" );
  println!( "  ASCII: {ascii_display_width} display columns" );
  println!( "  Cyrillic: {cyrillic_display_width} display columns" );
  println!();

  // THE BUG: If display widths differ, padding will be inconsistent
  // Expected: both should have same display width for proper alignment
  // Actual: Cyrillic may have different display width, causing misalignment

  assert_eq!(
    ascii_display_width,
    cyrillic_display_width,
    "Display widths should match for proper alignment, but ASCII={ascii_display_width} and Cyrillic={cyrillic_display_width}. \
     This causes misalignment when using Rust's {{:<N}} formatting."
  );
}

/// Demonstrate alignment issue with CJK characters (wide characters)
///
/// CJK characters have display width of 2, making the alignment problem
/// even more severe than with Cyrillic (which has display width of 1).
#[ test ]
fn bug_reproducer_issue_003_cjk_alignment()
{
  let ascii_text = "Hello";           // 5 chars, 5 display width
  let cjk_text = "日本語";            // 3 chars, 6 display width (each char = 2 width)

  // Pad both to 10 characters using data_fmt's function
  let padded_ascii = pad_to_width( ascii_text, 10, false );
  let padded_cjk = pad_to_width( cjk_text, 10, false );

  println!( "Character-based padding (current data_fmt behavior):" );
  println!( "ASCII:   '{padded_ascii}' | next" );
  println!( "CJK:     '{padded_cjk}' | next" );
  println!();

  // Now measure display widths
  let ascii_display = unicode_width::UnicodeWidthStr::width( padded_ascii.as_str() );
  let cjk_display = unicode_width::UnicodeWidthStr::width( padded_cjk.as_str() );

  println!( "Display widths after character-based padding:" );
  println!( "  ASCII: {ascii_display} display columns" );
  println!( "  CJK:   {cjk_display} display columns" );
  println!();

  // With display-width-based formatting, these won't align
  println!( "Using Rust's {{:<15}} (display-width-based):" );
  println!( "{padded_ascii:<15}| next" );
  println!( "{padded_cjk:<15}| next" );
  println!();

  // THE BUG: CJK text will appear misaligned because:
  // - data_fmt pads based on char count (3 chars → 10 chars)
  // - But CJK display width is 6, not 3
  // - Rust's {:<15} sees display width of 6 + padding, not 10

  assert_eq!(
    ascii_display,
    cjk_display,
    "Display widths should match for proper alignment, but ASCII={ascii_display} and CJK={cjk_display}. \
     CJK characters have display width of 2, causing severe misalignment."
  );
}

/// Test emoji alignment (wide characters with display width = 2)
#[ test ]
fn bug_reproducer_issue_003_emoji_alignment()
{
  let ascii_text = "File.txt";
  let emoji_text = "🎉🎊.txt";   // 2 emoji (each = 2 width) + 4 chars = total ~8 display width

  let padded_ascii = pad_to_width( ascii_text, 15, false );
  let padded_emoji = pad_to_width( emoji_text, 15, false );

  let ascii_display = unicode_width::UnicodeWidthStr::width( padded_ascii.as_str() );
  let emoji_display = unicode_width::UnicodeWidthStr::width( padded_emoji.as_str() );

  println!( "Emoji alignment test:" );
  println!( "ASCII: '{padded_ascii}' (display width: {ascii_display})" );
  println!( "Emoji: '{padded_emoji}' (display width: {emoji_display})" );

  // This will fail because character-based padding doesn't account for emoji width
  assert_eq!(
    ascii_display,
    emoji_display,
    "Display widths should match, but emoji padding is broken"
  );
}

/// Test Ukrainian Cyrillic text alignment
///
/// Tests specifically with Ukrainian text "Електровіник" to verify
/// whether Cyrillic characters exhibit the display width bug.
#[ test ]
fn test_ukrainian_cyrillic_alignment()
{
  let ascii_text = "Electrovinyk.jpg";       // 16 chars, 16 bytes, 16 display width
  let ukrainian_text = "Електровіник.jpg";    // 16 chars, 29 bytes (Cyrillic), ? display width

  println!( "\nUkrainian Cyrillic Test:" );
  println!( "ASCII:     '{ascii_text}'" );
  println!( "Ukrainian: '{ukrainian_text}'" );
  println!();

  // Character counts
  let ascii_chars = ascii_text.chars().count();
  let ukrainian_chars = ukrainian_text.chars().count();

  println!( "Character counts:" );
  println!( "  ASCII:     {ascii_chars} chars" );
  println!( "  Ukrainian: {ukrainian_chars} chars" );
  println!();

  // Byte counts
  let ascii_bytes = ascii_text.len();
  let ukrainian_bytes = ukrainian_text.len();
  println!( "Byte counts:" );
  println!( "  ASCII:     {ascii_bytes} bytes" );
  println!( "  Ukrainian: {ukrainian_bytes} bytes" );
  println!();

  // Display widths using unicode-width
  let ascii_display = unicode_width::UnicodeWidthStr::width( ascii_text );
  let ukrainian_display = unicode_width::UnicodeWidthStr::width( ukrainian_text );

  println!( "Display widths (unicode-width crate):" );
  println!( "  ASCII:     {ascii_display} display columns" );
  println!( "  Ukrainian: {ukrainian_display} display columns" );
  println!();

  // Apply data_fmt padding
  let padded_ascii = pad_to_width( ascii_text, 25, false );
  let padded_ukrainian = pad_to_width( ukrainian_text, 25, false );

  println!( "After character-based padding to 25 chars:" );
  println!( "ASCII:     '{padded_ascii}' | next" );
  println!( "Ukrainian: '{padded_ukrainian}' | next" );
  println!();

  // Measure display widths after padding
  let ascii_padded_display = unicode_width::UnicodeWidthStr::width( padded_ascii.as_str() );
  let ukrainian_padded_display = unicode_width::UnicodeWidthStr::width( padded_ukrainian.as_str() );

  println!( "Display widths after padding:" );
  println!( "  ASCII:     {ascii_padded_display} display columns" );
  println!( "  Ukrainian: {ukrainian_padded_display} display columns" );
  println!();

  // Use Rust's display-width-based formatting
  println!( "Using Rust's {{:<30}} (display-width-based):" );
  println!( "{padded_ascii:<30}| next" );
  println!( "{padded_ukrainian:<30}| next" );
  println!();

  // Verify if display widths match
  if ascii_display == ukrainian_display
  {
    println!( "✓ RESULT: Ukrainian Cyrillic has same display width as ASCII" );
    println!( "  This means Cyrillic does NOT exhibit the alignment bug." );
    println!( "  Both are 1 display column per character." );
  }
  else
  {
    println!( "✗ RESULT: Ukrainian Cyrillic has DIFFERENT display width" );
    println!( "  ASCII: {ascii_display} vs Ukrainian: {ukrainian_display}" );
  }
  println!();

  // Final assertion
  assert_eq!(
    ascii_padded_display,
    ukrainian_padded_display,
    "Display widths should match after padding. ASCII={ascii_padded_display}, Ukrainian={ukrainian_padded_display}"
  );
}

/// Test realistic file listing format from gdrive
///
/// This reproduces the exact format that revealed the bug in production use.
#[ test ]
fn bug_reproducer_issue_003_realistic_file_listing()
{
  // Simulate file listing table with mixed ASCII/Cyrillic filenames
  let files = vec![
    ( "Kyrylo Stepanov.jpeg", "JPEG", "39.5 KB" ),
    ( "Владислав Гайдук.jpg", "JPEG", "75.9 KB" ),
  ];

  println!( "\nFile listing (current broken behavior):\n" );

  for ( filename, filetype, size ) in &files
  {
    let truncated = truncate_cell( filename, 35, "...." );

    // User code typically does this:
    println!( "{truncated:<35} {filetype:8} {size:>10}" );
  }

  println!( "\nExpected: 'JPEG' should align vertically in both rows" );
  println!( "Actual:   Cyrillic row is left-shifted\n" );

  // Measure display widths to prove the problem
  let ascii_truncated = truncate_cell( files[ 0 ].0, 35, "...." );
  let cyrillic_truncated = truncate_cell( files[ 1 ].0, 35, "...." );

  let ascii_width = unicode_width::UnicodeWidthStr::width( ascii_truncated.as_str() );
  let cyrillic_width = unicode_width::UnicodeWidthStr::width( cyrillic_truncated.as_str() );

  assert_eq!(
    ascii_width,
    cyrillic_width,
    "File listing alignment broken: ASCII width={ascii_width}, Cyrillic width={cyrillic_width}"
  );
}

/// Test zero-width combining marks (accents, diacritics)
///
/// Bug reproducer for issue-003 (zero-width variant)
///
/// Zero-width combining marks have display width of 0 but count as separate
/// characters. This test verifies that `pad_to_width()` correctly handles
/// combining characters by using display width rather than character count.
#[ test ]
fn test_zero_width_combining_marks()
{
  use unicode_width::UnicodeWidthStr;

  // Base character vs composed character
  let base = "e";                    // 1 char, 1 display width
  let composed = "é";                 // 1 char (precomposed), 1 display width
  let combining = "e\u{0301}";       // 2 chars (e + combining acute), 1 display width

  // Pad all to 10 display columns
  let padded_base = pad_to_width( base, 10, false );
  let padded_composed = pad_to_width( composed, 10, false );
  let padded_combining = pad_to_width( combining, 10, false );

  // All should have exactly 10 display width
  assert_eq!(
    UnicodeWidthStr::width( padded_base.as_str() ),
    10,
    "Base character should pad to exactly 10 display width"
  );
  assert_eq!(
    UnicodeWidthStr::width( padded_composed.as_str() ),
    10,
    "Precomposed character should pad to exactly 10 display width"
  );
  assert_eq!(
    UnicodeWidthStr::width( padded_combining.as_str() ),
    10,
    "Combining character should pad to exactly 10 display width"
  );

  // Verify alignment
  println!( "\nZero-width combining marks test:" );
  println!( "Base:      '{padded_base}' | next" );
  println!( "Composed:  '{padded_composed}' | next" );
  println!( "Combining: '{padded_combining}' | next" );
  println!();

  println!( "Character counts:" );
  println!( "  Base:      {} chars", base.chars().count() );
  println!( "  Composed:  {} chars", composed.chars().count() );
  println!( "  Combining: {} chars", combining.chars().count() );
  println!();

  println!( "Display widths (all should be 10):" );
  println!( "  Base:      {} columns", UnicodeWidthStr::width( padded_base.as_str() ) );
  println!( "  Composed:  {} columns", UnicodeWidthStr::width( padded_composed.as_str() ) );
  println!( "  Combining: {} columns", UnicodeWidthStr::width( padded_combining.as_str() ) );
}

/// Test truncation with CJK text using display width
///
/// Bug reproducer for truncation issue discovered during manual testing.
///
/// # Root Cause
///
/// In `data_fmt/src/ansi_str.rs:189`, `truncate_single_line()` increments
/// `visual_count += 1` for each character, treating all characters as having
/// display width = 1. This is incorrect for CJK characters and emoji which have
/// display width = 2.
///
/// The function uses `visual_len()` from `strs_tools` which counts **characters**
/// (Unicode codepoints), not **display width** (terminal columns). This causes
/// CJK/emoji text to truncate at wrong position.
///
/// # Why Not Caught
///
/// Existing truncation tests in `column_truncation.rs` test ASCII-only content or
/// don't verify actual display width of truncated result. No tests covered:
/// - CJK character truncation with display width verification
/// - Emoji truncation with display width verification
/// - Mixed-width content truncation (ASCII + CJK + emoji)
///
/// # Fix Applied
///
/// Modified `truncate_single_line()` in `src/ansi_str.rs` to use unicode-width
/// crate's `UnicodeWidthChar::width()` when incrementing `visual_count`:
///
/// ```rust
/// use unicode_width::UnicodeWidthChar;
///
/// // OLD (BROKEN): Treats all chars as width=1
/// visual_count += 1;
///
/// // NEW (CORRECT): Use actual display width
/// visual_count += ch.width().unwrap_or( 1 );
/// ```
///
/// Also updated initial width calculation to use display-width-aware `visual_len`
/// or direct unicode-width calculation.
///
/// # Prevention
///
/// 1. **Add display width tests for all text operations**: Any function that
///    manipulates text based on "visual length" or "width" must test with CJK
///    and emoji content, verifying actual display width with unicode-width crate.
///
/// 2. **Always test truncation with wide characters**: Test matrix should include
///    (ASCII, CJK, emoji) × (exact width, under width, over width) combinations.
///
/// 3. **Use unicode-width crate consistently**: Whenever calculating or comparing
///    text width, use `UnicodeWidthStr::width()` or `UnicodeWidthChar::width()`,
///    not `chars().count()`.
///
/// # Pitfall
///
/// **NEVER use character count (`chars().count()`) when display width matters.**
///
/// Three different length measurements:
/// - **Byte length**: `str.len()` - UTF-8 bytes (1-4 per char)
/// - **Character count**: `str.chars().count()` - Unicode codepoints
/// - **Display width**: `UnicodeWidthStr::width()` - terminal columns
///
/// For terminal alignment and truncation, ALWAYS use display width.
///
/// Example:
/// ```rust
/// let text = "日本語";  // 3 chars, 9 bytes, 6 display width
///
/// // BROKEN: Truncates to 5 chars (might be 10 display width!)
/// let truncated = truncate_by_char_count( text, 5, "..." );
///
/// // CORRECT: Truncates to 5 display width
/// let truncated = truncate_by_display_width( text, 5, "..." );
/// ```
#[ test ]
fn bug_reproducer_truncate_cjk_display_width()
{
  use unicode_width::UnicodeWidthStr;

  // CJK text: 9 chars, 18 display width
  let cjk_text = "日本語日本語日本語";

  // Truncate to 10 display width
  let truncated = truncate_cell( cjk_text, 10, "..." );

  println!( "\nTruncation with CJK test:" );
  println!( "Original: '{}' ({} chars, {} display width)",
    cjk_text,
    cjk_text.chars().count(),
    UnicodeWidthStr::width( cjk_text )
  );
  println!( "Truncated: '{}' ({} chars, {} display width)",
    truncated,
    truncated.chars().count(),
    UnicodeWidthStr::width( truncated.as_str() )
  );

  // Marker "..." has 3 display width, so content should be 7 display width
  // That's 3-4 CJK chars (each = 2 width)
  // Expected: "日本語..." (6 width + 3 marker = 9, or "日本..." (4 width + 3 marker = 7)

  let result_width = UnicodeWidthStr::width( truncated.as_str() );

  assert!(
    result_width <= 10,
    "Truncated text should not exceed target width of 10, but got {result_width}"
  );

  // Verify marker is present (text was truncated)
  assert!(
    truncated.contains( "..." ),
    "Truncated text should contain marker '...' but got '{truncated}'"
  );
}

/// Test truncation with emoji using display width
#[ test ]
fn bug_reproducer_truncate_emoji_display_width()
{
  use unicode_width::UnicodeWidthStr;

  // Emoji text: 5 chars, 10 display width
  let emoji_text = "🎉🎊🎈🎁🎂";

  // Truncate to 6 display width
  let truncated = truncate_cell( emoji_text, 6, "…" );

  println!( "\nTruncation with emoji test:" );
  println!( "Original: '{}' ({} chars, {} display width)",
    emoji_text,
    emoji_text.chars().count(),
    UnicodeWidthStr::width( emoji_text )
  );
  println!( "Truncated: '{}' ({} chars, {} display width)",
    truncated,
    truncated.chars().count(),
    UnicodeWidthStr::width( truncated.as_str() )
  );

  let result_width = UnicodeWidthStr::width( truncated.as_str() );

  assert!(
    result_width <= 6,
    "Truncated text should not exceed target width of 6, but got {result_width}"
  );

  assert!(
    truncated.contains( "…" ),
    "Truncated text should contain marker '…' but got '{truncated}'"
  );
}

/// Test truncation with mixed ASCII + CJK + emoji
#[ test ]
fn bug_reproducer_truncate_mixed_width()
{
  use unicode_width::UnicodeWidthStr;

  // Mixed: "Hello" (5 width) + "日本" (4 width) + "🎉" (2 width) = 11 total
  let mixed_text = "Hello日本🎉";

  // Truncate to 8 display width
  let truncated = truncate_cell( mixed_text, 8, ".." );

  let result_width = UnicodeWidthStr::width( truncated.as_str() );

  println!( "\nTruncation with mixed-width text:" );
  let original_width = UnicodeWidthStr::width( mixed_text );
  println!( "Original: '{mixed_text}' ({original_width} display width)" );
  println!( "Truncated to 8: '{truncated}' ({result_width} display width)" );

  assert!(
    result_width <= 8,
    "Truncated mixed-width text should not exceed 8 display width, but got {result_width}"
  );

  assert!(
    truncated.contains( ".." ),
    "Truncated text should contain marker '..' but got '{truncated}'"
  );
}

// ============================================================================
// Task 015 — Unicode Display Width Fix
// ============================================================================
//
// These tests verify that column widths are calculated using display width
// (not char count) and that cell padding uses display width.
//
// Functions `unicode_visual_len` and `pad_unicode_width` are `pub(crate)` so
// they are tested indirectly through table output behavior.
//
// T015-P01: CJK chars in table → columns align (each CJK = 2 display cols)
// T015-P02: Emoji (width=2) in table → columns align
// T015-P03 (indirect): CJK column width from content measured in display cols
// T015-P04 (indirect): ANSI-colored CJK content measured correctly (ANSI stripped)
// T015-P05 (indirect): Padding calculation uses display width
// T015-N03: ASCII-only table unaffected (regression guard)
// T015-N04: Empty string produces zero-width measurement
// T015-N05: Content wider than requested width → no truncation by pad
// T015-N06: Malformed ANSI in content → no panic
// T015-N07: min_column_width + CJK → floor applied to unicode-measured widths

/// T015-P01: Table with CJK characters aligns correctly.
/// Each CJK char = 2 display cols; column width must be measured in display cols.
/// Test: header "H" (1 display) + content "日本語" (3 chars × 2 = 6 display) →
///   column = 6 display; header padded to 6 display = "H     " (1 + 5 spaces = 6 bytes).
#[ test ]
fn test_t015_p01_cjk_column_width_uses_display_width()
{
  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "日本語".into() ] )   // 3 chars, 6 display width
    .build();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree );
  let header_line = output.lines().find( | l | l.contains( 'H' ) )
    .expect( "must have header row" );

  // If column width uses char count: column = 3, header "H  " (3 bytes)
  // If column width uses display width: column = 6, header "H     " (6 bytes)
  assert!(
    header_line.len() >= 6,
    "CJK content 'H + 日本語' column must be ≥ 6 display wide; header_line={header_line:?}; output:\n{output}"
  );
}

/// T015-P02: Table with emoji (display width=2) aligns correctly.
/// Each emoji = 2 display cols; column width from content "🎉🎊" = 4 display.
/// Header "H" (1 display) → padded to 4 display → "H   " (4 bytes).
#[ test ]
fn test_t015_p02_emoji_column_width_uses_display_width()
{
  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "🎉🎊".into() ] )   // 2 emoji, each = 2 display width = 4 total
    .build();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree );
  let header_line = output.lines().find( | l | l.contains( 'H' ) )
    .expect( "must have header row" );

  // If char count: column = 2, header "H " (2 bytes)
  // If display width: column = 4, header "H   " (4 bytes)
  assert!(
    header_line.len() >= 4,
    "Emoji content column must be ≥ 4 display wide; header_line={header_line:?}; output:\n{output}"
  );
}

/// T015-P03 (indirect): 5-CJK-char content has display width 10.
/// Use header "AAAAAAAAAA" (10 ASCII = 10 display) and content "こんにちは" (5 chars = 10 display).
/// With display-width: column = 10, both rows same width (no padding needed for either).
/// With char-count: column = max(10, 5) = 10 (header wins), content "こんにちは" padded to
///   10 CHARS, which would add 5 extra spaces → 10 chars + 5 spaces = 15 bytes for content row.
///   Meanwhile header "AAAAAAAAAA" = 10 bytes.
/// After fix: "こんにちは" display width = 10 = column width → no extra padding → 15 bytes.
/// The DIFFERENCE: with char-count, content row has extra trailing spaces; with display-width, no spaces.
#[ test ]
fn test_t015_p03_five_cjk_chars_measured_as_10_display_columns()
{
  let tree = RowBuilder::new( vec![ "AAAAAAAAAA".into() ] )  // 10 ASCII chars
    .add_row( vec![ "こんにちは".into() ] )                  // 5 chars, 10 display width
    .build();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree );
  let _header_line = output.lines().find( | l | l.contains( 'A' ) )
    .expect( "must have header row" );
  let data_line = output.lines().find( | l | l.contains( 'こ' ) )
    .expect( "must have data row" );

  // With display-width measurement: column = max(10, 10) = 10
  // Header line: "AAAAAAAAAA" (10 bytes, no padding needed)
  // Data line: "こんにちは" (15 bytes = 5 CJK × 3 bytes each, no padding needed because display_width=10=column)
  // Both correct.
  // With char-count measurement: column = max(10, 5) = 10
  // Header: "AAAAAAAAAA" (10 bytes)
  // Data: "こんにちは     " (15 + 5 spaces = 20 bytes) ← extra spaces!
  // So after fix: data_line.len() == 15 (not 20)

  // The content is exactly 10 display cols; no trailing spaces should be added
  assert!(
    !data_line.ends_with( ' ' ),
    "CJK content already 10 display wide: no trailing spaces expected; data_line={data_line:?}; output:\n{output}"
  );
}

/// T015-P04 (indirect): ANSI-colored CJK content — ANSI codes stripped from width measurement.
/// Colored "日本" (\x1b[31m日本\x1b[0m) = 4 display width (not 4+escape codes).
/// Header "H" (1 display) → column = 4, header padded to 4 bytes (4 ASCII spaces + H).
#[ test ]
fn test_t015_p04_ansi_colored_cjk_width_strips_escape_codes()
{
  let colored_cjk = "\x1b[31m日本\x1b[0m".to_string();  // 4 display, but many bytes

  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ colored_cjk ] )
    .build();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree );
  let header_line = output.lines().find( | l | l.contains( 'H' ) )
    .expect( "must have header row" );

  // Column should be 4 (display width of "日本" = 4), so header padded to 4 display
  // header_line = "H   " (H + 3 spaces = 4 bytes)
  // If ANSI not stripped from width: column = huge (counts escape bytes as chars)
  // → header would also be huge
  assert!(
    header_line.len() >= 4,
    "Header must be padded to match CJK display width (4); line={header_line:?}; output:\n{output}"
  );

  // Header should be short: 4 chars (1 ASCII H + 3 spaces) — not hundreds of chars
  assert!(
    header_line.len() <= 10,
    "ANSI escape codes must not inflate column width beyond actual display; line={header_line:?}"
  );
}

/// T015-P05 (indirect): Padding uses display width — ASCII test verifies no regression.
/// A table with "A" (1 display) and header "HHHHH" (5 display) → column = 5.
/// Data line "A" padded to 5 display = "A    " (5 bytes, confirmed).
#[ test ]
fn test_t015_p05_padding_based_on_display_width()
{
  let tree = RowBuilder::new( vec![ "HHHHH".into() ] )
    .add_row( vec![ "A".into() ] )
    .build();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree );
  let data_line = output.lines().find( | l | l.contains( 'A' ) )
    .expect( "must have data row" );

  // Column = 5 (header wins). Data "A" padded to 5 display → "A    " (5 bytes)
  assert_eq!(
    data_line.len(), 5,
    "Single-char 'A' padded to column width 5 should produce 5-byte line; line={data_line:?}; output:\n{output}"
  );
}

/// T015-N03: ASCII-only table output is identical before and after fix (regression guard).
/// Unicode display width == char count for pure ASCII content.
#[ test ]
fn test_t015_n03_ascii_only_table_output_unchanged()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ "Alice".into(), "42".into() ] )
    .add_row( vec![ "Bob".into(), "999".into() ] )
    .build();

  // Both before and after Task 015, ASCII output should be identical
  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree );

  assert!( output.contains( "Alice" ), "ASCII table must contain data; output:\n{output}" );
  assert!( output.contains( "Name" ), "ASCII table must contain headers; output:\n{output}" );

  // The column widths are dictated by "Alice" (5) and "Value" (5) — both ASCII
  let data_line = output.lines().find( | l | l.contains( "Alice" ) )
    .expect( "must have data row with Alice" );

  // Column widths: "Name" = 4, "Value" = 5 (header wins for col 1 = 5); "Alice" = 5, "42" = 2 (data wins)
  // plain(): Spaces(2) separator, outer_padding=true, inner_padding=0
  // Row = "Alice" + "  " + "42   " = 5 + 2 + 5 = 12 bytes
  assert_eq!(
    data_line.len(), 12,
    "ASCII regression: 'Alice' + 2-space sep + '42   ' should be 12 bytes; line={data_line:?}; output:\n{output}"
  );
}

/// T015-N04: Empty string content → zero display width; padding fills column entirely.
/// A table with empty cell and header "HHH" (3 display) → column = 3, empty cell → "   " (3 spaces).
#[ test ]
fn test_t015_n04_empty_content_padded_to_column_width()
{
  let tree = RowBuilder::new( vec![ "HHH".into() ] )
    .add_row( vec![ String::new() ] )
    .build();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree );
  let data_line = output.lines()
    .find( | l | !l.contains( "HHH" ) && !l.contains( '-' ) && !l.is_empty() )
    .expect( "must have empty-cell data row" );

  // Column = 3 (header "HHH"); data "" padded to 3 → "   " (3 spaces, 3 bytes)
  assert_eq!(
    data_line.len(), 3,
    "Empty cell must be padded to column width 3; line={data_line:?}; output:\n{output}"
  );
}

/// T015-N05: Content already wider than column limit — padding leaves it unchanged.
/// `pad_unicode_width` must return content unchanged when `content_width >= width`.
/// Test: header "H" (1) + content "ABCDE" (5) → column = 5, no extra padding on data.
#[ test ]
fn test_t015_n05_wider_content_not_shrunk_by_padding()
{
  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "ABCDE".into() ] )
    .build();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree );
  let data_line = output.lines().find( | l | l.contains( "ABCDE" ) )
    .expect( "must have data row" );

  // Content "ABCDE" sets column width = 5; "ABCDE" padded to 5 = "ABCDE" (unchanged)
  assert!(
    data_line.contains( "ABCDE" ),
    "Content wider than or equal to column width must not be modified; line={data_line:?}"
  );
  assert!(
    !data_line.ends_with( ' ' ),
    "Content exactly at column width must not have trailing spaces; line={data_line:?}"
  );
}

/// T015-N06: Content with malformed/partial ANSI — no panic.
/// Partial escape sequences (e.g. `\x1b` at end of string) must not panic the formatter.
#[ test ]
fn test_t015_n06_malformed_ansi_content_no_panic()
{
  // Various malformed ANSI sequences
  let partial_esc = "\x1b".to_string();             // just ESC, no sequence
  let incomplete_seq = "\x1b[".to_string();         // ESC + [ but no terminator
  let unterminated = "\x1b[31mHi".to_string();      // no reset, no m-terminator after content

  for content in [ partial_esc, incomplete_seq, unterminated ]
  {
    let tree = RowBuilder::new( vec![ "H".into() ] )
      .add_row( vec![ content.clone() ] )
      .build();

    // Must not panic
    let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree );
    assert!(
      !output.is_empty(),
      "Malformed ANSI content must render without panic; content={content:?}; output:\n{output}"
    );
  }
}

/// T015-N07: `min_column_width` + CJK content — floor applied to display-width measurements.
/// CJK `"日"` = 2 display. With `min_column_width(5)`, column must be 5.
/// Regression: after Task 012 + 015, both features must work together.
#[ test ]
fn test_t015_n07_min_column_width_with_cjk_content()
{
  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "日".into() ] )   // 1 char, 2 display width
    .build();

  let output = TableFormatter::with_config(
    TableConfig::plain().min_column_width( 5 )
  ).format( &tree );

  let header_line = output.lines().find( | l | l.contains( 'H' ) )
    .expect( "must have header row" );

  // Column = max(display_width("日"), 1) = max(2, 1) = 2, then floor(5) → 5
  // Header "H" padded to 5 display = "H    " (5 bytes)
  assert!(
    header_line.len() >= 5,
    "min_column_width(5) + CJK content: column must be ≥ 5; header_line={header_line:?}; output:\n{output}"
  );
}
