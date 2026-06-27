//! Input type spec tests (IV-1..IV-4 per spec)
//!
//! Covers `TableView` (001) and `TreeNode` (002) Rust-level type
//! contracts: construction, metadata, Format acceptance, specializations.

#![ cfg( feature = "enabled" ) ]

#[ allow( unused_imports ) ]
use data_fmt::{
  RowBuilder, TableView, TableMetadata, DataType,
  TreeBuilder, TreeNode, ColumnData, TableShapedView,
  Format, TreeFormatter,
};

// =============================================================================
// input_type/001_table_view — IV-1..IV-4
// =============================================================================

/// IV-1: construct `TableView` via `build_view`
///
/// `RowBuilder::build_view()` produces a `TableView` with correct
/// rows, cells, and metadata.
// test_kind: spec_case(IV-1)
#[ test ]
fn input_type_001_iv_01_construct_table_view_via_build_view()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  assert_eq!( view.rows.len(), 1, "one row" );
  assert_eq!( view.rows[ 0 ].len(), 2, "two cells per row" );
  assert_eq!( view.rows[ 0 ][ 0 ].render(), "Alice" );
  assert_eq!( view.rows[ 0 ][ 1 ].render(), "30" );
}

/// IV-2: metadata carries column names and type classifications
///
/// `TableMetadata` provides both column names (ordered) and `DataType`
/// per column.  Default type is `DataType::String`.
// test_kind: spec_case(IV-2)
#[ test ]
fn input_type_001_iv_02_metadata_carries_names_and_types()
{
  // Default construction — all String types
  let view = RowBuilder::new( vec![ "id".into(), "name".into(), "active".into() ] )
    .add_row( vec![ "1".into(), "Alice".into(), "true".into() ] )
    .build_view();

  assert_eq!(
    view.metadata.column_names,
    vec![ "id", "name", "active" ],
    "column names in order",
  );
  assert_eq!( view.metadata.column_types.len(), 3, "one type per column" );
  assert!(
    view.metadata.column_types.iter().all( | t | *t == DataType::String ),
    "default type is String",
  );

  // Explicit types via TableMetadata::with_types
  let meta = TableMetadata::with_types(
    vec![ "id".into(), "name".into(), "active".into() ],
    vec![ DataType::Integer, DataType::String, DataType::Boolean ],
  );
  assert_eq!( meta.column_types[ 0 ], DataType::Integer );
  assert_eq!( meta.column_types[ 1 ], DataType::String );
  assert_eq!( meta.column_types[ 2 ], DataType::Boolean );
}

/// IV-3: `row_details` parallels rows in length
///
/// After mixing `add_row` and `add_row_with_detail`, the `row_details`
/// vector has exactly the same length as `rows`.
// test_kind: spec_case(IV-3)
#[ test ]
fn input_type_001_iv_03_row_details_parallels_rows()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .add_row( vec![ "3".into(), "4".into() ] )
    .add_row( vec![ "5".into(), "6".into() ] )
    .add_row( vec![ "7".into(), "8".into() ] )
    .add_row( vec![ "9".into(), "10".into() ] )
    .add_row_with_detail( vec![ "11".into(), "12".into() ], Some( "note".into() ) )
    .build_view();

  assert_eq!( view.rows.len(), 6, "6 rows total" );
  assert_eq!(
    view.row_details.len(),
    view.rows.len(),
    "row_details.len() == rows.len()",
  );
  // First 5 rows have no detail
  for i in 0..5
  {
    assert!( view.row_details[ i ].is_none(), "row {i} has no detail" );
  }
  // 6th row has detail
  assert!( view.row_details[ 5 ].is_some(), "row 5 has detail" );
}

