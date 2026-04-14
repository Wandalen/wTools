//! Column truncation tests for `TableFormatter`
//!
//! # Purpose
//!
//! Tests the `max_column_width` feature which truncates cell content exceeding
//! a specified visual width. Critical for displaying tables with potentially
//! unbounded content (e.g., log messages, descriptions, URLs) where horizontal
//! space is limited.
//!
//! # Why This Feature Exists
//!
//! **Problem**: Long cell content breaks table readability in terminals/reports
//! - Makes tables too wide to fit screen
//! - Forces horizontal scrolling
//! - Obscures other columns
//!
//! **Solution**: Truncate cells to maximum width with visual indicator (`...`)
//!
//! # Key Design Decisions
//!
//! 1. **ANSI-Aware Truncation**: Visual width excludes escape codes
//!    - Rationale: Color codes shouldn't count toward cell width
//!    - Implementation: Character-by-character scan preserving `\x1b[...m`
//!
//! 2. **Per-Line Truncation for Multiline Cells**: Each line truncated independently
//!    - Rationale: Better visual result than truncating whole cell before split
//!    - Example: "Long line 1\nShort" → "Long li...\nShort" (not "Long li...")
//!
//! 3. **Truncation Before Padding**: Applied in `format_row()` before alignment
//!    - Rationale: Padding should work on already-truncated content
//!    - Order: truncate → pad → render
//!
//! 4. **Disabled by Default**: `max_column_width = None`
//!    - Rationale: Backward compatibility - no behavior change without opt-in
//!    - Migration: Users explicitly set `max_column_width(Some(n))`
//!
//! # Known Edge Cases
//!
//! - **Empty marker**: Valid, produces truncation without indicator
//! - **Marker longer than limit**: Handled gracefully (`saturating_sub`)
//! - **Unicode characters**: Count correctly by visual width
//! - **CSV/TSV formats**: Truncation still applied (no special handling needed)
//! - **Exact fit**: Content at exact `max_width` is NOT truncated
//!
//! # Test Organization
//!
//! - Basic: Core truncation behavior (fits, exceeds, exact)
//! - ANSI: Color code preservation across truncation
//! - Edge Cases: Empty strings, tiny limits, unicode, marker edge cases
//! - Table Styles: Compatibility with all 9 formatter styles
//! - Multiple Columns: Interaction between columns of varying lengths
//! - Custom Markers: Non-default truncation indicators
//!
//! # How to Interpret Failures
//!
//! - "Should contain truncation marker": Truncation didn't occur when expected
//!   → Check `visual_len` calculation or `max_width` comparison
//!
//! - "Should not contain full text": Truncation occurred but didn't remove enough
//!   → Check `content_width` calculation (max - `marker_len`)
//!
//! - "Should preserve ANSI codes": Color codes were stripped
//!   → Check escape sequence detection in `truncate_single_line`
//!
//! - "Visual length should respect `max_column_width`": Width exceeded limit
//!   → Check that ANSI codes aren't counted in `visual_len`
//!
//! # Related Tests
//!
//! - `tests/multiline_cells.rs`: Multiline + truncation interaction
//! - `tests/table_styles_*.rs`: Table rendering without truncation
//! - Integration tests: Real-world usage with colored output

use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };

// ============================================================================
// Basic Truncation Tests
// ============================================================================

#[ test ]
fn test_truncate_long_cell_basic()
{
  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Very long content that definitely exceeds the limit".into() ] )
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 20 ) )
    .truncation_marker( "...".to_string() );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should contain truncation marker
  assert!( output.contains( "..." ), "Output should contain truncation marker" );

  // Should not contain full text
  assert!(
    !output.contains( "definitely exceeds the limit" ),
    "Output should not contain full text after truncation"
  );
}

#[ test ]
fn test_no_truncation_when_fits()
{
  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Short".into() ] )
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 20 ) )
    .truncation_marker( "...".to_string() );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should contain full text
  assert!( output.contains( "Short" ), "Output should contain full text" );

  // Should not contain truncation marker (text fits)
  let lines : Vec<&str> = output.lines().collect();
  let data_line = lines.iter().find( |l| l.contains( "Short" ) ).unwrap();
  assert!(
    !data_line.contains( "..." ),
    "Should not truncate when content fits"
  );
}

