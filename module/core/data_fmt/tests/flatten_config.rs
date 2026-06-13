//! Tests for `FlattenConfig` functionality

#![ cfg( feature = "enabled" ) ]

use data_fmt::
{
  TreeBuilder, Format,
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
  let headers = flattened.metadata.column_names.clone();

  assert_eq!( headers, vec![ "name", "data" ] );
  assert_eq!( flattened.rows.len(), 3 ); // root, dir, file.txt
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
  let headers = flattened.metadata.column_names.clone();

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
  let headers = flattened.metadata.column_names.clone();

  assert_eq!( headers, vec![ "path" ] );

  assert!( flattened.rows[ 0 ][ 0 ].render().contains( "root" ) );
  assert!( flattened.rows[ 1 ][ 0 ].render().contains( "root/a" ) );
  assert!( flattened.rows[ 2 ][ 0 ].render().contains( "root/a/b" ) );
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
  let output = Format::format( &formatter, &flattened ).unwrap();

  assert!( output.contains( "File" ) );
  assert!( output.contains( "Lines" ) );
  assert!( output.contains( "main.rs" ) );
  assert!( output.contains( "150" ) );
  assert!( !output.contains( "project/src" ) ); // path excluded
}

// --- AP-7: FlattenConfig defaults all fields to true; custom names override defaults ---
//
// Given: FlattenConfig::default() (all include flags = true, column_names = None).
// When: field values are inspected, and custom names are set via column_names().
// Then: all four include flags are true; custom names appear in the resulting TableView metadata.

/// AP-7 — `api/002_builders`: `FlattenConfig` defaults all fields to true; custom names override.
// test_kind: standard
#[ test ]
fn flatten_config_defaults_all_true_custom_names_override_ap7()
{
  // FlattenConfig::default() must have all include flags = true
  let cfg = FlattenConfig::default();
  assert!( cfg.include_path, "include_path must default to true" );
  assert!( cfg.include_name, "include_name must default to true" );
  assert!( cfg.include_depth, "include_depth must default to true" );
  assert!( cfg.include_data, "include_data must default to true" );

  // Custom column names must override defaults and appear in the resulting TableView
  let custom = FlattenConfig::new()
    .column_names( "P".into(), "N".into(), "D".into(), "V".into() );
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "leaf" ], 1 )
    .build();
  let view = flatten_to_table_tree_with_config( &tree, &custom );
  assert_eq!(
    view.metadata.column_names,
    vec![ "P", "N", "D", "V" ],
    "custom column names must replace default names in TableView metadata",
  );
}
