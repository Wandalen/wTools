//! `TableFormatter`: Horizontal tabular display (rows × columns)
//!
//! Use when: Spreadsheet-like data, reports, dashboards, comparing rows side-by-side
//! Output: Traditional table with borders and aligned columns
//!
//! NOTE: `TableFormatter` also implements the `Format` trait for format-agnostic code.
//! See `unified_formats.rs` example for usage with the unified interface.

use tree_fmt::{ RowBuilder, TableFormatter };

fn main()
{
  println!( "=== TableFormatter: Horizontal Tabular Display ===" );
  println!( "Best for: Spreadsheet-like data, comparing multiple rows\n" );

  // Create sample data: Product inventory
  let mut builder = RowBuilder::new( vec![ "Product".into(), "Stock".into(), "Price".into() ] );
  builder.add_row_mut( vec![ "Laptop".into(), "23".into(), "$899".into() ] );
  builder.add_row_mut( vec![ "Mouse".into(), "156".into(), "$15".into() ] );
  builder.add_row_mut( vec![ "Keyboard".into(), "89".into(), "$45".into() ] );
  builder.add_row_mut( vec![ "Monitor".into(), "34".into(), "$299".into() ] );
  let tree = builder.build();

  // Render as table
  let formatter = TableFormatter::new();
  let output = formatter.format( &tree );

  println!( "{output}" );

  println!( "\n✓ Characteristics:" );
  println!( "  • Horizontal layout - easy to scan across rows" );
  println!( "  • Borders separate cells clearly" );
  println!( "  • Columns auto-align to content width" );
  println!( "  • Compact for comparing multiple records" );
}
