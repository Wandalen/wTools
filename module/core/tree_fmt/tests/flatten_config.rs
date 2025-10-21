//! Tests for FlattenConfig functionality

#![ cfg( feature = "integration" ) ]
#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::
{
  TreeBuilder, TableShapedView,
  conversions::{ flatten_to_table_tree_with_config, FlattenConfig },
  TableFormatter,
};

// =============================================================================
// FlattenConfig Tests
// =============================================================================

#[ test ]
fn test_flatten_config_column_selection()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "dir", "file.txt" ], 100 )
    .build();

  // Only name and data columns
  let config = FlattenConfig::new()
    .include_path( false )
    .include_depth( false );

  let flattened = flatten_to_table_tree_with_config( &tree, &config );
  let headers = flattened.extract_headers().unwrap();

  assert_eq!( headers, vec![ "name", "data" ] );
  assert_eq!( flattened.children.len(), 3 ); // root, dir, file.txt
}

#[ test ]
fn test_flatten_config_custom_column_names()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "file.txt" ], 100 )
    .build();

  let config = FlattenConfig::new()
    .column_names( "Full Path".into(), "File Name".into(), "Level".into(), "Size".into() );

  let flattened = flatten_to_table_tree_with_config( &tree, &config );
  let headers = flattened.extract_headers().unwrap();

  assert_eq!( headers, vec![ "Full Path", "File Name", "Level", "Size" ] );
}

#[ test ]
fn test_flatten_config_path_only()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "a", "b", "c.txt" ], 100 )
    .build();

  let config = FlattenConfig::new()
    .include_name( false )
    .include_depth( false )
    .include_data( false );

  let flattened = flatten_to_table_tree_with_config( &tree, &config );
  let headers = flattened.extract_headers().unwrap();

  assert_eq!( headers, vec![ "path" ] );

  let rows = flattened.to_rows();
  assert!( rows[ 0 ][ 0 ].contains( "root" ) );
  assert!( rows[ 1 ][ 0 ].contains( "root/a" ) );
  assert!( rows[ 2 ][ 0 ].contains( "root/a/b" ) );
}

#[ test ]
fn test_flatten_config_with_table_formatter()
{
  let tree = TreeBuilder::new( "project" )
    .insert( &[ "src", "main.rs" ], 150 )
    .insert( &[ "src", "lib.rs" ], 200 )
    .build();

  let config = FlattenConfig::new()
    .include_path( false )
    .include_depth( false )
    .column_names( "ignored".into(), "File".into(), "ignored2".into(), "Lines".into() );

  let flattened = flatten_to_table_tree_with_config( &tree, &config );
  let formatter = TableFormatter::new();
  let output = formatter.format( &flattened );

  assert!( output.contains( "File" ) );
  assert!( output.contains( "Lines" ) );
  assert!( output.contains( "main.rs" ) );
  assert!( output.contains( "150" ) );
  assert!( !output.contains( "project/src" ) ); // path excluded
}
