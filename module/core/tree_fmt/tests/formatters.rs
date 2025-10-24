//! Tests for formatters, traits, generic types, and write support

#![ cfg( feature = "integration" ) ]
#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::
{
  TreeNode, RowBuilder, TableShapedView,
  formatters::TableShapedFormatter,
  TableFormatter, ExpandedFormatter,
};

// =============================================================================
// TableShapedFormatter Trait Tests
// =============================================================================

#[ test ]
fn test_table_shaped_formatter_trait_polymorphism()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Score".into() ] )
    .add_row( vec![ "Alice".into(), "95".into() ] )
    .build();

  // Use trait object for polymorphism
  let formatters : Vec< Box< dyn TableShapedFormatter > > = vec![
    Box::new( TableFormatter::new() ),
    Box::new( ExpandedFormatter::new() ),
  ];

  for formatter in formatters
  {
    let output = formatter.format( &tree );
    assert!( !output.is_empty() );
    assert!( output.contains( "Alice" ) );
    assert!( output.contains( "95" ) );
  }
}

#[ test ]
fn test_table_shaped_formatter_trait_reference()
{
  let tree = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "data".into() ] )
    .build();

  let table_fmt = TableFormatter::new();
  let expanded_fmt = ExpandedFormatter::new();

  let table_ref : &dyn TableShapedFormatter = &table_fmt;
  let expanded_ref : &dyn TableShapedFormatter = &expanded_fmt;

  let table_output = table_ref.format( &tree );
  let expanded_output = expanded_ref.format( &tree );

  assert!( table_output.contains( "data" ) );
  assert!( expanded_output.contains( "data" ) );
  assert_ne!( table_output, expanded_output ); // Different formats
}

// =============================================================================
// Generic TableView Tests (T: Display)
// =============================================================================

#[ test ]
fn test_generic_table_view_with_integers()
{
  let mut root = TreeNode::new( "root".to_string(), None::<u64> );

  let mut row1 = TreeNode::new( "row1".to_string(), None );
  row1.children.push( TreeNode::new( "A".into(), Some( 100u64 ) ) );
  row1.children.push( TreeNode::new( "B".into(), Some( 200u64 ) ) );

  let mut row2 = TreeNode::new( "row2".to_string(), None );
  row2.children.push( TreeNode::new( "A".into(), Some( 300u64 ) ) );
  row2.children.push( TreeNode::new( "B".into(), Some( 400u64 ) ) );

  root.children.push( row1 );
  root.children.push( row2 );

  // Extract headers (generic over T)
  let headers = root.extract_headers().unwrap();
  assert_eq!( headers, vec![ "A", "B" ] );

  // Check table shape (generic over T)
  assert!( root.is_table_shaped() );

  // Extract rows (converts T to String via Display)
  let rows = root.to_rows();
  assert_eq!( rows.len(), 2 );
  assert_eq!( rows[ 0 ], vec![ "100", "200" ] );
  assert_eq!( rows[ 1 ], vec![ "300", "400" ] );
}

#[ test ]
fn test_generic_table_view_with_floats()
{
  let mut root = TreeNode::new( "prices".to_string(), None::<f64> );

  let mut row = TreeNode::new( "item1".to_string(), None );
  row.children.push( TreeNode::new( "cost".into(), Some( 19.99f64 ) ) );
  row.children.push( TreeNode::new( "tax".into(), Some( 1.60f64 ) ) );

  root.children.push( row );

  let rows = root.to_rows();
  assert_eq!( rows[ 0 ][ 0 ], "19.99" );
  assert_eq!( rows[ 0 ][ 1 ], "1.6" );
}

#[ test ]
fn test_generic_table_view_with_custom_type()
{
  #[ derive( Debug, Clone ) ]
  struct Status
  {
    code : u32,
    ok : bool,
  }

  impl std::fmt::Display for Status
  {
    fn fmt( &self, f : &mut std::fmt::Formatter ) -> std::fmt::Result
    {
      write!( f, "{}:{}", self.code, if self.ok { "OK" } else { "ERR" } )
    }
  }

  let mut root = TreeNode::new( "statuses".to_string(), None::<Status> );

  let mut row = TreeNode::new( "server1".to_string(), None );
  row.children.push( TreeNode::new( "http".into(), Some( Status { code: 200, ok: true } ) ) );
  row.children.push( TreeNode::new( "db".into(), Some( Status { code: 500, ok: false } ) ) );

  root.children.push( row );

  let rows = root.to_rows();
  assert_eq!( rows[ 0 ][ 0 ], "200:OK" );
  assert_eq!( rows[ 0 ][ 1 ], "500:ERR" );
}

