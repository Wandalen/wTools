//! Tests for fluent builder APIs and config builder patterns

#![ cfg( feature = "enabled" ) ]

use data_fmt::
{
  RowBuilder,
  Format, TableFormatter, ExpandedFormatter,
};

// =============================================================================
// Fluent RowBuilder API Tests
// =============================================================================

#[ test ]
fn test_fluent_builder_single_chain()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  assert_eq!( view.rows.len(), 1 );
  assert_eq!( view.rows[ 0 ][ 0 ].render(), "Alice" );
}

#[ test ]
fn test_fluent_builder_multiple_chains()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .add_row( vec![ "3".into(), "4".into() ] )
    .add_row( vec![ "5".into(), "6".into() ] )
    .build_view();

  assert_eq!( view.rows.len(), 3 );
  assert_eq!( view.rows[ 0 ][ 0 ].render(), "1" );
  assert_eq!( view.rows[ 1 ][ 0 ].render(), "3" );
  assert_eq!( view.rows[ 2 ][ 0 ].render(), "5" );
}

#[ test ]
fn test_fluent_builder_with_custom_names()
{
  let view = RowBuilder::new( vec![ "Value".into() ] )
    .add_row_with_name( "first".into(), vec![ "100".into() ] )
    .add_row_with_name( "second".into(), vec![ "200".into() ] )
    .build_view();

  assert_eq!( view.rows.len(), 2 );
  assert_eq!( view.rows[ 0 ][ 0 ].render(), "100" );
  assert_eq!( view.rows[ 1 ][ 0 ].render(), "200" );
}

#[ test ]
fn test_fluent_builder_mixed_with_mut()
{
  let mut builder = RowBuilder::new( vec![ "X".into() ] );
  builder.add_row_mut( vec![ "1".into() ] );
  builder.add_row_mut( vec![ "2".into() ] );
  let view = builder.build_view();

  assert_eq!( view.rows.len(), 2 );
}

#[ test ]
fn test_fluent_builder_with_formatter_trait()
{
  // Build view fluently
  let view = RowBuilder::new( vec![ "Language".into(), "Year".into() ] )
    .add_row( vec![ "Rust".into(), "2015".into() ] )
    .add_row( vec![ "Python".into(), "1991".into() ] )
    .build_view();

  // Format using trait polymorphism
  let formatters : Vec< Box< dyn Format > > = vec![
    Box::new( TableFormatter::new() ),
    Box::new( ExpandedFormatter::new() ),
  ];

  for formatter in formatters
  {
    let output = formatter.format( &view ).unwrap_or_default();
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
  use data_fmt::{ TreeConfig, TreeFormatter, TreeBuilder };

  let config = TreeConfig::new()
    .show_branches( false )
    .show_root( true )
    .indent_size( 2 )
    .max_depth( Some( 3 ) );

  let tree = TreeBuilder::new( "root" )
    .insert( &[ "dir", "file.txt" ], 100 )
    .build();

  let formatter = TreeFormatter::with_config( config );
  let output = formatter.format( &tree, | n | format!( "{n}" ) );

  assert!( output.contains( "root" ) );
}

#[ test ]
fn test_table_config_builder()
{
  use data_fmt::{ TableConfig, TableFormatter, RowBuilder, BorderVariant, Format };

  let config = TableConfig::new()
    .border_variant( BorderVariant::None )
    .column_widths( vec![ 10, 15 ] )
    .align_right( vec![ false, true ] );

  let view = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ "Item".into(), "123".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( config );
  let output = formatter.format( &view ).unwrap_or_default();

  assert!( output.contains( "Item" ) );
  assert!( !output.contains( '|' ) ); // No borders
}

#[ test ]
fn test_expanded_config_builder()
{
  use data_fmt::{ ExpandedConfig, ExpandedFormatter, RowBuilder, Format };

  let config = ExpandedConfig::new()
    .record_separator( "--- Record {} ---".into() )
    .key_value_separator( " = ".into() );

  let view = RowBuilder::new( vec![ "Key".into() ] )
    .add_row( vec![ "Value".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config( config );
  let output = formatter.format( &view ).unwrap_or_default();

  assert!( output.contains( "--- Record" ) );
  assert!( output.contains( " = " ) );
}

// T05: Builder chain with .border_variant() and .column_separator() — survives field-privacy change
#[ test ]
fn test_table_config_builder_border_and_separator()
{
  use data_fmt::{ TableConfig, TableFormatter, RowBuilder, BorderVariant, ColumnSeparator, Format };

  let config = TableConfig::new()
  .border_variant( BorderVariant::Unicode )
  .column_separator( ColumnSeparator::Character( '│' ) );

  let view = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
  .add_row( vec![ "Alice".into(), "42".into() ] )
  .build_view();

  let output = Format::format( &TableFormatter::with_config( config ), &view ).unwrap_or_default();

  // Verify builder methods compile and produce correct output
  assert!( output.contains( "Alice" ), "output must contain data; output:\n{output}" );
  assert!( output.contains( '│' ), "output must contain Unicode │ separator; output:\n{output}" );
}
