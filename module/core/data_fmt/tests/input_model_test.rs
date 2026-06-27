//! Input model spec tests (IM-1..IM-4 per spec)
//!
//! Covers tabular model (001) and hierarchical model (002)
//! data shape invariants.

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TreeBuilder, TreeNode, TreeFormatter };

// =============================================================================
// input_model/001_tabular — IM-1..IM-4
// =============================================================================

/// IM-1: headers define column schema
///
/// `build_view()` metadata must carry exactly the headers passed to
/// `RowBuilder::new`, in insertion order.
// test_kind: spec_case(IM-1)
#[ test ]
fn input_model_001_im_01_headers_define_column_schema()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into(), "City".into() ] )
    .add_row( vec![ "Alice".into(), "30".into(), "NYC".into() ] )
    .build_view();

  assert_eq!(
    view.metadata.column_names,
    vec![ "Name", "Age", "City" ],
    "metadata must carry exactly the 3 headers in insertion order",
  );
  assert_eq!(
    view.metadata.column_names.len(),
    3,
    "column count must match header count",
  );
}

/// IM-2: every row has same cell count as headers
///
/// `RowBuilder` enforces this at insertion time; the resulting
/// `TableView.rows` must all have the same length as headers.
// test_kind: spec_case(IM-2)
#[ test ]
fn input_model_001_im_02_every_row_same_cell_count_as_headers()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into() ] )
    .add_row( vec![ "1".into(), "2".into(), "3".into() ] )
    .add_row( vec![ "4".into(), "5".into(), "6".into() ] )
    .add_row( vec![ "7".into(), "8".into(), "9".into() ] )
    .build_view();

  let header_count = view.metadata.column_names.len();
  for ( i, row ) in view.rows.iter().enumerate()
  {
    assert_eq!(
      row.len(),
      header_count,
      "row {i} length must equal header count {header_count}",
    );
  }
}

/// IM-3: row details parallel to rows
///
/// `row_details.len()` must always equal `rows.len()`.  Rows added
/// without detail get `None`; rows with detail get `Some(...)`.
// test_kind: spec_case(IM-3)
#[ test ]
fn input_model_001_im_03_row_details_parallel_to_rows()
{
  let view = RowBuilder::new( vec![ "X".into(), "Y".into() ] )
    .add_row( vec![ "a".into(), "b".into() ] )
    .add_row_with_detail( vec![ "c".into(), "d".into() ], Some( "detail".into() ) )
    .add_row( vec![ "e".into(), "f".into() ] )
    .build_view();

  assert_eq!(
    view.row_details.len(),
    view.rows.len(),
    "row_details length must equal rows length",
  );
  assert_eq!( view.row_details.len(), 3, "exactly 3 entries" );
  assert!( view.row_details[ 0 ].is_none(), "row 0: no detail" );
  assert!( view.row_details[ 1 ].is_some(), "row 1: has detail" );
  assert!( view.row_details[ 2 ].is_none(), "row 2: no detail" );
}

/// IM-4: column order stable across rows
///
/// Cell at index `i` in every row corresponds to header at index `i`.
// test_kind: spec_case(IM-4)
#[ test ]
fn input_model_001_im_04_column_order_stable_across_rows()
{
  let view = RowBuilder::new( vec![ "X".into(), "Y".into(), "Z".into() ] )
    .add_row( vec![ "x1".into(), "y1".into(), "z1".into() ] )
    .add_row( vec![ "x2".into(), "y2".into(), "z2".into() ] )
    .add_row( vec![ "x3".into(), "y3".into(), "z3".into() ] )
    .build_view();

  for ( i, row ) in view.rows.iter().enumerate()
  {
    let n = i + 1;
    assert_eq!( row[ 0 ].render(), format!( "x{n}" ), "row {i} col 0 → header X" );
    assert_eq!( row[ 1 ].render(), format!( "y{n}" ), "row {i} col 1 → header Y" );
    assert_eq!( row[ 2 ].render(), format!( "z{n}" ), "row {i} col 2 → header Z" );
  }
}

// =============================================================================
// input_model/002_hierarchical — IM-1..IM-4
// =============================================================================

/// IM-1: every tree has exactly one root node
///
/// `TreeBuilder::build()` returns a single `TreeNode` root; top-level
/// directories are children of root, not siblings.
// test_kind: spec_case(IM-1)
#[ test ]
fn input_model_002_im_01_every_tree_has_exactly_one_root()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "src", "main.rs" ], 100 )
    .insert( &[ "tests", "test.rs" ], 50 )
    .build();

  assert_eq!( tree.name, "root", "root node name" );
  assert_eq!( tree.children.len(), 2, "root has 2 children: src and tests" );
  assert!( tree.data.is_none(), "root is a directory — no data" );
}

