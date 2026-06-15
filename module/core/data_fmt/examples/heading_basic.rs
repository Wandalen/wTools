//! Minimal `Heading` demo: titled rule printed above a table
//!
//! Demonstrates `Heading::new()` and `TableConfig::with_heading()`
//! to prepend a titled horizontal rule to table output.

#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
fn main()
{
  use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Heading, Format };

  println!( "=== Heading: Basic Titled Rule ===" );
  println!( "A horizontal rule with a title appears above the table.\n" );

  let view = RowBuilder::new( vec![ "Name".into(), "Score".into() ] )
    .add_row( vec![ "Alice".into(), "42".into() ] )
    .add_row( vec![ "Bob".into(), "37".into() ] )
    .add_row( vec![ "Carol".into(), "91".into() ] )
    .build_view();

  let config = TableConfig::plain()
    .with_heading( Heading::new( "Results" ) );

  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  println!( "{output}" );
}
