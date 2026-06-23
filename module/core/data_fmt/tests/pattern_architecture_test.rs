//! Pattern: Three-Layer Architecture spec tests (PT-1..PT-3)
//!
//! Validates that Layer 1 (data types), Layer 2 (builders), and
//! Layer 3 (formatters) maintain their architectural boundaries.

#![ cfg( feature = "enabled" ) ]

use data_fmt::
{
  TreeNode, TableView, RowBuilder, TreeBuilder,
  TableFormatter, ExpandedFormatter, Format,
};

/// PT-1: Layer 1 data types are public
///
/// Both `TreeNode` and `TableView` are importable from the crate root
/// without feature gates.
// test_kind: spec_case(PT-1)
#[ test ]
fn pattern_001_pt_01_layer1_data_types_public()
{
  // TreeNode has name, data, children fields
  let node : TreeNode< i32 > = TreeNode::new( "test".into(), Some( 42 ) );
  assert_eq!( node.name, "test" );
  assert_eq!( node.data, Some( 42 ) );
  assert!( node.children.is_empty() );

  // TableView has metadata and rows (constructed via builder)
  let view : TableView = RowBuilder::new( vec![ "A".into() ] )
    .add_row( vec![ "v".into() ] )
    .build_view();

  assert_eq!( view.metadata.column_names.len(), 1 );
  assert_eq!( view.rows.len(), 1 );
  assert_eq!( view.row_details.len(), 1 );
}

/// PT-2: Layer 2 builders produce Layer 1 types
///
/// `RowBuilder::build_view()` returns `TableView`; `TreeBuilder::build()`
/// returns `TreeNode`. Builders expose no formatter-specific details.
// test_kind: spec_case(PT-2)
#[ test ]
fn pattern_001_pt_02_builders_produce_layer1_types()
{
  // RowBuilder → TableView (Layer 1)
  let view : TableView = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();
  assert_eq!( view.metadata.column_names.len(), 2 );

  // TreeBuilder → TreeNode (Layer 1)
  let tree : TreeNode< i64 > = TreeBuilder::new( "root" )
    .insert( &[ "child" ], 100 )
    .build();
  assert_eq!( tree.name, "root" );
  assert_eq!( tree.children.len(), 1 );
}

/// PT-3: Layer 3 formatters consume Layer 1/2 output
///
/// Multiple formatters accept the same `TableView` without modification.
/// `TreeFormatter` separately accepts `TreeNode` directly.
// test_kind: spec_case(PT-3)
#[ test ]
fn pattern_001_pt_03_formatters_consume_layer1()
{
  let view = RowBuilder::new( vec![ "X".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  // TableFormatter accepts TableView
  let r1 = Format::format( &TableFormatter::new(), &view );
  assert!( r1.is_ok(), "TableFormatter accepts TableView" );

  // ExpandedFormatter accepts same TableView
  let r2 = Format::format( &ExpandedFormatter::new(), &view );
  assert!( r2.is_ok(), "ExpandedFormatter accepts same TableView" );

  // JsonFormatter accepts same TableView
  #[ cfg( feature = "format_json" ) ]
  {
    let r3 = Format::format( &data_fmt::JsonFormatter::new(), &view );
    assert!( r3.is_ok(), "JsonFormatter accepts same TableView" );
  }

  // TreeFormatter accepts TreeNode directly (not TableView)
  let tree = TreeBuilder::new( "r" ).insert( &[ "a" ], 1 ).build();
  let out = data_fmt::TreeFormatter::new().format( &tree, ToString::to_string );
  assert!( out.contains( 'a' ), "TreeFormatter accepts TreeNode directly" );
}
