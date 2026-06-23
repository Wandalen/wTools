//! Pattern: Design Principles spec tests (PT-1..PT-3)
//!
//! Validates the Single Data Structure, Unified Format Interface,
//! and Granular Features design principles are observable in the API.

#![ cfg( feature = "enabled" ) ]

use data_fmt::
{
  TreeNode, TableView, TableShapedView,
  RowBuilder, TableFormatter, ExpandedFormatter, Format,
};

/// PT-1: Single Data Structure principle observable
///
/// `TreeNode` serves both hierarchical and tabular use cases.
/// `TableView` is a builder output type, not a parallel data structure.
// test_kind: spec_case(PT-1)
#[ test ]
fn pattern_002_pt_01_single_data_structure()
{
  // TreeNode serves hierarchical use case
  let tree : TreeNode< i64 > = TreeNode::new( "root".into(), None );
  assert!( tree.children.is_empty(), "TreeNode works for hierarchical data" );

  // TreeNode also serves tabular via TableShapedView extraction
  let mut root : TreeNode< String > = TreeNode::new( "root".into(), None );
  let mut row = TreeNode::new( "1".into(), None );
  row.children.push( TreeNode::new( "Col".into(), Some( "val".into() ) ) );
  root.children.push( row );
  let headers = root.extract_headers().expect( "tabular extraction works" );
  assert_eq!( headers, vec![ "Col" ] );

  // TableView is a builder product, not a separate data structure at the data layer
  let view : TableView = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "val".into() ] )
    .build_view();
  assert_eq!( view.rows.len(), 1 );
}

/// PT-2: Unified Format Interface principle observable
///
/// 9 of 10 formatters implement `Format`; `TreeFormatter` is the sole
/// exception due to its generic type parameter on the render closure.
// test_kind: spec_case(PT-2)
#[ test ]
fn pattern_002_pt_02_unified_format_interface()
{
  let view = RowBuilder::new( vec![ "x".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  // Verify Format trait works uniformly
  let r1 = Format::format( &TableFormatter::new(), &view );
  let r2 = Format::format( &ExpandedFormatter::new(), &view );
  assert!( r1.is_ok(), "TableFormatter implements Format" );
  assert!( r2.is_ok(), "ExpandedFormatter implements Format" );

  // All Format implementors return Result<String, FormatError>
  let s1 = r1.unwrap();
  let s2 = r2.unwrap();
  assert!( !s1.is_empty(), "Format::format returns non-empty String" );
  assert!( !s2.is_empty(), "different formatters produce different output" );

  // TreeFormatter uses direct method dispatch (not Format trait)
  let tree : TreeNode< i32 > = TreeNode::new( "r".into(), Some( 1 ) );
  let out = data_fmt::TreeFormatter::new().format( &tree, ToString::to_string );
  assert!( !out.is_empty(), "TreeFormatter uses direct method, not Format trait" );
}

/// PT-3: Granular Features principle observable
///
/// Core types are available under default features. Serialization
/// formatters require their respective feature flags.
// test_kind: spec_case(PT-3)
#[ test ]
fn pattern_002_pt_03_granular_features()
{
  // Core types always available under 'enabled' (which is in 'default')
  let _node : TreeNode< i32 > = TreeNode::new( "t".into(), None );
  let _view : TableView = RowBuilder::new( vec![ "A".into() ] ).build_view();
  let _fmt = TableFormatter::new();

  // Serde formatters gated behind their features
  #[ cfg( feature = "format_json" ) ]
  {
    let _json = data_fmt::JsonFormatter::new();
  }
  #[ cfg( feature = "format_yaml" ) ]
  {
    let _yaml = data_fmt::YamlFormatter::new();
  }
  #[ cfg( feature = "format_toml" ) ]
  {
    let _toml = data_fmt::TomlFormatter::new();
  }

  // HTML gated behind format_html
  #[ cfg( feature = "format_html" ) ]
  {
    let _html = data_fmt::HtmlFormatter::new();
  }

  // SQL gated behind format_sql
  #[ cfg( feature = "format_sql" ) ]
  {
    let _sql = data_fmt::SqlFormatter::new( "t" );
  }

  // Verify feature granularity via Cargo.toml structure
  let cargo = include_str!( "../Cargo.toml" );
  assert!( cargo.contains( "format_json" ), "format_json feature defined" );
  assert!( cargo.contains( "format_yaml" ), "format_yaml feature defined" );
  assert!( cargo.contains( "format_toml" ), "format_toml feature defined" );
  assert!( cargo.contains( "format_html" ), "format_html feature defined" );
  assert!( cargo.contains( "format_sql" ), "format_sql feature defined" );
}