#[ test ]
fn test_truncation_exact_fit()
{
  // Content exactly at max width - should NOT truncate
  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Exactly20Characters!".into() ] )  // 20 chars
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 20 ) )
    .truncation_marker( "...".to_string() );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should contain full text
  assert!( output.contains( "Exactly20Characters!" ), "Exact fit should not truncate" );
}

#[ test ]
fn test_truncation_one_over_limit()
{
  // Content one character over limit - should truncate
  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "TwentyOneCharacters!!".into() ] )  // 21 chars
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 20 ) )
    .truncation_marker( "...".to_string() );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should be truncated
  assert!( output.contains( "..." ), "One over should truncate" );
  assert!( !output.contains( "TwentyOneCharacters!!" ), "Full text should not appear" );
}

// ============================================================================
// ANSI Color Code Tests
// ============================================================================

#[ test ]
fn test_truncation_preserves_ansi_codes()
{
  // Content with ANSI color codes
  let colored = "\x1b[31mRed colored text that is very long\x1b[0m";

  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ colored.into() ] )
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 15 ) )
    .truncation_marker( "...".to_string() );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should contain ANSI codes (color preserved)
  assert!( output.contains( "\x1b[31m" ), "Should preserve ANSI color codes" );
  assert!( output.contains( "..." ), "Should contain truncation marker" );

  // Visual length should be respected (not counting ANSI codes)
  let lines : Vec<&str> = output.lines().collect();
  let data_line = lines.iter().find( |l| l.contains( "Red" ) ).unwrap();

  // Count visual characters (excluding ANSI)
  let visual_len = tree_fmt::visual_len( data_line.trim() );

  // Should be around 15-18 (15 content + 3 marker, plus column padding)
  assert!(
    (15..=25).contains( &visual_len ),
    "Visual length should respect max_column_width (got {visual_len})"
  );
}

#[ test ]
fn test_truncation_multiple_ansi_codes()
{
  // Multiple color codes in one cell
  let colored = "\x1b[31mRed\x1b[0m\x1b[34mBlue\x1b[0m\x1b[32mGreen text here\x1b[0m";

  let data = RowBuilder::new( vec![ "Colors".into() ] )
    .add_row( vec![ colored.into() ] )
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 15 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should contain truncation
  assert!( output.contains( "..." ), "Should truncate multicolor text" );

  // Should preserve at least some color codes
  assert!(
    output.contains( "\x1b[" ),
    "Should preserve ANSI codes in truncated output"
  );
}

// ============================================================================
// Edge Cases
// ============================================================================

#[ test ]
fn test_truncation_empty_string()
{
  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ String::new() ] )
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 20 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should handle empty string gracefully
  assert!( !output.contains( "..." ), "Empty string should not show marker" );
}

#[ test ]
fn test_truncation_marker_longer_than_limit()
{
  // Edge case: marker itself is longer than max width
  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Some text".into() ] )
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 5 ) )
    .truncation_marker( "...TRUNCATED".to_string() );  // 12 chars, longer than limit

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should handle gracefully (either show just marker or empty)
  // The exact behavior is implementation-defined, but shouldn't panic
  assert!( !output.is_empty(), "Should produce some output" );
}

#[ test ]
fn test_truncation_very_small_limit()
{
  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Text".into() ] )
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 1 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should handle very small limits
  assert!( output.contains( "..." ), "Should truncate with small limit" );
}

#[ test ]
fn test_truncation_unicode_characters()
{
  // Unicode characters should count as visual width correctly
  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Hello 世界 and more text here".into() ] )
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 15 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should truncate (total visual length > 15)
  assert!( output.contains( "..." ), "Should truncate unicode text" );
}

// ============================================================================
// Table Style Compatibility Tests
// ============================================================================

