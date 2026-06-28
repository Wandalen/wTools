//! Formatter design pattern spec tests (PT-1..PT-3)
//!
//! Covers `Format` trait implementation count, `TableShapedView`
//! decoupling, and dual output surface (`format` + `write_to`).

#![ cfg( feature = "enabled" ) ]

use data_fmt::{
  RowBuilder, TableFormatter, TreeNode, TableShapedView,
  Format, TableConfig,
};
#[ cfg( feature = "full" ) ]
use data_fmt::ExpandedFormatter;

/// PT-1: Format trait implemented by 9 formatters
///
/// All 9 non-tree formatters accept `&TableView` via `Format::format`.
/// `TreeFormatter` does not implement `Format`.
// test_kind: spec_case(PT-1)
#[ cfg( feature = "full" ) ]
#[ test ]
fn pattern_003_pt_01_format_trait_implemented_by_nine()
{
  let view = RowBuilder::new( vec![ "x".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let mut count = 0u32;

  // 1. TableFormatter
  { let f = TableFormatter::new(); assert!( f.format( &view ).is_ok() ); count += 1; }
  // 2. ExpandedFormatter
  { let f = ExpandedFormatter::new(); assert!( f.format( &view ).is_ok() ); count += 1; }
  // 3. LogfmtFormatter
  #[ cfg( feature = "format_logfmt" ) ]
  { let f = data_fmt::LogfmtFormatter::new(); assert!( f.format( &view ).is_ok() ); count += 1; }
  // 4. JsonFormatter
  #[ cfg( feature = "format_json" ) ]
  { let f = data_fmt::JsonFormatter::new(); assert!( f.format( &view ).is_ok() ); count += 1; }
  // 5. YamlFormatter
  #[ cfg( feature = "format_yaml" ) ]
  { let f = data_fmt::YamlFormatter::new(); assert!( f.format( &view ).is_ok() ); count += 1; }
  // 6. TomlFormatter
  #[ cfg( feature = "format_toml" ) ]
  { let f = data_fmt::TomlFormatter::new(); assert!( f.format( &view ).is_ok() ); count += 1; }
  // 7. HtmlFormatter
  #[ cfg( any( feature = "html_minimal", feature = "html_bootstrap", feature = "html_tailwind", feature = "html_custom" ) ) ]
  { let f = data_fmt::HtmlFormatter::new(); assert!( f.format( &view ).is_ok() ); count += 1; }
  // 8. SqlFormatter
  #[ cfg( any( feature = "sql_ansi", feature = "sql_postgres", feature = "sql_mysql", feature = "sql_sqlite" ) ) ]
  { let f = data_fmt::SqlFormatter::new( "t" ); assert!( f.format( &view ).is_ok() ); count += 1; }
  // 9. TextFormatter
  #[ cfg( feature = "format_text" ) ]
  { let f = data_fmt::TextFormatter::new( data_fmt::TextVariant::Bullets ); assert!( f.format( &view ).is_ok() ); count += 1; }

  assert_eq!( count, 9, "exactly 9 formatters implement Format" );
}

/// PT-2: `TableShapedView` decouples formatters from tree internals
///
/// Formatters receive flat `TableView` data, not raw `TreeNode`.
/// `TableShapedView` extracts headers and rows from a table-shaped tree.
// test_kind: spec_case(PT-2)
#[ test ]
fn pattern_003_pt_02_table_shaped_view_decouples()
{
  // Build a table-shaped tree
  let mut root : TreeNode< String > = TreeNode::new( "root".into(), None );
  let mut r1 = TreeNode::new( "1".into(), None );
  r1.children.push( TreeNode::new( "Name".into(), Some( "Alice".into() ) ) );
  r1.children.push( TreeNode::new( "Age".into(), Some( "30".into() ) ) );
  root.children.push( r1 );
  let mut r2 = TreeNode::new( "2".into(), None );
  r2.children.push( TreeNode::new( "Name".into(), Some( "Bob".into() ) ) );
  r2.children.push( TreeNode::new( "Age".into(), Some( "25".into() ) ) );
  root.children.push( r2 );

  // Extract via TableShapedView — formatter doesn't need to know tree structure
  let headers = root.extract_headers().expect( "extractable" );
  assert_eq!( headers, vec![ "Name", "Age" ] );

  let rows = root.to_rows();
  assert_eq!( rows.len(), 2 );
  assert_eq!( rows[ 0 ], vec![ "Alice", "30" ] );
  assert_eq!( rows[ 1 ], vec![ "Bob", "25" ] );
}

/// PT-3: Dual output surface available
///
/// Both `format()` (returns `String`) and `write_to()` (writes to `io::Write`)
/// produce identical output content.
// test_kind: spec_case(PT-3)
#[ test ]
fn pattern_003_pt_03_dual_output_surface()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "val".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::plain() );

  // format() → String
  let via_format = Format::format( &formatter, &view ).unwrap();

  // write_to() → buffer
  let mut buf = Vec::new();
  formatter.write_to( &view, &mut buf ).unwrap();
  let via_write = String::from_utf8( buf ).unwrap();

  assert_eq!(
    via_format, via_write,
    "format() and write_to() must produce identical output",
  );
}