/// IV-4: 9 of 10 formatters accept `TableView` via `Format` trait
///
/// All formatters except `TreeFormatter` implement `Format` and
/// return `Ok(String)` for a valid `TableView`.
// test_kind: spec_case(IV-4)
#[ test ]
fn input_type_001_iv_04_nine_formatters_accept_table_view()
{
  let view = RowBuilder::new( vec![ "x".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let mut count = 0u32;

  // 1. TableFormatter (always available with `enabled`)
  {
    let f = data_fmt::TableFormatter::new();
    assert!( f.format( &view ).is_ok(), "TableFormatter accepts TableView" );
    count += 1;
  }

  // 2. ExpandedFormatter
  #[ cfg( any( feature = "expanded_postgres", feature = "expanded_property" ) ) ]
  {
    let f = data_fmt::ExpandedFormatter::new();
    assert!( f.format( &view ).is_ok(), "ExpandedFormatter accepts TableView" );
    count += 1;
  }

  // 3. LogfmtFormatter
  #[ cfg( feature = "format_logfmt" ) ]
  {
    let f = data_fmt::LogfmtFormatter::new();
    assert!( f.format( &view ).is_ok(), "LogfmtFormatter accepts TableView" );
    count += 1;
  }

  // 4. JsonFormatter
  #[ cfg( feature = "format_json" ) ]
  {
    let f = data_fmt::JsonFormatter::new();
    assert!( f.format( &view ).is_ok(), "JsonFormatter accepts TableView" );
    count += 1;
  }

  // 5. YamlFormatter
  #[ cfg( feature = "format_yaml" ) ]
  {
    let f = data_fmt::YamlFormatter::new();
    assert!( f.format( &view ).is_ok(), "YamlFormatter accepts TableView" );
    count += 1;
  }

  // 6. TomlFormatter
  #[ cfg( feature = "format_toml" ) ]
  {
    let f = data_fmt::TomlFormatter::new();
    assert!( f.format( &view ).is_ok(), "TomlFormatter accepts TableView" );
    count += 1;
  }

  // 7. HtmlFormatter
  #[ cfg( any(
    feature = "html_minimal",
    feature = "html_bootstrap",
    feature = "html_tailwind",
    feature = "html_custom",
  ) ) ]
  {
    let f = data_fmt::HtmlFormatter::new();
    assert!( f.format( &view ).is_ok(), "HtmlFormatter accepts TableView" );
    count += 1;
  }

  // 8. SqlFormatter
  #[ cfg( any(
    feature = "sql_ansi",
    feature = "sql_postgres",
    feature = "sql_mysql",
    feature = "sql_sqlite",
  ) ) ]
  {
    let f = data_fmt::SqlFormatter::new( "test_table" );
    assert!( f.format( &view ).is_ok(), "SqlFormatter accepts TableView" );
    count += 1;
  }

  // 9. TextFormatter
  #[ cfg( feature = "format_text" ) ]
  {
    let f = data_fmt::TextFormatter::new( data_fmt::TextVariant::Bullets );
    assert!( f.format( &view ).is_ok(), "TextFormatter accepts TableView" );
    count += 1;
  }

  // With --all-features, all 9 are tested
  assert_eq!( count, 9, "exactly 9 formatters implement Format" );
}

// =============================================================================
// input_type/002_tree_node — IV-1..IV-4
// =============================================================================

/// IV-1: generic hierarchical specialization stores typed leaf data
///
/// `TreeNode<T>` is generic over the payload; `data` field holds
/// `Some(T)` for leaves and `None` for directories.
// test_kind: spec_case(IV-1)
#[ test ]
fn input_type_002_iv_01_generic_specialization_stores_typed_data()
{
  let mut root : TreeNode< i64 > = TreeNode::new( "project".into(), None );
  let mut src : TreeNode< i64 > = TreeNode::new( "src".into(), None );
  src.children.push( TreeNode::new( "main.rs".into(), Some( 150i64 ) ) );
  root.children.push( src );

  assert_eq!( root.name, "project" );
  assert!( root.data.is_none(), "directory node has no data" );
  let leaf = &root.children[ 0 ].children[ 0 ];
  assert_eq!( leaf.name, "main.rs" );
  assert_eq!( leaf.data, Some( 150i64 ), "leaf carries typed i64 data" );
}

