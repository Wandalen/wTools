//! Multi-style `Heading` demo: title + fields across border styles
//!
//! Demonstrates three heading combinations:
//! 1. Title only with plain borders
//! 2. Title + one field with bordered style
//! 3. Title + two fields with grid style

#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
fn main()
{
  use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Heading, Format };

  let view = RowBuilder::new( vec![ "Service".into(), "Status".into(), "Latency".into() ] )
    .add_row( vec![ "auth".into(), "OK".into(), "12ms".into() ] )
    .add_row( vec![ "api-gw".into(), "OK".into(), "34ms".into() ] )
    .add_row( vec![ "worker".into(), "WARN".into(), "340ms".into() ] )
    .build_view();

  // --- 1. Title only, plain style ---
  println!( "=== 1. Title only (plain) ===" );
  let config = TableConfig::plain()
    .with_heading( Heading::new( "Health Check" ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();
  println!( "{output}\n" );

  // --- 2. Title + one field, bordered style ---
  println!( "=== 2. Title + field (bordered) ===" );
  let heading = Heading::new( "Dashboard" )
    .with_field( "3 services" );
  let config = TableConfig::bordered()
    .with_heading( heading );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();
  println!( "{output}\n" );

  // --- 3. Title + two fields, grid style ---
  println!( "=== 3. Title + two fields (grid) ===" );
  let heading = Heading::new( "Needs Review" )
    .with_field( "3 services" )
    .with_field( "1 warning" );
  let config = TableConfig::grid()
    .with_heading( heading );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();
  println!( "{output}" );
}
