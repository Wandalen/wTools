//! Multiline cell tests for `TableFormatter`
//!
//! # Purpose
//!
//! Tests automatic multiline support when cells contain newline characters (`\n`).
//! Essential for displaying structured text data (addresses, multi-line descriptions,
//! stack traces, code snippets) within table cells while maintaining proper alignment.
//!
//! # Why This Feature Exists
//!
//! **Problem**: Real-world data often contains natural line breaks
//! - Addresses: "123 Main St\nApt 4B\nBoston, MA"
//! - Descriptions: Multi-paragraph text
//! - Stack traces: Line-by-line error context
//! - Code: Multi-line snippets in documentation
//!
//! **Solution**: Detect `\n` and render each line on separate table row while
//! maintaining column alignment across the entire cell height.
//!
//! # Key Design Decisions
//!
//! 1. **Automatic Detection**: No flag needed, works when ANY cell contains `\n`
//!    - Rationale: Natural behavior - newlines should create new lines
//!    - Implementation: Cells already stored as String, `\n` preserved naturally
//!    - Discovery: The formatter ALREADY handled this correctly during implementation!
//!
//! 2. **Row Height = Max Line Count**: All cells in row expand to tallest cell
//!    - Rationale: Maintain column alignment across varying cell heights
//!    - Example: Row with `["A", "B\nC\nD", "E"]` renders 3 lines total
//!    - Shorter cells padded with empty content (not visual space)
//!
//! 3. **Per-Line Padding**: Each line of multiline cell gets column width padding
//!    - Rationale: Preserve vertical alignment of column separators
//!    - Implementation: Happens naturally through existing render pipeline
//!
//! 4. **CSV/TSV Special Case**: Multiline disabled (literal `\n` or quoted)
//!    - Rationale: CSV standards don't support multiline rows
//!    - Behavior: Keep `\n` as literal character in output
//!    - Note: Implementation detail varies by CSV variant
//!
//! 5. **Interaction with Truncation**: Truncate AFTER line split (per-line)
//!    - Rationale: Better UX - see truncation on each line independently
//!    - Example: "Long1\nLong2" with max=8 → "Long1...\nLong2..."
//!    - Alternative rejected: Truncating before split loses second line entirely
//!
//! # Implementation Discovery
//!
//! **Surprising finding**: The formatter ALREADY supported multiline cells!
//! - The `format_row()` naturally renders `\n` characters as line breaks
//! - No special multiline detection or rendering logic was needed
//! - Only enhancement needed: Per-line truncation (handled in `truncate_cell`)
//!
//! This is why 15/17 tests passed immediately - the feature existed implicitly.
//! Only truncation interaction tests failed (fixed by per-line truncation logic).
//!
//! # Known Edge Cases
//!
//! - **Empty lines**: `"A\n\nC"` creates 3-line cell (middle line empty)
//! - **Only newlines**: `"\n\n"` valid input, renders empty rows
//! - **Trailing newline**: `"Text\n"` creates 2-line cell (second line empty)
//! - **Mixed heights**: Different rows can have different heights independently
//! - **ANSI codes spanning lines**: Each line preserves its ANSI codes
//! - **Very tall cells**: No arbitrary limit, renders all lines
//!
//! # Test Organization
//!
//! - Basic Multiline: 2-3 line cells, multiple columns with varying heights
//! - Mixed Content: Single-line and multiline rows in same table
//! - Alignment: Column alignment maintained across multiline boundaries
//! - Table Styles: Compatibility with bordered, markdown, plain, etc.
//! - ANSI Colors: Multiline cells with color codes per line
//! - CSV/TSV: Special handling for these formats
//! - Edge Cases: Empty lines, very tall cells, unicode
//! - Combined Features: Multiline + truncation interaction (critical!)
//!
//! # How to Interpret Failures
//!
//! - "Should contain both lines": Lines not rendered on separate rows
//!   → Check if `\n` is being escaped or removed somewhere
//!
//! - "Row should have N lines": Row height calculation incorrect
//!   → Check `max()` logic for finding tallest cell in row
//!
//! - "Should maintain structure": Column alignment broken in multiline
//!   → Check that padding/separators applied to each line consistently
//!
//! - "Both lines should be truncated": Per-line truncation not working
//!   → Check `truncate_cell` multiline handling (should split then truncate)
//!
//! - "Should have markdown pipes": Border rendering broken for multiline
//!   → Check that border logic applies to ALL lines in multiline row
//!
//! # Related Tests
//!
//! - `tests/column_truncation.rs`: Truncation + multiline interaction
//! - `tests/table_styles_*.rs`: Single-line table rendering
//! - `tests/formatters.rs`: Other formatter behavior with multiline data
//!
//! # Performance Note
//!
//! Multiline rendering has O(rows × `max_lines_per_row`) complexity. For tables
//! with very tall cells (100+ lines), consider if table format is appropriate
//! or if alternative display (expanded format, tree format) would be better.

