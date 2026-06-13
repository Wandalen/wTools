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

//! Task 015 display width fix tests (T015-P01 through T015-N07) are in
//! `unicode_display_width_table.rs`.

#![ cfg( feature = "enabled" ) ]
use data_fmt::{ truncate_cell, pad_to_width, visual_len, RowBuilder, TableFormatter, Format };

/// Bug reproducer for BUG-001: Cyrillic filenames misalign in column padding.
///
/// ## Root Cause
/// `pad_to_width()` pads by character count; downstream `{:<N}` formats by display
/// width. For Cyrillic (1 col/char) both agree, but the same code path breaks for
/// CJK (2 cols/char), so this scenario validates the Cyrillic boundary condition.
///
/// ## Why Not Caught
/// All padding tests used ASCII-only filenames. The gdrive listing with mixed
/// Cyrillic/ASCII filenames was never included in the test matrix.
///
/// ## Fix Applied
/// Updated `pad_to_width()` to measure column width via `UnicodeWidthStr::width()`
/// so padding is display-width-correct for all Unicode categories.
///
/// ## Prevention
/// Any new text-padding function must include a test using both Cyrillic (width-1)
/// and CJK (width-2) content alongside ASCII.
///
/// ## Pitfall
/// Cyrillic looks correct visually when rendered alone — the bug only surfaces in
/// side-by-side column alignment where width mismatch shifts subsequent columns.
// test_kind: bug_reproducer(BUG-001)
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

/// Bug reproducer for BUG-001: CJK characters misalign with character-count padding.
///
/// ## Root Cause
/// CJK characters have display width 2 but `pad_to_width()` counts each as width 1,
/// so padding stops 1 space short per CJK character, leaving the column visually
/// narrower than the target width.
///
/// ## Why Not Caught
/// No test mixed CJK text with ASCII in a padded-column scenario. Tests verified
/// string length, not terminal display width.
///
/// ## Fix Applied
/// `pad_to_width()` now uses `UnicodeWidthStr::width()` — pads until display width
/// reaches the target, not character count.
///
/// ## Prevention
/// Test matrix must cover (ASCII, Cyrillic, CJK, emoji) × (pad, truncate, align).
///
/// ## Pitfall
/// `str.chars().count()` and `UnicodeWidthStr::width()` agree for ASCII and Cyrillic
/// but diverge for CJK (×2) and emoji (×2). Always use display width for terminals.
// test_kind: bug_reproducer(BUG-001)
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

/// Bug reproducer for BUG-001: Emoji misalign with character-count padding.
///
/// ## Root Cause
/// Emoji have display width 2; character-count padding produces half the needed
/// space, causing each emoji to consume an extra column from the alignment budget.
///
/// ## Why Not Caught
/// Emoji were absent from all padding test cases. Terminal rendering differences
/// also made emoji tests harder without a dedicated display-width library.
///
/// ## Fix Applied
/// `pad_to_width()` now uses `UnicodeWidthStr::width()` so emoji are padded correctly.
///
/// ## Prevention
/// Include emoji in the standard test matrix for all string-width operations.
///
/// ## Pitfall
/// Some terminals render emoji as width-1; `unicode-width` follows the Unicode
/// standard (mostly width-2). Test against the standard, not terminal behavior.
// test_kind: bug_reproducer(BUG-001)
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

/// Bug reproducer for BUG-001: Production gdrive listing reveals padding bug.
///
/// ## Root Cause
/// Real-world file listing mixed ASCII and Cyrillic filenames. `pad_to_width()`
/// padded by character count; for Cyrillic this matched display width, but the same
/// path fails for wide characters — this test validates the exact production scenario.
///
/// ## Why Not Caught
/// The gdrive listing was a manual observation, not a programmatic regression test.
/// No automated test reproduced the mixed-script filename table until this reproducer.
///
/// ## Fix Applied
/// Display-width-aware padding ensures both ASCII and Cyrillic filenames align.
///
/// ## Prevention
/// New format use-cases from production should be added as reproducers immediately
/// when the bug is filed.
///
/// ## Pitfall
/// The bug looked like a Cyrillic issue, but the real fix handles ALL wide scripts.
/// A Cyrillic-only fix would have left CJK and emoji broken.
// test_kind: bug_reproducer(BUG-001)
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
/// Bug reproducer for BUG-001 (zero-width variant)
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
/// ## Root Cause
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
/// ## Why Not Caught
///
/// Existing truncation tests in `column_truncation.rs` test ASCII-only content or
/// don't verify actual display width of truncated result. No tests covered:
/// - CJK character truncation with display width verification
/// - Emoji truncation with display width verification
/// - Mixed-width content truncation (ASCII + CJK + emoji)
///
/// ## Fix Applied
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
/// ## Prevention
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
/// ## Pitfall
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
// test_kind: bug_reproducer(BUG-001)
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

