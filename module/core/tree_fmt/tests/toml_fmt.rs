//! Integration tests for `TomlFormatter`
//!
//! Validates TOML array-of-tables output format (`[[row]]` sections),
//! empty table handling, and round-trip parse correctness.

#![ cfg( feature = "enabled" ) ]
#[ cfg( feature = "format_toml" ) ]
mod toml_tests
{
  use std::collections::HashMap;
  use tree_fmt::{ RowBuilder, TableMetadata, TableView, TomlFormatter, Format };

  #[ test ]
  fn toml_formatter_basic()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = TomlFormatter::new();
    let toml_str = formatter.format( &view ).unwrap();

    // Should be TOML array of tables format: [[row]]
    assert!( toml_str.contains( "[[row]]" ) );

    // Should contain column names as keys
    assert!( toml_str.contains( "Name" ) );
    assert!( toml_str.contains( "Age" ) );

    // Should contain data
    assert!( toml_str.contains( "Alice" ) );
    assert!( toml_str.contains( "30" ) );
    assert!( toml_str.contains( "Bob" ) );
    assert!( toml_str.contains( "25" ) );
  }

  #[ test ]
  fn toml_formatter_empty_table()
  {
    let view = TableView::new(
      TableMetadata::new( vec![ "Column".to_string() ] ),
      vec![]
    );

    let formatter = TomlFormatter::new();
    let toml_str = formatter.format( &view ).unwrap();

    // Empty table should produce no [[row]] sections
    assert!( !toml_str.contains( "[[row]]" ) );
  }

  #[ test ]
  fn toml_formatter_output_structure()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = TomlFormatter::new();
    let toml_str = formatter.format( &view ).unwrap();

    // Parse back to verify structure
    #[ derive( serde::Deserialize ) ]
    struct TomlWrapper
    {
      row : Vec< HashMap< String, String > >,
    }

    let parsed : TomlWrapper = toml::from_str( &toml_str )
      .expect( "Output should be valid TOML with [[row]] tables" );

    assert_eq!( parsed.row.len(), 2 );
    assert_eq!( parsed.row[ 0 ][ "Name" ], "Alice" );
    assert_eq!( parsed.row[ 0 ][ "Age" ], "30" );
    assert_eq!( parsed.row[ 1 ][ "Name" ], "Bob" );
    assert_eq!( parsed.row[ 1 ][ "Age" ], "25" );
  }
}
