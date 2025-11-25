//! `ExpandedFormatter`: Vertical record display (`PostgreSQL` `\x` mode)
//!
//! Use when: Inspecting individual records, wide tables with many columns, detailed view
//! Output: Vertical key-value pairs with record separators
//!
//! NOTE: `ExpandedFormatter` also implements the `Format` trait for format-agnostic code.
//! See `unified_formats.rs` example for usage with the unified interface.

use tree_fmt::{ RowBuilder, ExpandedFormatter };

fn main()
{
  println!( "=== ExpandedFormatter: Vertical Record Display ===" );
  println!( "Best for: Database query inspection, detailed record view, wide tables\n" );

  // Create sample data: Server configuration with many fields
  let tree = RowBuilder::new( vec![
    "Hostname".into(),
    "IP".into(),
    "Region".into(),
    "Memory".into(),
    "CPU".into(),
    "Status".into()
  ])
  .add_row( vec![
    "web-prod-01".into(),
    "10.0.1.15".into(),
    "us-east".into(),
    "32GB".into(),
    "8 cores".into(),
    "active".into()
  ])
  .add_row( vec![
    "db-primary".into(),
    "10.0.2.42".into(),
    "eu-west".into(),
    "64GB".into(),
    "16 cores".into(),
    "active".into()
  ])
  .build();

  // Render as expanded records (PostgreSQL \x mode)
  let formatter = ExpandedFormatter::new();
  let output = formatter.format( &tree );

  println!( "{output}" );

  println!( "✓ Characteristics:" );
  println!( "  • Vertical layout - each field on separate line" );
  println!( "  • Record separators (-[ RECORD N ]) divide entries" );
  println!( "  • Aligned key-value pairs for readability" );
  println!( "  • Perfect for wide tables that dont fit horizontally" );
}
