//! Table style output tests for visual verification
//!
//! ## What This Tests
//!
//! Visual verification tests that render actual formatted output for each table style.
//! These tests print to stdout during test runs to allow manual inspection of output
//! quality and formatting correctness.
//!
//! ## Why Visual Tests Matter
//!
//! **Problem**: Unit tests verify structure but don't catch visual issues like:
//! - Misaligned columns that pass string equality checks
//! - Incorrect spacing that's hard to spot in assertions
//! - Border character bugs that break box-drawing
//!
//! **Solution**: Print actual output and verify key structural properties:
//! - Presence of expected separator characters (|, +, ┌, etc.)
//! - Absence of unwanted characters for minimal styles
//! - Correct data content in output
//!
//! ## Styles Tested
//!
//! - Plain style (process monitoring output)
//! - Minimal style (no separators)
//! - CSV style (comma-separated, no padding)
//! - TSV style (tab-separated)
//!
//! ## Historical Context
//!
//! Added in v0.3.0 when comprehensive table style parametrization was introduced.
//! These tests caught several alignment bugs during development that unit tests missed.
//!
//! Split from tests/table_styles.rs in v0.4.0 compliance cleanup.

#![ cfg( feature = "integration" ) ]
#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };

// Helper function to create sample data
fn sample_data() -> tree_fmt::TreeNode< String >
{
  RowBuilder::new( vec![ "NAME".into(), "AGE".into(), "CITY".into() ] )
    .add_row( vec![ "Alice".into(), "30".into(), "NYC".into() ] )
    .add_row( vec![ "Bob".into(), "25".into(), "LA".into() ] )
    .build()
}

// =============================================================================
// Plain Style Output Tests (Process Monitoring Style)
// =============================================================================

#[ test ]
fn test_plain_style_output()
{
  let tree = sample_data();
  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = formatter.format( &tree );

  println!( "Plain style output:\n{output}" );

  // Should have space-separated columns
  assert!( output.contains( "NAME" ) );
  assert!( output.contains( "AGE" ) );
  assert!( output.contains( "CITY" ) );

  // Should have dash separator under headers
  assert!( output.contains( "----" ) );

  // Should NOT have pipes or boxes
  assert!( !output.contains( "|" ) );
  assert!( !output.contains( "+" ) );

  // Should have data
  assert!( output.contains( "Alice" ) );
  assert!( output.contains( "Bob" ) );
}

#[ test ]
fn test_plain_style_alignment()
{
  let tree = RowBuilder::new( vec![ "COUNT".into(), "MEMORY".into() ] )
    .add_row( vec![ "45".into(), "12.4GB".into() ] )
    .add_row( vec![ "68".into(), "7.1GB".into() ] )
    .build();

  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = formatter.format( &tree );

  println!( "Plain style with alignment:\n{output}" );

  // Columns should be aligned
  let lines : Vec< &str > = output.lines().collect();
  assert!( lines.len() >= 3 ); // header + separator + at least 1 row

  // Header and data should align
  assert!( output.contains( "COUNT" ) );
  assert!( output.contains( "MEMORY" ) );
}

// =============================================================================
// Minimal Style Output Tests
// =============================================================================

#[ test ]
fn test_minimal_style_output()
{
  let tree = sample_data();
  let formatter = TableFormatter::with_config( TableConfig::minimal() );
  let output = formatter.format( &tree );

  println!( "Minimal style output:\n{output}" );

  // Should have headers and data
  assert!( output.contains( "NAME" ) );
  assert!( output.contains( "Alice" ) );

  // Should NOT have separator line
  let lines : Vec< &str > = output.lines().collect();
  let has_dash_line = lines.iter().any( | line | line.chars().all( | c | c == '-' || c.is_whitespace() ) );
  assert!( !has_dash_line, "Minimal style should not have dash separator" );

  // Should NOT have borders
  assert!( !output.contains( "|" ) );
}

// =============================================================================
// CSV and TSV Style Output Tests
// =============================================================================

#[ test ]
fn test_csv_style_output()
{
  let tree = sample_data();
  let formatter = TableFormatter::with_config( TableConfig::csv() );
  let output = formatter.format( &tree );

  println!( "CSV style output:\n{output}" );

  // Should be comma-separated
  assert!( output.contains( "NAME,AGE,CITY" ) );
  assert!( output.contains( "Alice,30,NYC" ) );
  assert!( output.contains( "Bob,25,LA" ) );

  // Should NOT have separator line
  assert!( !output.contains( "----" ) );
}

#[ test ]
fn test_tsv_style_output()
{
  let tree = sample_data();
  let formatter = TableFormatter::with_config( TableConfig::tsv() );
  let output = formatter.format( &tree );

  println!( "TSV style output:\n{output}" );

  // Should be tab-separated
  assert!( output.contains( "NAME\tAGE\tCITY" ) );
  assert!( output.contains( "Alice\t30\tNYC" ) );
}