#[ test ]
fn test_truncation_with_bordered_style()
{
  let data = RowBuilder::new( vec![ "Name".into(), "Description".into() ] )
    .add_row( vec![
      "Alice".into(),
      "Very long description that should be truncated".into()
    ])
    .build();

  let config = TableConfig::bordered()
    .max_column_width( Some( 20 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should contain borders and truncation
  assert!( output.contains( '|' ), "Should have borders" );
  assert!( output.contains( "..." ), "Should have truncation" );
  assert!( output.contains( "Alice" ), "Should contain non-truncated cell" );
}

#[ test ]
fn test_truncation_with_markdown_style()
{
  let data = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![
      "Item".into(),
      "Very long value text here that exceeds limit".into()
    ])
    .build();

  let config = TableConfig::markdown()
    .max_column_width( Some( 20 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should be valid markdown with truncation
  assert!( output.contains( '|' ), "Should have markdown pipes" );
  assert!( output.contains( "..." ), "Should have truncation" );
}

#[ test ]
fn test_truncation_with_grid_style()
{
  let data = RowBuilder::new( vec![ "Column".into() ] )
    .add_row( vec![ "Long text that needs truncation here".into() ] )
    .build();

  let config = TableConfig::grid()
    .max_column_width( Some( 20 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should have grid-style borders (pipes and dashes) and truncation
  assert!( output.contains( '|' ), "Should have grid pipes" );
  assert!( output.contains( "---" ), "Should have header separator dashes" );
  assert!( output.contains( "..." ), "Should have truncation" );
}

#[ test ]
fn test_truncation_with_csv_style()
{
  let data = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![
      "Item".into(),
      "Very long value that should be truncated".into()
    ])
    .build();

  let config = TableConfig::csv()
    .max_column_width( Some( 20 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // CSV should still work with truncation
  assert!( output.contains( ',' ), "Should have CSV commas" );
  assert!( output.contains( "..." ), "Should have truncation" );
}

// ============================================================================
// Multiple Column Tests
// ============================================================================

#[ test ]
fn test_truncation_multiple_columns()
{
  let data = RowBuilder::new( vec![ "Col1".into(), "Col2".into(), "Col3".into() ] )
    .add_row( vec![
      "Short".into(),
      "This is a very long text in column 2".into(),
      "Another long text in column 3 here".into()
    ])
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 15 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Both long columns should be truncated
  let truncation_count = output.matches( "..." ).count();
  assert!(
    truncation_count >= 2,
    "Should truncate both long columns (found {truncation_count} markers)"
  );

  // Short column should not be truncated
  assert!( output.contains( "Short" ), "Short column should remain intact" );
}

#[ test ]
fn test_truncation_headers_and_data()
{
  let data = RowBuilder::new( vec![
    "Very long header text that exceeds limit".into(),
    "Short".into()
  ])
    .add_row( vec![
      "Data".into(),
      "Very long data text here".into()
    ])
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 15 ) );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Both header and data should be truncated where needed
  let lines : Vec<&str> = output.lines().collect();

  // Header line should have truncation
  let header_line = lines[ 0 ];
  assert!( header_line.contains( "..." ), "Long header should be truncated" );

  // Data line should have truncation
  let data_line = lines.iter().find( |l| l.contains( "Data" ) ).unwrap();
  assert!( data_line.contains( "..." ), "Long data should be truncated" );
}

// ============================================================================
// Custom Marker Tests
// ============================================================================

#[ test ]
fn test_truncation_custom_marker()
{
  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Very long content here that will be truncated".into() ] )
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 20 ) )
    .truncation_marker( " [more]".to_string() );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should use custom marker
  assert!( output.contains( "[more]" ), "Should use custom truncation marker" );
  assert!( !output.contains( "..." ), "Should not use default marker" );
}

#[ test ]
fn test_truncation_empty_marker()
{
  let data = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Very long content that exceeds the width limit here".into() ] )
    .build();

  let config = TableConfig::plain()
    .max_column_width( Some( 20 ) )
    .truncation_marker( String::new() );

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &data );

  // Should truncate without marker
  assert!(
    !output.contains( "width limit here" ),
    "Content should be truncated"
  );

  // Visual length should be around 20 (plus padding)
  let lines : Vec<&str> = output.lines().collect();
  let data_line = lines.iter().find( |l| l.contains( "Very" ) ).unwrap();
  let visual_len = tree_fmt::visual_len( data_line.trim() );

  assert!(
    (20..=25).contains( &visual_len ),
    "Should respect max width even without marker (got {visual_len})"
  );
}
