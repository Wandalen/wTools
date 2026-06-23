//! `TableShapedView` trait spec tests (TR-7..TR-12)
//!
//! Covers `extract_headers`, `is_table_shaped`, `to_rows`, trait object dispatch,
//! empty tree, and mismatched columns.

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ TreeNode, TableShapedView };

/// Helper: build a table-shaped tree with the given rows.
/// Each row is a vec of (`column_name`, `cell_value`) pairs.
fn make_table_tree( rows : Vec< Vec< ( &str, &str ) > > ) -> TreeNode< String >
{
  let mut root = TreeNode::new( "root".to_string(), None );
  for ( i, row_data ) in rows.into_iter().enumerate()
  {
    let mut row = TreeNode::new( ( i + 1 ).to_string(), None );
    for ( col_name, value ) in row_data
    {
      row.children.push( TreeNode::new( col_name.to_string(), Some( value.to_string() ) ) );
    }
    root.children.push( row );
  }
  root
}

/// TR-7: `extract_headers` returns column names from first row
// test_kind: spec_case(TR-7)
#[ test ]
fn trait_003_tr_07_extract_headers_returns_column_names_from_first_row()
{
  let tree = make_table_tree( vec![
    vec![ ( "Name", "Alice" ), ( "Age", "30" ) ],
  ]);

  let headers = tree.extract_headers();
  assert_eq!(
    headers,
    Some( vec![ "Name".to_string(), "Age".to_string() ] ),
    "headers should match first row's child names",
  );
}

/// TR-8: `is_table_shaped` returns true for uniform structure
// test_kind: spec_case(TR-8)
#[ test ]
fn trait_003_tr_08_is_table_shaped_returns_true_for_uniform_structure()
{
  let tree = make_table_tree( vec![
    vec![ ( "Col1", "a" ), ( "Col2", "b" ) ],
    vec![ ( "Col1", "c" ), ( "Col2", "d" ) ],
    vec![ ( "Col1", "e" ), ( "Col2", "f" ) ],
  ]);

  assert!( tree.is_table_shaped(), "uniform column names should be table-shaped" );
}

/// TR-9: `to_rows` returns cell values as string matrix
// test_kind: spec_case(TR-9)
#[ test ]
fn trait_003_tr_09_to_rows_returns_cell_values_as_string_matrix()
{
  let tree = make_table_tree( vec![
    vec![ ( "Name", "Alice" ), ( "Age", "30" ) ],
    vec![ ( "Name", "Bob" ), ( "Age", "25" ) ],
  ]);

  let rows = tree.to_rows();
  assert_eq!( rows.len(), 2, "should have 2 rows" );
  assert_eq!( rows[ 0 ], vec![ "Alice", "30" ], "row 1 values" );
  assert_eq!( rows[ 1 ], vec![ "Bob", "25" ], "row 2 values" );
}

/// TR-10: trait object dispatch through dyn `TableShapedView`
// test_kind: spec_case(TR-10)
#[ test ]
fn trait_003_tr_10_trait_object_dispatch_through_dyn_table_shaped_view()
{
  let tree = make_table_tree( vec![
    vec![ ( "Name", "Alice" ), ( "Age", "30" ) ],
  ]);

  // Direct calls
  let direct_headers = tree.extract_headers();
  let direct_shaped = tree.is_table_shaped();
  let direct_rows = tree.to_rows();

  // Trait object calls
  let dyn_view : &dyn TableShapedView = &tree;
  let obj_headers = dyn_view.extract_headers();
  let obj_shaped = dyn_view.is_table_shaped();
  let obj_rows = dyn_view.to_rows();

  assert_eq!( direct_headers, obj_headers, "extract_headers should match" );
  assert_eq!( direct_shaped, obj_shaped, "is_table_shaped should match" );
  assert_eq!( direct_rows, obj_rows, "to_rows should match" );
}

/// TR-11: `extract_headers` returns None on empty tree
// test_kind: spec_case(TR-11)
#[ test ]
fn trait_003_tr_11_extract_headers_returns_none_on_empty_tree()
{
  let tree : TreeNode< String > = TreeNode::new( "root".to_string(), None );

  assert_eq!( tree.extract_headers(), None, "empty tree should return None" );
  assert!( tree.is_table_shaped(), "empty tree is trivially table-shaped" );
}

/// TR-12: `is_table_shaped` returns false on mismatched columns
// test_kind: spec_case(TR-12)
#[ test ]
fn trait_003_tr_12_is_table_shaped_returns_false_on_mismatched_columns()
{
  let mut root = TreeNode::new( "root".to_string(), None::< String > );

  // Row 1: children A, B
  let mut row1 = TreeNode::new( "1".to_string(), None );
  row1.children.push( TreeNode::new( "A".to_string(), Some( "1".to_string() ) ) );
  row1.children.push( TreeNode::new( "B".to_string(), Some( "2".to_string() ) ) );

  // Row 2: children A, C (different second column name)
  let mut row2 = TreeNode::new( "2".to_string(), None );
  row2.children.push( TreeNode::new( "A".to_string(), Some( "3".to_string() ) ) );
  row2.children.push( TreeNode::new( "C".to_string(), Some( "4".to_string() ) ) );

  root.children.push( row1 );
  root.children.push( row2 );

  assert!( !root.is_table_shaped(), "mismatched column names should not be table-shaped" );
  // extract_headers still returns first row's headers regardless
  assert_eq!(
    root.extract_headers(),
    Some( vec![ "A".to_string(), "B".to_string() ] ),
    "extract_headers returns first row's headers even when not table-shaped",
  );
}
