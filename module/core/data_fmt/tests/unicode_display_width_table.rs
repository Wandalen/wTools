//! Task 015 — Unicode display width verification tests
//!
//! ## What This Tests
//!
//! Verifies that column widths are calculated using display width (not char count)
//! and that cell padding uses display width. These tests exercise the
//! `unicode_visual_len` and `pad_unicode_width` implementations indirectly
//! through table output behavior.
//!
//! ## Test Matrix
//!
//! T015-P01: CJK chars in table → columns align (each CJK = 2 display cols)
//! T015-P02: Emoji (width=2) in table → columns align
//! T015-P03 (indirect): CJK column width from content measured in display cols
//! T015-P04 (indirect): ANSI-colored CJK content measured correctly (ANSI stripped)
//! T015-P05 (indirect): Padding calculation uses display width
//! T015-N03: ASCII-only table unaffected (regression guard)
//! T015-N04: Empty string produces zero-width measurement
//! T015-N05: Content wider than requested width → no truncation by pad
//! T015-N06: Malformed ANSI in content → no panic
//! T015-N07: `min_column_width` + CJK → floor applied to unicode-measured widths
//!
//! See `unicode_display_width_alignment.rs` for the `bug_reproducer` BUG-001 tests
//! that motivated this implementation.

#![ cfg( feature = "enabled" ) ]
use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

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
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();
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
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();
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
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();
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
    .add_row( vec![ colored_cjk.into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();
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
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();
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
    .build_view();

  // Both before and after Task 015, ASCII output should be identical
  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();

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
    .add_row( vec![ String::new().into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();
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
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();
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
      .add_row( vec![ content.clone().into() ] )
      .build_view();

    // Must not panic
    let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();
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
    .build_view();

  let output = TableFormatter::with_config(
    TableConfig::plain().with_min_column_width( 5 )
  ).format( &tree ).unwrap_or_default();

  let header_line = output.lines().find( | l | l.contains( 'H' ) )
    .expect( "must have header row" );

  // Column = max(display_width("日"), 1) = max(2, 1) = 2, then floor(5) → 5
  // Header "H" padded to 5 display = "H    " (5 bytes)
  assert!(
    header_line.len() >= 5,
    "min_column_width(5) + CJK content: column must be ≥ 5; header_line={header_line:?}; output:\n{output}"
  );
}

// =============================================================================
// IN-4 through IN-8: ANSI and Unicode Invariant Tests
// =============================================================================

/// IN-4 — `invariant/002`: every colored output line ends with ANSI reset before newline.
///
/// A table row where a cell carries an ANSI color via `with_color` produces output lines
/// that each contain the reset sequence `\x1b[0m`, preventing color bleed across lines.
// test_kind: standard
#[ test ]
fn colored_output_line_ends_with_ansi_reset_in4()
{
  use data_fmt::DecoratedText;

  let view = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ DecoratedText::from( "green" ).with_color( "\x1b[32m" ) ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &view ).unwrap_or_default();

  assert!(
    output.contains( "\x1b[32m" ),
    "colored cell must appear in output:\n{output:?}",
  );
  for line in output.lines()
  {
    if line.contains( '\x1b' )
    {
      assert!(
        line.contains( "\x1b[0m" ),
        "every line with ANSI color must contain reset '\\x1b[0m':\n  line={line:?}\n  full output:\n{output:?}",
      );
    }
  }
}

/// IN-5 — `invariant/002`: multiline cells receive per-sub-line color wrapping.
///
/// A cell with raw ANSI on sub-line 0 and plain text on sub-line 1. Sub-line 0 preserves
/// original ANSI sequences; sub-line 1 contains only its plain text — no injected codes;
/// sequences from sub-line 0 do not copy across to sub-line 1.
// test_kind: standard
#[ test ]
fn multiline_cell_per_sub_line_color_wrapping_in5()
{
  use data_fmt::DecoratedText;

  // Raw ANSI on sub-line 0, plain text on sub-line 1 — no with_color() called
  let cell_text = "\x1b[31mred-line1\x1b[0m\nline2";
  let view = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ DecoratedText::from( cell_text ) ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &view ).unwrap_or_default();

  // Sub-line 0: original ANSI codes must appear on its line
  let sub0 = output.lines().find( | l | l.contains( "red-line1" ) )
    .expect( "sub-line 0 with 'red-line1' must appear in output" );
  assert!(
    sub0.contains( "\x1b[31m" ),
    "sub-line 0 must contain original color code:\n  line={sub0:?}\n  output:\n{output:?}",
  );

  // Sub-line 1: must not carry color codes injected from sub-line 0
  let sub1 = output.lines().find( | l | l.contains( "line2" ) )
    .expect( "sub-line 1 with 'line2' must appear in output" );
  assert!(
    !sub1.contains( "\x1b[31m" ),
    "sub-line 1 must not receive color code from sub-line 0:\n  line={sub1:?}\n  output:\n{output:?}",
  );
}

/// IN-6 — `invariant/002`: `DecoratedText` detail lines iterate raw text, not rendered output.
///
/// A row detail whose text contains `\n`-separated segments with embedded ANSI codes and no
/// color override. Each segment is emitted verbatim — no double-wrapping, no additional
/// rendering pass applied by the formatter.
// test_kind: standard
#[ test ]
fn detail_lines_iterate_raw_text_not_rendered_in6()
{
  use data_fmt::DecoratedText;

  // Detail has raw ANSI in text, no with_color() override → segments emitted as-is
  let detail = DecoratedText::from( "\x1b[33myellow-seg\x1b[0m\nplain-seg" );
  let view = RowBuilder::new( vec![ "H".into() ] )
    .add_row_with_detail( vec![ "data".into() ], Some( detail ) )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &view ).unwrap_or_default();

  // First segment: raw ANSI codes preserved verbatim (not double-wrapped)
  assert!(
    output.contains( "\x1b[33myellow-seg\x1b[0m" ),
    "raw ANSI code in first detail segment must appear verbatim:\n{output:?}",
  );

  // Second segment: no injected color from the formatter
  let plain_line = output.lines().find( | l | l.contains( "plain-seg" ) )
    .expect( "second detail segment must appear in output" );
  assert!(
    !plain_line.contains( "\x1b[33m" ),
    "plain detail segment must not receive injected color:\n  line={plain_line:?}\n  output:\n{output:?}",
  );
}