/// IV-2: multi-column specialization uses `ColumnData`
///
/// `TreeNode<ColumnData>` is passed to `format_aligned()` which
/// aligns columns across leaf nodes.
// test_kind: spec_case(IV-2)
#[ cfg( feature = "tree_aligned" ) ]
#[ test ]
fn input_type_002_iv_02_multi_column_uses_column_data()
{
  let tree : TreeNode< ColumnData > = TreeBuilder::new( "root" )
    .insert(
      &[ "src", "main.rs" ],
      ColumnData::new( vec![ "v1.0".into(), "(150 lines)".into() ] ),
    )
    .insert(
      &[ "src", "lib.rs" ],
      ColumnData::new( vec![ "v2.0".into(), "(300 lines)".into() ] ),
    )
    .build();

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &tree );

  assert!( !output.is_empty(), "format_aligned produces output" );
  assert!( output.contains( "src" ), "output contains directory name" );
  assert!( output.contains( "v1.0" ), "output contains column data" );

  // Directory nodes don't carry ColumnData
  assert!( tree.data.is_none(), "root directory has no ColumnData" );
  assert!( tree.children[ 0 ].data.is_none(), "'src' directory has no ColumnData" );
}

/// IV-3: `TableShapedView` extracts headers and rows from tree
///
/// A table-shaped `TreeNode<String>` exposes `extract_headers()`,
/// `to_rows()`, and `is_table_shaped()` via the `TableShapedView` trait.
// test_kind: spec_case(IV-3)
#[ test ]
fn input_type_002_iv_03_table_shaped_view_extracts_headers_and_rows()
{
  // Build a table-shaped tree: root → row children → column-named leaves
  let mut root : TreeNode< String > = TreeNode::new( "root".into(), None );

  let mut first = TreeNode::new( "1".into(), None );
  first.children.push( TreeNode::new( "Name".into(), Some( "Alice".into() ) ) );
  first.children.push( TreeNode::new( "Age".into(), Some( "30".into() ) ) );
  root.children.push( first );

  let mut second = TreeNode::new( "2".into(), None );
  second.children.push( TreeNode::new( "Name".into(), Some( "Bob".into() ) ) );
  second.children.push( TreeNode::new( "Age".into(), Some( "25".into() ) ) );
  root.children.push( second );

  assert!( root.is_table_shaped(), "tree is table-shaped" );

  let headers = root.extract_headers().expect( "headers extractable" );
  assert_eq!( headers, vec![ "Name", "Age" ], "headers from first row" );

  let rows = root.to_rows();
  assert_eq!( rows.len(), 2, "2 data rows" );
  assert_eq!( rows[ 0 ], vec![ "Alice", "30" ] );
  assert_eq!( rows[ 1 ], vec![ "Bob", "25" ] );
}

/// IV-4: legacy tabular specialization removed in v0.3.0
///
/// `RowBuilder` exposes `build_view()` (returns `TableView`), not
/// `build()`.  The `Format` trait is the replacement API.
// test_kind: spec_case(IV-4)
#[ test ]
fn input_type_002_iv_04_legacy_tabular_removed()
{
  // build_view() is the canonical construction method
  let view : TableView = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "val".into() ] )
    .build_view();

  // The result is a TableView, not a TreeNode
  assert_eq!( view.metadata.column_names, vec![ "Col" ] );
  assert_eq!( view.rows.len(), 1 );

  // Format trait is the replacement API (dispatches through TableView)
  let formatter = data_fmt::TableFormatter::new();
  let result = Format::format( &formatter, &view );
  assert!( result.is_ok(), "Format trait accepts TableView from build_view()" );
}