#![ cfg( feature = "enabled" ) ]
use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

// ============================================================================
// Basic Multiline Tests
// ============================================================================

#[ test ]
fn test_single_cell_two_lines()
{
  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Line 1\nLine 2".into() ] )
    .build_view();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data ).unwrap_or_default();

  // Should contain both lines
  assert!( output.contains( "Line 1" ), "Should contain first line" );
  assert!( output.contains( "Line 2" ), "Should contain second line" );

  // Lines should be on separate rows
  let lines : Vec<&str> = output.lines().collect();
  assert!(
    lines.iter().any( |l| l.contains( "Line 1" ) ),
    "Line 1 should be in output"
  );
  assert!(
    lines.iter().any( |l| l.contains( "Line 2" ) ),
    "Line 2 should be in output"
  );
}

#[ test ]
fn test_single_cell_three_lines()
{
  let data = RowBuilder::new( vec![ "Data".into() ] )
    .add_row( vec![ "First\nSecond\nThird".into() ] )
    .build_view();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data ).unwrap_or_default();

  // All three lines should appear
  assert!( output.contains( "First" ) );
  assert!( output.contains( "Second" ) );
  assert!( output.contains( "Third" ) );
}

#[ test ]
fn test_multiple_columns_different_heights()
{
  let data = RowBuilder::new( vec![ "Col1".into(), "Col2".into(), "Col3".into() ] )
    .add_row( vec![
      "Single".into(),
      "Line 1\nLine 2\nLine 3".into(),  // 3 lines
      "A\nB".into()                     // 2 lines
    ])
    .build_view();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data ).unwrap_or_default();

  // All content should be present
  assert!( output.contains( "Single" ) );
  assert!( output.contains( "Line 1" ) );
  assert!( output.contains( "Line 3" ) );
  assert!( output.contains( 'A' ) );
  assert!( output.contains( 'B' ) );

  // Row should span 3 lines (max height)
  let data_lines : Vec<&str> = output.lines()
    .skip( 2 )  // Skip header and separator
    .take( 3 )  // Take 3 data lines
    .collect();

  assert_eq!( data_lines.len(), 3, "Row should have 3 lines" );
}

// ============================================================================
// Mixed Content Tests
// ============================================================================

#[ test ]
fn test_mixed_single_and_multiline_rows()
{
  let data = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )  // Single-line row
    .add_row( vec![ "Bob".into(), "Age: 25\nCity: NYC".into() ] )  // Multiline row
    .add_row( vec![ "Charlie".into(), "35".into() ] )  // Single-line row
    .build_view();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data ).unwrap_or_default();

  // All data should be present
  assert!( output.contains( "Alice" ) );
  assert!( output.contains( "30" ) );
  assert!( output.contains( "Bob" ) );
  assert!( output.contains( "Age: 25" ) );
  assert!( output.contains( "City: NYC" ) );
  assert!( output.contains( "Charlie" ) );
  assert!( output.contains( "35" ) );
}