/// IM-2: leaf nodes carry data, directory nodes have None
///
/// Data presence distinguishes leaf from directory — no separate
/// node-kind field.
// test_kind: spec_case(IM-2)
#[ test ]
fn input_model_002_im_02_leaf_carry_data_directory_none()
{
  let tree = TreeBuilder::new( "project" )
    .insert( &[ "src", "main.rs" ], 150 )
    .build();

  assert!( tree.data.is_none(), "root directory has no data" );

  let src = &tree.children[ 0 ];
  assert_eq!( src.name, "src" );
  assert!( src.data.is_none(), "directory 'src' has no data" );

  let main_rs = &src.children[ 0 ];
  assert_eq!( main_rs.name, "main.rs" );
  assert_eq!( main_rs.data, Some( 150 ), "leaf 'main.rs' has data 150" );
}

/// IM-3: node names are plain strings not paths
///
/// Hierarchy is expressed through nesting, not path separators in names.
// test_kind: spec_case(IM-3)
#[ test ]
fn input_model_002_im_03_node_names_are_plain_strings()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "src", "lib.rs" ], 200 )
    .build();

  fn no_separators( node : &TreeNode< i32 > )
  {
    assert!(
      !node.name.contains( '/' ),
      "node '{}' must not contain '/'",
      node.name,
    );
    for child in &node.children
    {
      no_separators( child );
    }
  }

  no_separators( &tree );
  assert_eq!( tree.children[ 0 ].name, "src", "intermediate is 'src' not 'src/'" );
  assert_eq!(
    tree.children[ 0 ].children[ 0 ].name,
    "lib.rs",
    "leaf is 'lib.rs' not 'src/lib.rs'",
  );
}

/// IM-4: three specializations consume hierarchical data
///
/// Generic (`format`), multi-column (`format_aligned`), and
/// aggregated (`format_with_aggregation`) all produce well-formed output.
// test_kind: spec_case(IM-4)
#[ test ]
fn input_model_002_im_04_three_specializations_consume_hierarchical()
{
  let formatter = TreeFormatter::new();

  // (a) Generic tree with typed leaf data — format()
  let generic_tree = TreeBuilder::new( "root" )
    .insert( &[ "src", "main.rs" ], 100u64 )
    .insert( &[ "src", "lib.rs" ], 200u64 )
    .build();
  let out_generic = formatter.format( &generic_tree, | v | format!( "{v} lines" ) );
  assert!( !out_generic.is_empty(), "format() produced output" );
  assert!( out_generic.contains( "main.rs" ), "format() contains leaf name" );

  // (b) Multi-column tree with ColumnData — format_aligned()
  #[ cfg( feature = "tree_aligned" ) ]
  {
    use data_fmt::ColumnData;
    let aligned_tree : TreeNode< ColumnData > = TreeBuilder::new( "root" )
      .insert(
        &[ "src", "main.rs" ],
        ColumnData::new( vec![ "v1.0".into(), "(150 lines)".into() ] ),
      )
      .insert(
        &[ "src", "lib.rs" ],
        ColumnData::new( vec![ "v2.0".into(), "(300 lines)".into() ] ),
      )
      .build();
    let out_aligned = formatter.format_aligned( &aligned_tree );
    assert!( !out_aligned.is_empty(), "format_aligned() produced output" );
    assert!( out_aligned.contains( "v1.0" ), "format_aligned() contains column data" );
  }

  // (c) Aggregating tree with computed totals — format_with_aggregation()
  #[ cfg( feature = "tree_aggregated" ) ]
  {
    let agg_tree = TreeBuilder::new( "root" )
      .insert( &[ "src", "main.rs" ], 100u64 )
      .insert( &[ "src", "lib.rs" ], 200u64 )
      .build();
    let out_agg = formatter.format_with_aggregation(
      &agg_tree,
      300u64,
      | val | *val,
      | v | v as f64,
      | val, _total, pct | format!( "{val} lines ({pct:.0}%)" ),
      | _name, total, pct | format!( "{total} lines ({pct:.0}%)" ),
    );
    assert!( !out_agg.is_empty(), "format_with_aggregation() produced output" );
    assert!( out_agg.contains( "main.rs" ), "aggregated output contains leaf" );
  }
}