/// IN-7 — `invariant/002`: CJK column width uses East Asian Width (display width, not char count).
///
/// `"中文"` has char count 2 but display width 4. The formatter correctly measures column
/// width via `unicode_visual_len` (East Asian Width), so the column is allocated 4 display
/// columns — matching the CJK cell's visual footprint. Both the CJK line and the ASCII
/// sibling "ab" (padded to 4) have the same display width (4). This is a regression guard
/// verifying that East Asian Width remains the measurement basis for column allocation.
///
/// Note: `visual_len` (char count) is used by `truncate_cell` — that separate path is the
/// known limitation for BUG-001. Column width allocation correctly uses display width.
// test_kind: standard
#[ test ]
fn cjk_column_width_uses_east_asian_width_in7()
{
  use unicode_width::UnicodeWidthStr;

  // "中文" (char count 2, display width 4) and "ab" (char count 2, display width 2)
  // Column width = max(display_width("H")=1, display_width("中文")=4, display_width("ab")=2) = 4
  let view = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "中文".into() ] )
    .add_row( vec![ "ab".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &view ).unwrap_or_default();

  let cjk_line = output.lines().find( | l | l.contains( "中文" ) )
    .expect( "CJK data row must appear in output" );
  let ascii_line = output.lines().find( | l | l.contains( "ab" ) )
    .expect( "ASCII data row must appear in output" );

  // Column width = 4 (East Asian Width of "中文"). Both lines must have the same display width.
  // CJK: "中文" (4 display) = column width 4 → no overflow
  // ASCII: "ab  " (ab + 2 spaces = 4 display) = column width 4 → correct padding
  let cjk_display = UnicodeWidthStr::width( cjk_line );
  let ascii_display = UnicodeWidthStr::width( ascii_line );
  assert_eq!(
    cjk_display, ascii_display,
    "CJK and ASCII rows must have equal display width (East Asian Width used for column allocation):\n  CJK line display={cjk_display}: {cjk_line:?}\n  ASCII line display={ascii_display}: {ascii_line:?}\n  output:\n{output:?}",
  );
  // Column width must reflect CJK display width (4), not char count (2)
  assert!(
    cjk_display >= 4,
    "column must be at least 4 display cols (CJK display width); got {cjk_display}:\n{output:?}",
  );
}

/// IN-8 — `invariant/002`: ANSI codes combined with CJK — width measured as char count.
///
/// `"\x1b[32m中文\x1b[0m"` is measured as width 2 by `visual_len`: ANSI bytes are
/// stripped, then chars are counted (not display width). A plain "ab" cell is also 2.
/// Both measurement deficiencies are additive: ANSI exclusion + EAW gap coexist.
///
/// Known limitation: measurement uses char count, not East Asian Width. Fix tracked as
/// BUG-001 in `src/ansi_str.rs`.
// test_kind: standard
#[ test ]
fn ansi_plus_cjk_width_measured_as_char_count_in8()
{
  use data_fmt::visual_len;

  let vl_plain = visual_len( "ab" );
  let vl_ansi_cjk = visual_len( "\x1b[32m中文\x1b[0m" );

  // Both measured as 2: ANSI bytes stripped, char count = 2 for both
  assert_eq!(
    vl_plain,
    vl_ansi_cjk,
    "visual_len of ANSI+CJK must equal visual_len of same-char-count ASCII:\n  visual_len('ab')={vl_plain}, visual_len('\\x1b[32m中文\\x1b[0m')={vl_ansi_cjk}",
  );

  // Measurement is char count (2), not byte count (~15) or display width (4)
  assert_eq!(
    vl_ansi_cjk,
    2,
    "ANSI+CJK visual_len must be char count (2), not byte count or display width:\n  got={vl_ansi_cjk}",
  );
}
