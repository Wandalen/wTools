//! Integration tests for `YamlFormatter`
//!
//! Validates YAML list-of-objects output format, empty table handling,
//! special character handling, and round-trip parse correctness.

#![ cfg( feature = "enabled" ) ]
#[ cfg( feature = "format_yaml" ) ]
mod yaml_tests
{
  use std::collections::HashMap;
  use tree_fmt::{ RowBuilder, TableMetadata, TableView, YamlFormatter, Format };

  #[ test ]
  fn yaml_formatter_basic()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = YamlFormatter::new();
    let yaml = formatter.format( &view ).unwrap();

    // Should be YAML list (starts with -)
    assert!( yaml.contains( '-' ) );

    // Should contain column names as keys
    assert!( yaml.contains( "Name:" ) );
    assert!( yaml.contains( "Age:" ) );

    // Should contain data
    assert!( yaml.contains( "Alice" ) );
    assert!( yaml.contains( "'30'" ) || yaml.contains( "\"30\"" ) || yaml.contains( "30" ) );
    assert!( yaml.contains( "Bob" ) );

    // Should NOT have JSON brackets (YAML uses dashes for lists)
    assert!( !yaml.contains( '{' ) );
  }

  #[ test ]
  fn yaml_formatter_empty_table()
  {
    let view = TableView::new(
      TableMetadata::new( vec![ "Column".to_string() ] ),
      vec![]
    );

    let formatter = YamlFormatter::new();
    let yaml = formatter.format( &view ).unwrap();

    // Empty table should produce empty array: []
    assert!( yaml.trim() == "[]" );
  }

  #[ test ]
  fn yaml_formatter_special_characters()
  {
    let view = RowBuilder::new( vec![ "Key".into() ] )
      .add_row( vec![ "value: with colon".into() ] )
      .build_view();

    let formatter = YamlFormatter::new();
    let yaml = formatter.format( &view ).unwrap();

    // YAML should properly escape/quote strings with colons
    assert!( yaml.contains( "Key:" ) );
    assert!(
      yaml.contains( "value: with colon" )
      || yaml.contains( "'value: with colon'" )
      || yaml.contains( "\"value: with colon\"" )
    );
  }

  #[ test ]
  fn yaml_formatter_output_structure()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = YamlFormatter::new();
    let yaml_str = formatter.format( &view ).unwrap();

    // Parse back to verify structure
    let parsed : Vec< HashMap< String, String > > = serde_yaml::from_str( &yaml_str )
      .expect( "Output should be valid YAML list of objects" );

    assert_eq!( parsed.len(), 2 );
    assert_eq!( parsed[ 0 ][ "Name" ], "Alice" );
    assert_eq!( parsed[ 0 ][ "Age" ], "30" );
    assert_eq!( parsed[ 1 ][ "Name" ], "Bob" );
    assert_eq!( parsed[ 1 ][ "Age" ], "25" );
  }
}