#[ test ]
fn test_generic_table_view_with_formatter()
{
  // Create tree with numeric data
  let mut root = TreeNode::new( "scores".to_string(), None::<i32> );

  let mut row = TreeNode::new( "player1".to_string(), None );
  row.children.push( TreeNode::new( "round1".into(), Some( 85i32 ) ) );
  row.children.push( TreeNode::new( "round2".into(), Some( 92i32 ) ) );

  root.children.push( row );

  // Verify generic TableView works
  assert!( root.is_table_shaped() );
  let rows = root.to_rows();
  assert_eq!( rows[ 0 ], vec![ "85", "92" ] );
}

// =============================================================================
// Write Trait Tests
// =============================================================================

#[ test ]
fn test_write_trait_to_stdout()
{
  use tree_fmt::{ RowBuilder, TableFormatter };
  use std::io::Cursor;

  let tree = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "Data".into() ] )
    .build();

  let formatter = TableFormatter::new();
  let mut buffer = Cursor::new( Vec::new() );

  formatter.write_to( &tree, &mut buffer ).unwrap();

  let output = String::from_utf8( buffer.into_inner() ).unwrap();
  assert!( output.contains( "Data" ) );
  assert!( output.contains( "Col" ) );
}

#[ test ]
fn test_write_trait_multiple_formatters()
{
  use tree_fmt::{ RowBuilder, TableFormatter, ExpandedFormatter };
  use std::io::Cursor;

  let tree = RowBuilder::new( vec![ "X".into() ] )
    .add_row( vec![ "Y".into() ] )
    .build();

  // Test TableFormatter
  let mut buf1 = Cursor::new( Vec::new() );
  TableFormatter::new().write_to( &tree, &mut buf1 ).unwrap();
  let out1 = String::from_utf8( buf1.into_inner() ).unwrap();

  // Test ExpandedFormatter
  let mut buf2 = Cursor::new( Vec::new() );
  ExpandedFormatter::new().write_to( &tree, &mut buf2 ).unwrap();
  let out2 = String::from_utf8( buf2.into_inner() ).unwrap();

  assert!( out1.contains( "Y" ) );
  assert!( out2.contains( "Y" ) );
  assert_ne!( out1, out2 ); // Different formats
}

// =============================================================================
// Colored Keys Tests (NEW v0.2.0)
// =============================================================================

#[ test ]
fn test_expanded_formatter_no_color_by_default()
{
  use tree_fmt::{ RowBuilder, ExpandedFormatter };

  let tree = RowBuilder::new( vec![ "Name".into(), "Status".into() ] )
    .add_row( vec![ "server1".into(), "online".into() ] )
    .build();

  let formatter = ExpandedFormatter::new();
  let output = formatter.format( &tree );

  // Verify NO ANSI codes in default output
  assert!( !output.contains( "\x1b[" ) );
  assert!( output.contains( "Name" ) );
  assert!( output.contains( "server1" ) );
}

#[ test ]
fn test_expanded_formatter_with_colored_keys()
{
  use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };

  let tree = RowBuilder::new( vec![ "Name".into(), "Status".into() ] )
    .add_row( vec![ "server1".into(), "online".into() ] )
    .build();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .colorize_keys( true )
  );

  let output = formatter.format( &tree );

  // Verify ANSI codes present
  assert!( output.contains( "\x1b[90m" ) );  // Gray color
  assert!( output.contains( "\x1b[0m" ) );   // Reset
  assert!( output.contains( "Name" ) );
  assert!( output.contains( "Status" ) );
  assert!( output.contains( "server1" ) );
  assert!( output.contains( "online" ) );
}

#[ test ]
fn test_expanded_formatter_custom_color()
{
  use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };

  let tree = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .colorize_keys( true )
      .key_color( "\x1b[34m".to_string() )  // Blue
  );

  let output = formatter.format( &tree );

  // Verify custom color code present
  assert!( output.contains( "\x1b[34m" ) );  // Blue color
  assert!( output.contains( "\x1b[0m" ) );   // Reset
  assert!( output.contains( "Name" ) );
  assert!( output.contains( "Alice" ) );
}

#[ test ]
fn test_expanded_formatter_color_disabled_explicitly()
{
  use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };

  let tree = RowBuilder::new( vec![ "Key".into() ] )
    .add_row( vec![ "Value".into() ] )
    .build();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .colorize_keys( false )  // Explicitly disabled
  );

  let output = formatter.format( &tree );

  // Verify NO ANSI codes
  assert!( !output.contains( "\x1b[" ) );
}

// Property style and padding tests

#[ test ]
fn test_expanded_formatter_property_style_basic()
{
  use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };

  let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build();

  // property_style() now has colors by default - disable for plain output testing
  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::property_style().colorize_keys( false )
  );
  let output = formatter.format( &tree );

  assert!( output.contains( "Name: Alice" ) );
  assert!( output.contains( "Age:  30" ) );  // Note extra space for alignment
  assert!( !output.contains( "RECORD" ) );
  assert!( !output.contains( "|" ) );
}