/// Bug reproducer for BUG-001: Emoji truncation exceeds display-width target.
///
/// ## Root Cause
/// `truncate_cell()` counted characters, not display width. Emoji occupy 2 terminal
/// columns each, so truncating to N characters can produce up to 2N display columns.
///
/// ## Why Not Caught
/// Truncation tests used ASCII-only content. Emoji were never tested in the
/// truncation code path.
///
/// ## Fix Applied
/// `truncate_single_line()` uses `UnicodeWidthChar::width()` to accumulate display
/// width instead of incrementing by 1 per character.
///
/// ## Prevention
/// Any truncation function must test with ASCII (width=1), CJK (width=2), and
/// emoji (width=2), verifying `UnicodeWidthStr::width(result) <= target_width`.
///
/// ## Pitfall
/// After display-width truncation, the result can be 1 column SHORT of target when
/// a wide char straddles the boundary — this is correct, not a bug.
// test_kind: bug_reproducer(BUG-001)
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

/// Bug reproducer for BUG-001: Mixed-width content truncation exceeds display budget.
///
/// ## Root Cause
/// When truncating mixed ASCII + CJK + emoji, character-count-based truncation
/// underestimates wide characters, causing the result to exceed the display-width target.
///
/// ## Why Not Caught
/// No test combined ASCII, CJK, and emoji in a single truncated string. Each
/// character type was tested in isolation at most.
///
/// ## Fix Applied
/// `truncate_single_line()` accumulates display width via `UnicodeWidthChar::width()`
/// so all character categories are handled correctly in a single pass.
///
/// ## Prevention
/// Include at least one mixed-content truncation test (ASCII + wide + emoji) in the
/// test matrix for any text-manipulation function.
///
/// ## Pitfall
/// Mixed-width strings require width-aware operations end-to-end: truncation AND
/// padding AND alignment must all use display width, or alignment will still break.
// test_kind: bug_reproducer(BUG-001)
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

// --- IN-9: ANSI-only cell (no visible characters) — column width is zero, no extra padding ---
//
// Given: a cell containing only ANSI escape codes with no visible characters.
// When: visual_len() is called on the string, and the cell is rendered in a table.
// Then: visual_len() returns 0; ANSI codes are preserved verbatim; adjacent rows define column width.

/// IN-9 — `invariant/002_ansi_unicode`: ANSI-only cell contributes zero to column width measurement.
// test_kind: standard
#[ test ]
fn ansi_only_cell_zero_visual_width_in9()
{
  let ansi_only = "\x1b[32m\x1b[0m"; // ANSI color-on + reset; no visible characters

  // visual_len strips ANSI sequences and counts remaining chars → 0
  assert_eq!(
    visual_len( ansi_only ), 0,
    "ANSI-only string must have visual_len = 0; got {}",
    visual_len( ansi_only ),
  );

  // Table rendering: ANSI-only cell in one row + visible cell in another row (same column)
  // Column width is determined by the visible row ("abc"), not the ANSI-only row.
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ ansi_only.into() ] )
    .add_row( vec![ "abc".into() ] )
    .build_view();
  let output = TableFormatter::new().format( &view ).unwrap_or_default();

  // No panic; output non-empty (header + separator + data rows)
  assert!( !output.is_empty(), "table with ANSI-only cell must produce non-empty output" );
  // ANSI codes preserved verbatim in output
  assert!(
    output.contains( "\x1b[32m" ),
    "ANSI codes in ANSI-only cell must be preserved verbatim in output",
  );
  // Visible cell present
  assert!( output.contains( "abc" ), "visible cell must appear in output" );
}

// --- IN-10: combining character sequences — visual width equals base character width ---
//
// Given: a cell containing a base character followed by a combining mark (e.g. "e\u{0301}" = é).
// When: the column width is computed via unicode_visual_len (East Asian Width).
// Then: the combining mark contributes 0 to display width; column width = 1 (same as base alone).

/// IN-10 — `invariant/002_ansi_unicode`: combining marks have zero display width; column width equals base.
// test_kind: standard
#[ test ]
fn combining_marks_visual_width_equals_base_width_in10()
{
  use unicode_width::UnicodeWidthStr;

  // e + combining acute accent (U+0301) = decomposed grapheme; display width = 1
  let combining = "e\u{0301}"; // 2 code points, EAW width 1 (combining mark = 0)

  // pad_to_width uses EAW internally: for combining (width 1), adds 4 spaces to reach target 5
  let padded = pad_to_width( combining, 5, false );
  assert_eq!(
    UnicodeWidthStr::width( padded.as_str() ), 5,
    "combining grapheme (EAW=1) padded to 5 must have display width 5; \
     got: {} for '{}'",
    UnicodeWidthStr::width( padded.as_str() ),
    padded,
  );

  // Table rendering: combining cell and ASCII 'e' cell in same column must align
  let view = RowBuilder::new( vec![ "A".into() ] )
    .add_row( vec![ combining.into() ] )
    .add_row( vec![ "e".into() ] )
    .build_view();
  let output = TableFormatter::new().format( &view ).unwrap_or_default();
  assert!( !output.is_empty(), "combining mark table must render without panic" );
}