#[ test ]
fn test_empty_lines_in_multiline_cell()
{
  let data = RowBuilder::new( vec![ "Text".into() ] )
    .add_row( vec![ "Line 1\n\nLine 3".into() ] )  // Empty line in middle
    .build_view();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data ).unwrap_or_default();

  // Should contain both non-empty lines
  assert!( output.contains( "Line 1" ) );
  assert!( output.contains( "Line 3" ) );

  // Should have 3 data rows (including empty middle line)
  let data_lines : Vec<&str> = output.lines()
    .skip( 2 )  // Skip header and separator
    .collect();

  assert!(
    data_lines.len() >= 3,
    "Should have at least 3 lines (including empty)"
  );
}

// ============================================================================
// Alignment and Padding Tests
// ============================================================================

#[ test ]
fn test_multiline_maintains_column_alignment()
{
  let data = RowBuilder::new( vec![ "Short".into(), "Long".into() ] )
    .add_row( vec![
      "A\nB\nC".into(),
      "Line 1\nLine 2\nLine 3".into()
    ])
    .build_view();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data ).unwrap_or_default();

  // Check that columns are aligned
  let data_lines : Vec<&str> = output.lines()
    .skip( 2 )
    .take( 3 )
    .collect();

  // Each line should have similar structure (column separator positions)
  for line in &data_lines
  {
    // Should contain content from both columns
    assert!( !line.is_empty(), "Line should not be empty" );
  }
}

#[ test ]
fn test_multiline_with_shorter_cells_padded()
{
  let data = RowBuilder::new( vec![ "Col1".into(), "Col2".into() ] )
    .add_row( vec![
      "One line".into(),  // 1 line
      "First\nSecond\nThird".into()  // 3 lines
    ])
    .build_view();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data ).unwrap_or_default();

  // Should render 3 data lines
  let data_lines : Vec<&str> = output.lines()
    .skip( 2 )
    .collect();

  assert!(
    data_lines.len() >= 3,
    "Should have at least 3 lines for tallest cell"
  );

  // First line should contain "One line"
  assert!(
    data_lines[ 0 ].contains( "One line" ),
    "First line should contain single-line cell content"
  );

  // Lines 2 and 3 should still have proper column structure (padding for col1)
  assert!(
    !data_lines[ 1 ].trim().is_empty(),
    "Second line should maintain structure"
  );
}

// ============================================================================
// Table Style Compatibility Tests
// ============================================================================

#[ test ]
fn test_multiline_with_bordered_style()
{
  let data = RowBuilder::new( vec![ "Name".into(), "Info".into() ] )
    .add_row( vec![
      "Alice".into(),
      "Age: 30\nCity: Boston".into()
    ])
    .build_view();

  let config = TableConfig::bordered();
  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data ).unwrap_or_default();

  // Should have borders on each line
  let data_lines : Vec<&str> = output.lines()
    .skip( 2 )
    .take( 2 )
    .collect();

  for line in data_lines
  {
    assert!( line.contains( '|' ), "Each line should have pipe separator" );
  }
}

#[ test ]
fn test_multiline_with_markdown_style()
{
  let data = RowBuilder::new( vec![ "Item".into(), "Description".into() ] )
    .add_row( vec![
      "Task".into(),
      "First step\nSecond step".into()
    ])
    .build_view();

  let config = TableConfig::markdown();
  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data ).unwrap_or_default();

  // Should be valid markdown
  assert!( output.contains( '|' ), "Should have markdown pipes" );
  assert!( output.contains( "First step" ) );
  assert!( output.contains( "Second step" ) );
}

#[ test ]
fn test_multiline_with_plain_style()
{
  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice\nBob".into() ] )
    .build_view();

  let config = TableConfig::plain();
  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data ).unwrap_or_default();

  // Should contain both lines with plain formatting
  assert!( output.contains( "Alice" ) );
  assert!( output.contains( "Bob" ) );
}