#[ test ]
fn test_expanded_formatter_property_style_alignment()
{
  use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };

  let tree = RowBuilder::new( vec![ "ID".into(), "Description".into() ] )
    .add_row( vec![ "1".into(), "Test".into() ] )
    .build();

  // Disable colors for plain output testing
  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::property_style().colorize_keys( false )
  );
  let output = formatter.format( &tree );

  // Values should align (Description is longer key)
  assert!( output.contains( "ID:          1" ) );
  assert!( output.contains( "Description: Test" ) );
}

#[ test ]
fn test_expanded_formatter_property_style_multiple_records()
{
  use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };

  let tree = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .add_row( vec![ "Bob".into() ] )
    .build();

  // Disable colors for plain output testing
  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::property_style().colorize_keys( false )
  );
  let output = formatter.format( &tree );

  // Should have blank line between records
  assert!( output.contains( "Name: Alice\n\nName: Bob" ) );
}

#[ test ]
fn test_expanded_formatter_property_style_with_colors()
{
  use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };

  let tree = RowBuilder::new( vec![ "Key".into() ] )
    .add_row( vec![ "Value".into() ] )
    .build();

  // property_style() has gray keys enabled by default
  let formatter = ExpandedFormatter::with_config( ExpandedConfig::property_style() );
  let output = formatter.format( &tree );

  assert!( output.contains( "\x1b[90mKey:\x1b[0m" ) );
  assert!( output.contains( "Value" ) );
}

#[ test ]
fn test_expanded_formatter_property_style_no_colors()
{
  use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };

  let tree = RowBuilder::new( vec![ "Key".into() ] )
    .add_row( vec![ "Value".into() ] )
    .build();

  // Disable colors explicitly if needed
  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::property_style().colorize_keys( false )
  );
  let output = formatter.format( &tree );

  assert!( !output.contains( "\x1b[" ) );
  assert!( output.contains( "Key:" ) );
  assert!( output.contains( "Value" ) );
}

#[ test ]
fn test_expanded_formatter_padding_side_before()
{
  use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig, PaddingSide };

  let tree = RowBuilder::new( vec![ "A".into(), "LongKey".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .padding_side( PaddingSide::BeforeSeparator )
      .key_value_separator( " | ".to_string() )
  );
  let output = formatter.format( &tree );

  // Keys padded before separator
  assert!( output.contains( "A       |" ) );
  assert!( output.contains( "LongKey |" ) );
}

#[ test ]
fn test_expanded_formatter_padding_side_after()
{
  use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig, PaddingSide };

  let tree = RowBuilder::new( vec![ "A".into(), "LongKey".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .padding_side( PaddingSide::AfterSeparator )
      .key_value_separator( ": ".to_string() )
  );
  let output = formatter.format( &tree );

  // Values padded after separator
  // separator `: ` has 1 trailing space, longest key gets just that space
  assert!( output.contains( "A:       1" ) );  // 7 spaces: 1 (from sep) + 6 (padding)
  assert!( output.contains( "LongKey: 2" ) );  // 1 space: 1 (from sep) + 0 (padding)
}

#[ test ]
fn test_expanded_formatter_no_record_separator()
{
  use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };

  let tree = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .add_row( vec![ "Bob".into() ] )
    .build();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new().record_separator( "".to_string() )
  );
  let output = formatter.format( &tree );

  assert!( !output.contains( "RECORD" ) );
  assert!( !output.contains( "[" ) );
  assert!( !output.contains( "]" ) );
}

#[ test ]
fn test_expanded_formatter_custom_record_separator()
{
  use tree_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig };

  let tree = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new().record_separator( "=== Entry {} ===".to_string() )
  );
  let output = formatter.format( &tree );

  assert!( output.contains( "=== Entry 1 ===" ) );
}

#[ test ]
fn test_expanded_config_builder_methods()
{
  use tree_fmt::{ ExpandedConfig, PaddingSide };

  let config = ExpandedConfig::new()
    .record_separator( "---".to_string() )
    .key_value_separator( " = ".to_string() )
    .show_record_numbers( false )
    .colorize_keys( true )
    .key_color( "\x1b[36m".to_string() )
    .padding_side( PaddingSide::AfterSeparator );

  assert_eq!( config.record_separator, "---" );
  assert_eq!( config.key_value_separator, " = " );
  assert_eq!( config.show_record_numbers, false );
  assert_eq!( config.colorize_keys, true );
  assert_eq!( config.key_color, "\x1b[36m" );
  assert_eq!( config.padding_side, PaddingSide::AfterSeparator );
}
