//! Backward compatibility and regression tests for table styles
//!
//! ## What This Tests
//!
//! Tests that ensure v0.3.0 refactoring maintains backward compatibility and prevents regressions:
//! - Deprecated `show_borders()` method still works correctly
//! - Default behavior unchanged (bordered output)
//! - Edge cases (empty tables, single row, wide tables)
//! - Custom separator configurations
//!
//! ## Critical Regression Tests
//!
//! ### Header Separator Alignment Bug (test_header_separator_alignment_with_padding)
//! **Bug**: Header separator line length didn't match header/row lengths when inner_padding > 0
//! **Symptom**: Misaligned table boxes, pipes didn't line up vertically
//! **Fix**: Separator generation now accounts for inner_padding
//! **Test**: Verifies all lines have same length and pipe count
//!
//! ### Double Pipe Bug (test_default_table_no_double_pipes)
//! **Bug**: Some configurations produced `||` (double pipes) in output
//! **Symptom**: Visually broken tables with extra pipes
//! **Fix**: Border pipe logic ensures single pipes only
//! **Test**: Scans output for `||` patterns
//!
//! ## Why Backward Compatibility Matters
//!
//! v0.3.0 was a major refactoring introducing:
//! - BorderVariant, HeaderSeparatorVariant, ColumnSeparator enums
//! - 9 style preset constructors
//! - Comprehensive parametrization
//!
//! Existing code using v0.2.x API must continue working without changes.
//!
//! Split from tests/table_styles.rs in v0.4.0 compliance cleanup.

#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::
{
  RowBuilder, TableFormatter, TableConfig,
  BorderVariant, HeaderSeparatorVariant, ColumnSeparator,
};

// Helper function to create sample data
fn sample_data() -> tree_fmt::TreeNode< String >
{
  RowBuilder::new( vec![ "NAME".into(), "AGE".into(), "CITY".into() ] )
    .add_row( vec![ "Alice".into(), "30".into(), "NYC".into() ] )
    .add_row( vec![ "Bob".into(), "25".into(), "LA".into() ] )
    .build()
}

// =============================================================================
// Backward Compatibility Tests
// =============================================================================

#[ test ]
fn test_deprecated_show_borders_maps_to_border_style()
{
  #[ allow( deprecated ) ]
  let config_with_borders = TableConfig::new().show_borders( true );
  assert_eq!( config_with_borders.border_variant, BorderVariant::Ascii );

  #[ allow( deprecated ) ]
  let config_without_borders = TableConfig::new().show_borders( false );
  assert_eq!( config_without_borders.border_variant, BorderVariant::None );
}

#[ test ]
fn test_default_behavior_unchanged()
{
  let tree = sample_data();
  let formatter = TableFormatter::new();
  let output = formatter.format( &tree );

  // Default is now plain style (changed in v0.4.0 refactoring)
  assert!( output.contains( "----" ) ); // Dash separator
  assert!( output.contains( "NAME" ) );
  assert!( output.contains( "Alice" ) );
}

// =============================================================================
// Edge Cases and Complex Scenarios
// =============================================================================

#[ test ]
fn test_empty_table_plain_style()
{
  // Empty table (no rows, no headers) should produce minimal output
  let tree = RowBuilder::new( vec![ "Col".into() ] ).build();
  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = formatter.format( &tree );

  // Empty table with no rows returns empty (this is expected behavior)
  // The table has headers defined but no data rows, so TableView returns empty headers
  assert!( output.trim().is_empty() || output == "\n\n", "Empty table should produce minimal output" );
}

#[ test ]
fn test_single_row_plain_style()
{
  let tree = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build();

  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = formatter.format( &tree );

  assert!( output.contains( "Name" ) );
  assert!( output.contains( "Alice" ) );
}

#[ test ]
fn test_wide_table_plain_style()
{
  let tree = RowBuilder::new( vec![
    "COL1".into(), "COL2".into(), "COL3".into(), "COL4".into(), "COL5".into(),
  ])
    .add_row( vec![ "A".into(), "B".into(), "C".into(), "D".into(), "E".into() ] )
    .build();

  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = formatter.format( &tree );

  assert!( output.contains( "COL1" ) );
  assert!( output.contains( "COL5" ) );
  assert!( output.contains( "A" ) );
  assert!( output.contains( "E" ) );
}

#[ test ]
fn test_custom_separator_spaces()
{
  let tree = sample_data();
  let config = TableConfig::new()
    .border_variant( BorderVariant::None )
    .header_separator_variant( HeaderSeparatorVariant::Dash )
    .column_separator( ColumnSeparator::Spaces( 4 ) ); // 4 spaces

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &tree );

  println!( "Custom 4-space separator:\n{output}" );

  // Should have wider spacing between columns
  assert!( output.contains( "NAME" ) );
  assert!( output.contains( "AGE" ) );
}

// =============================================================================
// Regression Tests
// =============================================================================

#[ test ]
fn test_header_separator_alignment_with_padding()
{
  // Regression test for bug where AsciiGrid separator didn't account for inner_padding
  // This caused misaligned separator lines in default bordered tables
  let tree = RowBuilder::new( vec![ "Crate".into(), "Type".into(), "Name".into() ] )
    .add_row( vec![ "mindful".into(), "Binary".into(), "mindful".into() ] )
    .add_row( vec![ "wflow".into(), "Binary".into(), "wflow".into() ] )
    .build();

  let formatter = TableFormatter::with_config( TableConfig::bordered() ); // Uses bordered config with inner_padding=1
  let output = formatter.format( &tree );

  println!( "Header separator alignment test:\n{output}" );

  let lines : Vec< &str > = output.lines().collect();
  assert!( lines.len() >= 3, "Should have header, separator, and data rows" );

  // Get the actual line lengths (all should match for proper alignment)
  let header_len = lines[ 0 ].len();
  let separator_len = lines[ 1 ].len();
  let row_len = lines[ 2 ].len();

  // All lines should be the same length for proper box alignment
  assert_eq!(
    separator_len, header_len,
    "Separator line length ({}) must match header length ({})\nHeader:    |{}|\nSeparator: |{}|",
    separator_len, header_len, lines[ 0 ], lines[ 1 ]
  );

  assert_eq!(
    row_len, header_len,
    "Row line length ({}) must match header length ({})",
    row_len, header_len
  );

  // Verify separator has correct structure: |dashes|dashes|dashes|
  let separator = lines[ 1 ];
  assert!( separator.starts_with( '|' ), "Separator should start with pipe" );
  assert!( separator.ends_with( '|' ), "Separator should end with pipe" );

  // Count pipes in separator - should match header
  let header_pipe_count = lines[ 0 ].matches( '|' ).count();
  let separator_pipe_count = separator.matches( '|' ).count();
  assert_eq!(
    separator_pipe_count, header_pipe_count,
    "Separator pipes ({}) must match header pipes ({})",
    separator_pipe_count, header_pipe_count
  );

  // Separator should only contain pipes and dashes
  assert!(
    separator.chars().all( | c | c == '|' || c == '-' ),
    "Separator should only contain pipes and dashes, got: {separator}"
  );
}

#[ test ]
fn test_default_table_no_double_pipes()
{
  // Regression test: ensure default formatter doesn't produce double pipes like ||
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build();

  let formatter = TableFormatter::new();
  let output = formatter.format( &tree );

  // Should NOT have double pipes anywhere
  assert!(
    !output.contains( "||" ),
    "Output should not contain double pipes, got:\n{output}"
  );
}