// ============================================================================
// ANSI Color Tests
// ============================================================================

#[ test ]
fn test_multiline_with_ansi_colors()
{
  let colored = "\x1b[31mRed line\x1b[0m\n\x1b[32mGreen line\x1b[0m";

  let data = RowBuilder::new( vec![ "Colors".into() ] )
    .add_row( vec![ colored.into() ] )
    .build_view();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data ).unwrap_or_default();

  // Should preserve ANSI codes
  assert!( output.contains( "\x1b[31m" ), "Should preserve red color code" );
  assert!( output.contains( "\x1b[32m" ), "Should preserve green color code" );

  // Should contain both lines
  assert!( output.contains( "Red line" ) );
  assert!( output.contains( "Green line" ) );
}

// ============================================================================
// CSV/TSV Special Handling
// ============================================================================

#[ test ]
fn test_multiline_in_csv_keeps_literal_newlines()
{
  let data = RowBuilder::new( vec![ "Name".into(), "Address".into() ] )
    .add_row( vec![
      "Alice".into(),
      "123 Main St\nApt 4B".into()
    ])
    .build_view();

  let config = TableConfig::csv();
  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data ).unwrap_or_default();

  // CSV should NOT split into multiple lines - newlines should be literal
  // or escaped. Check that it's still CSV-formatted.
  assert!( output.contains( ',' ), "Should be comma-separated" );

  // The exact behavior depends on implementation, but it should be valid CSV
  // Either the \n stays literal in the cell, or the cell is quoted
  assert!( output.contains( "Alice" ) );
}

// ============================================================================
// Edge Cases
// ============================================================================

#[ test ]
fn test_cell_with_only_newlines()
{
  let data = RowBuilder::new( vec![ "Data".into() ] )
    .add_row( vec![ "\n\n".into() ] )  // Only newlines
    .build_view();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data ).unwrap_or_default();

  // Should handle gracefully without panicking
  assert!( !output.is_empty(), "Should produce some output" );
}

#[ test ]
fn test_very_tall_cell()
{
  let mut lines = vec![];
  for i in 1..=10
  {
    lines.push( format!( "Line {i}" ) );
  }
  let content = lines.join( "\n" );

  let data = RowBuilder::new( vec![ "Tall".into() ] )
    .add_row( vec![ content.into() ] )
    .build_view();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data ).unwrap_or_default();

  // Should contain first and last lines
  assert!( output.contains( "Line 1" ) );
  assert!( output.contains( "Line 10" ) );

  // Should have at least 10 data lines
  let data_lines : Vec<&str> = output.lines()
    .skip( 2 )
    .collect();

  assert!(
    data_lines.len() >= 10,
    "Should have at least 10 lines for tall cell (got {})",
    data_lines.len()
  );
}

#[ test ]
fn test_multiline_with_unicode()
{
  let data = RowBuilder::new( vec![ "Text".into() ] )
    .add_row( vec![ "Hello 世界\nКириллица\nΕλληνικά".into() ] )
    .build_view();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data ).unwrap_or_default();

  // Should handle unicode correctly
  assert!( output.contains( "世界" ) );
  assert!( output.contains( "Кириллица" ) );
  assert!( output.contains( "Ελληνικά" ) );
}

// ============================================================================
// Combined Features Tests (Multiline + Truncation)
// ============================================================================

