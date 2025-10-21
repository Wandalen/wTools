//! Tests for fluent builder APIs and config builder patterns

#![ cfg( feature = "integration" ) ]
#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::
{
  RowBuilder,
  formatters::TableShapedFormatter,
  TableFormatter, ExpandedFormatter,
};

// =============================================================================
// Fluent RowBuilder API Tests
// =============================================================================

#[ test ]
fn test_fluent_builder_single_chain()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build();

  assert_eq!( tree.children.len(), 1 );
  assert_eq!( tree.children[ 0 ].name, "1" );
}

#[ test ]
fn test_fluent_builder_multiple_chains()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .add_row( vec![ "3".into(), "4".into() ] )
    .add_row( vec![ "5".into(), "6".into() ] )
    .build();

  assert_eq!( tree.children.len(), 3 );
  assert_eq!( tree.children[ 0 ].name, "1" );
  assert_eq!( tree.children[ 1 ].name, "2" );
  assert_eq!( tree.children[ 2 ].name, "3" );
}

#[ test ]
fn test_fluent_builder_with_custom_names()
{
  let tree = RowBuilder::new( vec![ "Value".into() ] )
    .add_row_with_name( "first".into(), vec![ "100".into() ] )
    .add_row_with_name( "second".into(), vec![ "200".into() ] )
    .build();

  assert_eq!( tree.children.len(), 2 );
  assert_eq!( tree.children[ 0 ].name, "first" );
  assert_eq!( tree.children[ 1 ].name, "second" );
}

#[ test ]
fn test_fluent_builder_mixed_with_mut()
{
  let mut builder = RowBuilder::new( vec![ "X".into() ] );
  builder.add_row_mut( vec![ "1".into() ] );
  builder.add_row_mut( vec![ "2".into() ] );
  let tree = builder.build();

  assert_eq!( tree.children.len(), 2 );
}

#[ test ]
fn test_fluent_builder_with_formatter_trait()
{
  // Build tree fluently
  let tree = RowBuilder::new( vec![ "Language".into(), "Year".into() ] )
    .add_row( vec![ "Rust".into(), "2015".into() ] )
    .add_row( vec![ "Python".into(), "1991".into() ] )
    .build();

  // Format using trait polymorphism
  let formatters : Vec< Box< dyn TableShapedFormatter > > = vec![
    Box::new( TableFormatter::new() ),
    Box::new( ExpandedFormatter::new() ),
  ];

  for formatter in formatters
  {
    let output = formatter.format( &tree );
    assert!( output.contains( "Rust" ) );
    assert!( output.contains( "2015" ) );
    assert!( output.contains( "Python" ) );
    assert!( output.contains( "1991" ) );
  }
}

// =============================================================================
// Config Builder Pattern Tests
// =============================================================================

#[ test ]
fn test_tree_config_builder()
{
  use tree_fmt::{ TreeConfig, TreeFormatter, TreeBuilder };

  let config = TreeConfig::new()
    .show_branches( false )
    .show_root( true )
    .indent_size( 2 )
    .max_depth( Some( 3 ) );

  let tree = TreeBuilder::new( "root" )
    .insert( &[ "dir", "file.txt" ], 100 )
    .build();

  let formatter = TreeFormatter::with_config( config );
  let output = formatter.format( &tree, | n | format!( "{}", n ) );

  assert!( output.contains( "root" ) );
}

#[ test ]
fn test_table_config_builder()
{
  use tree_fmt::{ TableConfig, TableFormatter, RowBuilder };

  let config = TableConfig::new()
    .show_borders( false )
    .column_widths( vec![ 10, 15 ] )
    .align_right( vec![ false, true ] );

  let tree = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ "Item".into(), "123".into() ] )
    .build();

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &tree );

  assert!( output.contains( "Item" ) );
  assert!( !output.contains( '|' ) ); // No borders
}

#[ test ]
fn test_expanded_config_builder()
{
  use tree_fmt::{ ExpandedConfig, ExpandedFormatter, RowBuilder };

  let config = ExpandedConfig::new()
    .record_separator( "--- Record {} ---".into() )
    .key_value_separator( " = ".into() );

  let tree = RowBuilder::new( vec![ "Key".into() ] )
    .add_row( vec![ "Value".into() ] )
    .build();

  let formatter = ExpandedFormatter::with_config( config );
  let output = formatter.format( &tree );

  assert!( output.contains( "--- Record" ) );
  assert!( output.contains( " = " ) );
}
