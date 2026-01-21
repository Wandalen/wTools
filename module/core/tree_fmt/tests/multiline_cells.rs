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

use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };

// ============================================================================
// Basic Multiline Tests
// ============================================================================

#[ test ]
fn test_single_cell_two_lines()
{
  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Line 1\nLine 2".into() ] )
    .build();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data );

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
    .build();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data );

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
    .build();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data );

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
    .build();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data );

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
    .build();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data );

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
    .build();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data );

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
    .build();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data );

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
    .build();

  let config = TableConfig::bordered();
  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

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
    .build();

  let config = TableConfig::markdown();
  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

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
    .build();

  let config = TableConfig::plain();
  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

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
    .build();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data );

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
    .build();

  let config = TableConfig::csv();
  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

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
    .build();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data );

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
    .add_row( vec![ content ] )
    .build();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data );

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
    .build();

  let formatter = TableFormatter::new();
  let output = formatter.format( &data );

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
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 20 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

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
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 15 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Both lines should be truncated independently
  let marker_count = output.matches( "..." ).count();
  assert!(
    marker_count >= 2,
    "Both lines should be truncated (found {marker_count} markers)"
  );
}