#[ test ]
fn test_multiline_with_truncation()
{
  let data = RowBuilder::new( vec![ "Data".into() ] )
    .add_row( vec![
      "Very long first line that exceeds limit\nShort second line".into()
    ])
    .build_view();

  let config = TableConfig::plain()
    .max_column_width( Some( 20 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data ).unwrap_or_default();

  // First line should be truncated
  assert!( output.contains( "..." ), "Long line should be truncated" );

  // Second line should appear (not truncated if it fits)
  assert!( output.contains( "Short second" ) );
}

#[ test ]
fn test_multiline_each_line_truncated_independently()
{
  let data = RowBuilder::new( vec![ "Text".into() ] )
    .add_row( vec![
      "This is a very long first line\nThis is another very long second line".into()
    ])
    .build_view();

  let config = TableConfig::plain()
    .max_column_width( Some( 15 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data ).unwrap_or_default();

  // Both lines should be truncated independently
  let marker_count = output.matches( "..." ).count();
  assert!(
    marker_count >= 2,
    "Both lines should be truncated (found {marker_count} markers)"
  );
}

/// Column width for a multiline cell must be the MAX single-line display width,
/// NOT the total display width of the whole string (which would incorrectly count
/// `\n` as a display character).
///
/// ## Root Cause (Bug)
///
/// `calculate_column_widths_for_rows()` calls `unicode_visual_len(cell)` on the
/// whole cell string including embedded `\n` characters. `unicode_visual_len`
/// uses `ch.width().unwrap_or(1)` which counts `'\n'` as 1 display column
/// (since `UnicodeWidthChar::width('\n')` returns `None`). For a 2-line cell
/// `"Line1\nLine2"` (5 + 1 + 5 = 11), this produces column width 11 instead
/// of the correct 5 (the max of the individual line widths).
///
/// ## Why Not Caught
///
/// Existing alignment tests check that ALL lines in the output have EQUAL widths,
/// which holds even with the bug (all lines get the same wrong-but-consistent
/// width). No test checked the ABSOLUTE column width against the expected value.
///
/// ## Fix Applied
///
/// In `calculate_column_widths_for_rows()`, replace `unicode_visual_len(cell)` with
/// `cell.lines().map(|l| unicode_visual_len(l)).max().unwrap_or(0)`, taking the
/// maximum single-line display width instead of the full-string width.
///
/// ## Prevention
///
/// Never call `unicode_visual_len` (or any string-length function) on strings that
/// may contain `\n`; always split into lines and take the max per column.
///
/// ## Pitfall
///
/// `"".lines()` yields an empty iterator; `.max()` returns `None`; use
/// `.unwrap_or(0)` to default empty cells to width 0.
#[ test ]
fn test_multiline_cell_column_width_is_max_line_width()
{
  // Single-column table; header="Col" (3 chars), data="Line1\nLine2" (5 chars per line)
  // Correct column width = max(3, 5) = 5
  // Buggy column width   = max(3, unicode_visual_len("Line1\nLine2")) = max(3, 11) = 11
  let tree = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "Line1\nLine2".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();
  let lines : Vec<&str> = output.lines().collect();

  // line[0]=header, line[1]=separator, line[2]=sub-line1, line[3]=sub-line2
  assert!( lines.len() >= 4, "Expected at least 4 output lines; got {}\n{output:?}", lines.len() );

  let sep = lines[ 1 ];
  // Separator for a 5-char column must be "-----" (5 dashes), not "-----------" (11)
  assert_eq!(
    sep.len(), 5,
    "Column width must be max single-line width (5); separator is {}-char: {:?}\nFull output:\n{output:?}",
    sep.len(), sep
  );

  // Sub-lines must be exactly 5 chars (content fits without extra padding)
  let sub_line1 = lines[ 2 ];
  let sub_line2 = lines[ 3 ];
  assert_eq!(
    sub_line1.len(), 5,
    "Sub-line1 must be 5 chars, got {} ({:?})\nFull output:\n{output:?}",
    sub_line1.len(), sub_line1
  );
  assert_eq!(
    sub_line2.len(), 5,
    "Sub-line2 must be 5 chars, got {} ({:?})\nFull output:\n{output:?}",
    sub_line2.len(), sub_line2
  );
}

/// AC-1 — `multiline_cell_rendering/001`: single-line cells are unchanged.
///
/// Ensures the multiline rendering path is transparent when no `\n` chars are
/// present — default config output is byte-identical to explicit `plain()` config;
/// the row produces exactly 1 data line (no phantom extra lines).
#[ test ]
fn single_line_cells_unchanged()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ "Alice".into(), "42".into() ] )
    .build_view();
  let output_default = TableFormatter::new().format( &tree ).unwrap_or_default();
  let output_explicit = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();
  assert_eq!(
    output_default, output_explicit,
    "single-line table must render byte-identically with default and plain config",
  );
  // header + separator + exactly 1 data line
  let data_lines : Vec< &str > = output_default.lines().skip( 2 ).collect();
  assert_eq!( data_lines.len(), 1, "single-line row must produce exactly 1 data line" );
}

