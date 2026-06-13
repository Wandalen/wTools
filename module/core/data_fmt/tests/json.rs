//! Integration tests for `JsonFormatter`
//!
//! Validates JSON array-of-objects output in both pretty and compact modes,
//! empty table handling, builder pattern, and round-trip parse correctness.

#![ cfg( feature = "enabled" ) ]
#[ cfg( feature = "format_json" ) ]
mod json_tests
{
  use std::collections::HashMap;
  use data_fmt::{ RowBuilder, TableMetadata, TableView, JsonFormatter, Format };

  #[ test ]
  fn json_formatter_pretty()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = JsonFormatter::new();
    let json = formatter.format( &view ).unwrap();

    // Should be pretty-printed (contains newlines)
    assert!( json.contains( '\n' ), "expected newlines in pretty output:\n{json}" );

    // Should be array of objects format
    assert!( json.starts_with( '[' ), "expected '[' at start of output:\n{json}" );
    assert!( json.trim().ends_with( ']' ), "expected ']' at end of output:\n{json}" );

    // Should contain column names as keys
    assert!( json.contains( "\"Name\"" ), "expected key \"Name\" in output:\n{json}" );
    assert!( json.contains( "\"Age\"" ), "expected key \"Age\" in output:\n{json}" );

    // Should contain data
    assert!( json.contains( "\"Alice\"" ), "expected value \"Alice\" in output:\n{json}" );
    assert!( json.contains( "\"30\"" ), "expected value \"30\" in output:\n{json}" );
    assert!( json.contains( "\"Bob\"" ), "expected value \"Bob\" in output:\n{json}" );
    assert!( json.contains( "\"25\"" ), "expected value \"25\" in output:\n{json}" );
  }

  #[ test ]
  fn json_formatter_compact()
  {
    let view = RowBuilder::new( vec![ "A".into() ] )
      .add_row( vec![ "1".into() ] )
      .build_view();

    let formatter = JsonFormatter::compact();
    let json = formatter.format( &view ).unwrap();

    // Compact format should have no newlines
    let newline_count = json.chars().filter( | c | *c == '\n' ).count();
    assert!( newline_count == 0, "Compact format should have no newlines" );

    // Should be array format: [{"A":"1"}]
    assert!( json.starts_with( '[' ), "expected '[' at start of compact output:\n{json}" );
    assert!( json.ends_with( ']' ), "expected ']' at end of compact output:\n{json}" );
    assert!( json.contains( "\"A\"" ), "expected key \"A\" in compact output:\n{json}" );
    assert!( json.contains( "\"1\"" ), "expected value \"1\" in compact output:\n{json}" );
  }

  #[ test ]
  fn json_formatter_empty_table()
  {
    let view = TableView::new(
      TableMetadata::new( vec![ "Column".to_string() ] ),
      vec![]
    );

    let formatter = JsonFormatter::new();
    let json = formatter.format( &view ).unwrap();

    // Empty table should produce empty array
    assert_eq!( json.trim(), "[]" );
  }

  #[ test ]
  fn json_formatter_builder_pattern()
  {
    let formatter = JsonFormatter::new()
      .with_pretty( false );

    assert!( !formatter.pretty, "expected pretty=false after with_pretty(false)" );
  }

  #[ test ]
  fn json_formatter_output_structure()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = JsonFormatter::new();
    let json_str = formatter.format( &view ).unwrap();

    // Parse back to verify structure
    let parsed : Vec< HashMap< String, String > > = serde_json::from_str( &json_str )
      .expect( "Output should be valid JSON array of objects" );

    assert_eq!( parsed.len(), 2 );
    assert_eq!( parsed[ 0 ][ "Name" ], "Alice" );
    assert_eq!( parsed[ 0 ][ "Age" ], "30" );
    assert_eq!( parsed[ 1 ][ "Name" ], "Bob" );
    assert_eq!( parsed[ 1 ][ "Age" ], "25" );
  }
}