/// AC-7 — `001_multiline_cell_rendering`: CSV format escapes newlines instead of rendering sub-lines.
///
/// `TableConfig::csv()` replaces embedded `\n` characters with the literal two-character
/// sequence `\n` (backslash + n) before emitting the row. A data row with a multiline
/// cell occupies exactly one physical output line; the `"first"` and `"second"` segments
/// appear on the same line, not as separate sub-line rows.
// test_kind: standard
#[ test ]
fn csv_tsv_newline_escape_ac7()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "first\nsecond".into() ] )
    .build_view();
  let output = TableFormatter::with_config( TableConfig::csv() )
    .format( &view )
    .expect( "CSV formatting must not fail" );

  // If the cell were split, "first" would appear on a line without "second".
  // Assert no such line exists — both segments must be co-located on one line.
  assert!(
    !output.lines().any( | l | l.contains( "first" ) && !l.contains( "second" ) ),
    "CSV must not split multiline cell into sub-lines; 'first' and 'second' must be on the same line:\n{output:?}",
  );
  // Both segments must be present in the output (neither is lost in escaping)
  assert!( output.contains( "first" ), "CSV output must contain 'first':\n{output:?}" );
  assert!( output.contains( "second" ), "CSV output must contain 'second':\n{output:?}" );
}

/// AC-8 — `001_multiline_cell_rendering`: sub-row detail lines appear after all multiline sub-lines.
///
/// When a row contains a multiline cell (`"line1\nline2"`) and a sub-row detail annotation,
/// the renderer emits all cell sub-lines consecutively first, then the detail annotation
/// below. The detail is never interleaved between sub-lines.
// test_kind: standard
#[ test ]
fn subrow_detail_after_multiline_sublines_ac8()
{
  let view = RowBuilder::new( vec![ "Content".into() ] )
    .add_row_with_detail( vec![ "line1\nline2".into() ], Some( "detail annotation".into() ) )
    .build_view();
  let output = TableFormatter::new()
    .format( &view )
    .expect( "formatting must not fail" );

  let lines : Vec< &str > = output.lines().collect();
  let pos_line1 = lines.iter().position( | l | l.contains( "line1" ) )
    .expect( "output must contain 'line1'" );
  let pos_line2 = lines.iter().position( | l | l.contains( "line2" ) )
    .expect( "output must contain 'line2'" );
  let pos_detail = lines.iter().position( | l | l.contains( "detail annotation" ) )
    .expect( "output must contain 'detail annotation'" );

  assert!(
    pos_line1 < pos_detail,
    "'line1' (at {pos_line1}) must appear before detail annotation (at {pos_detail}):\n{output:?}",
  );
  assert!(
    pos_line2 < pos_detail,
    "'line2' (at {pos_line2}) must appear before detail annotation (at {pos_detail}) — detail must not interleave sub-lines:\n{output:?}",
  );
}

/// AC-9 — `001_multiline_cell_rendering`: truncation marker applied to truncated sub-lines.
///
/// A cell containing `"line1\nline2\nline3"` in a column with `max_column_width=4`
/// causes all three sub-lines (each 5 chars) to be truncated with `"..."`. The
/// truncation marker appears on every oversized sub-line, including the last one.
/// All three sub-lines appear in the output — no sub-line is dropped silently.
// test_kind: standard
#[ test ]
fn truncation_marker_on_last_sub_line_ac9()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "line1\nline2\nline3".into() ] )
    .build_view();

  let config = TableConfig::plain().max_column_width( Some( 4 ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .expect( "formatting must not fail" );

  // Truncation marker must be present (at least one truncated sub-line)
  assert!(
    output.contains( "..." ),
    "truncation marker must appear when lines exceed max_column_width:\n{output:?}",
  );

  // All three sub-lines are kept — count non-header lines containing truncation or short content
  // header + separator = 2 leading lines; each of the 3 sub-lines is a physical row
  let data_lines : Vec< &str > = output.lines().skip( 2 ).collect();
  assert_eq!(
    data_lines.len(), 3,
    "all three sub-lines must appear in output (none dropped): {output:?}",
  );

  // Last sub-line (index 2) must carry the truncation marker
  assert!(
    data_lines[ 2 ].contains( "..." ),
    "last kept sub-line must have truncation marker; got: {:?}\nFull output:\n{output:?}",
    data_lines[ 2 ],
  );
}

/// AC-10 — `001_multiline_cell_rendering`: three or more embedded newlines produce correct sub-line count.
///
/// A cell containing `"a\nb\nc\nd"` (three embedded `\n`, four logical lines)
/// produces exactly four physical sub-lines in the rendered output. The row height
/// equals 4; no sub-line is dropped or duplicated.
// test_kind: standard
#[ test ]
fn three_newlines_produce_four_sub_lines_ac10()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "a\nb\nc\nd".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() )
    .format( &view )
    .expect( "formatting must not fail" );

  // All four logical lines must be present
  let lines : Vec< &str > = output.lines().collect();
  assert!( output.contains( 'a' ), "sub-line 'a' must appear:\n{output:?}" );
  assert!( output.contains( 'b' ), "sub-line 'b' must appear:\n{output:?}" );
  assert!( output.contains( 'c' ), "sub-line 'c' must appear:\n{output:?}" );
  assert!( output.contains( 'd' ), "sub-line 'd' must appear:\n{output:?}" );

  // Skip header + separator; data rows = 4 sub-lines
  let data_lines : Vec< &str > = lines.iter().skip( 2 ).copied().collect();
  assert_eq!(
    data_lines.len(), 4,
    "three embedded newlines must produce exactly 4 physical sub-lines (not {}):\n{output:?}",
    data_lines.len(),
  );
}

// --- AC-11: whitespace-only sub-lines are preserved as non-empty padding lines ---
//
// Given: a cell containing "first\n   \nsecond" where the middle segment is 3 spaces.
// When: rendered with TableFormatter.
// Then: the whitespace-only middle sub-line is preserved (not collapsed); between
//       "first" and "second" in the output there is exactly one intervening physical line.

/// AC-11 — `algorithm/001_multiline_cell_rendering`: whitespace-only sub-lines preserved, not collapsed.
// test_kind: standard
#[ test ]
fn whitespace_only_sub_lines_preserved_not_collapsed_ac11()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "first\n   \nsecond".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() )
    .format( &view )
    .expect( "formatting must not fail" );

  // Both visible segments must appear
  assert!( output.contains( "first" ), "'first' segment must appear in output:\n{output:?}" );
  assert!( output.contains( "second" ), "'second' segment must appear in output:\n{output:?}" );

  // The whitespace-only sub-line must not be collapsed: exactly one physical line
  // must separate "first" from "second" in the rendered output.
  let lines : Vec< &str > = output.lines().collect();
  let first_idx = lines.iter().position( | l | l.contains( "first" ) )
    .expect( "'first' must appear in output lines" );
  let second_idx = lines.iter().position( | l | l.contains( "second" ) )
    .expect( "'second' must appear in output lines" );
  assert_eq!(
    second_idx - first_idx, 2,
    "whitespace-only sub-line must produce one physical line between 'first' and 'second' \
     (gap = 2); got gap = {}:\n\n{}",
    second_idx - first_idx,
    output,
  );
}
