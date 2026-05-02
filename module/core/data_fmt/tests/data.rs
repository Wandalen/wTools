//! Tests for core data structures (`TreeNode`, `RowBuilder`, `TableView`)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ TreeNode, RowBuilder, TableFormatter, Format };

// =============================================================================
// TreeNode Tests
// =============================================================================

/// Test `TreeNode` creation with data
#[ test ]
fn test_tree_node_with_data()
{
  let node = TreeNode::new( "root".to_string(), Some( 42u64 ) );

  assert_eq!( node.name, "root" );
  assert_eq!( node.data, Some( 42u64 ) );
  assert!( node.children.is_empty() );
}

/// Test `TreeNode` creation without data (directory node)
#[ test ]
fn test_tree_node_directory()
{
  let node : TreeNode< u64 > = TreeNode::new( "dir".to_string(), None );

  assert_eq!( node.name, "dir" );
  assert_eq!( node.data, None );
  assert!( node.children.is_empty() );
}

/// Test adding children to `TreeNode`
#[ test ]
fn test_tree_node_add_children()
{
  let mut root = TreeNode::new( "root".to_string(), None::<u64> );
  let child1 = TreeNode::new( "child1".to_string(), Some( 1u64 ) );
  let child2 = TreeNode::new( "child2".to_string(), Some( 2u64 ) );

  root.children.push( child1 );
  root.children.push( child2 );

  assert_eq!( root.children.len(), 2 );
  assert_eq!( root.children[ 0 ].name, "child1" );
  assert_eq!( root.children[ 1 ].name, "child2" );
}

/// Test deep nesting in `TreeNode`
#[ test ]
fn test_tree_node_deep_nesting()
{
  let mut root = TreeNode::new( "root".to_string(), None::<u64> );
  let mut level1 = TreeNode::new( "level1".to_string(), None );
  let mut level2 = TreeNode::new( "level2".to_string(), None );
  let leaf = TreeNode::new( "leaf".to_string(), Some( 42u64 ) );

  level2.children.push( leaf );
  level1.children.push( level2 );
  root.children.push( level1 );

  assert_eq!( root.children.len(), 1 );
  assert_eq!( root.children[ 0 ].children.len(), 1 );
  assert_eq!( root.children[ 0 ].children[ 0 ].children.len(), 1 );
  assert_eq!( root.children[ 0 ].children[ 0 ].children[ 0 ].data, Some( 42u64 ) );
}

/// Test `TreeNode` clone
#[ test ]
fn test_tree_node_clone()
{
  let original = TreeNode::new( "node".to_string(), Some( 100u64 ) );
  let cloned = original.clone();

  assert_eq!( original.name, cloned.name );
  assert_eq!( original.data, cloned.data );
}

/// Test `TreeNode` debug formatting
#[ test ]
fn test_tree_node_debug()
{
  let node = TreeNode::new( "test".to_string(), Some( 42u64 ) );
  let debug_str = format!( "{node:?}" );

  assert!( debug_str.contains( "TreeNode" ) );
}

/// Test empty `TreeNode`
#[ test ]
fn test_tree_node_empty()
{
  let node : TreeNode< u64 > = TreeNode::new( "empty".to_string(), None );

  assert_eq!( node.name, "empty" );
  assert_eq!( node.data, None );
  assert!( node.children.is_empty() );
}

/// Test single `TreeNode`
#[ test ]
fn test_tree_node_single()
{
  let node = TreeNode::new( "single".to_string(), Some( "value".to_string() ) );

  assert_eq!( node.name, "single" );
  assert_eq!( node.data, Some( "value".to_string() ) );
}

/// Test wide `TreeNode` (many children)
#[ test ]
fn test_tree_node_wide()
{
  let mut root = TreeNode::new( "root".to_string(), None::<u64> );

  for i in 0u64..10
  {
    root.children.push( TreeNode::new( format!( "child{i}" ), Some( i ) ) );
  }

  assert_eq!( root.children.len(), 10 );
}

/// Test large `TreeNode`
#[ test ]
fn test_tree_node_large()
{
  let mut root = TreeNode::new( "root".to_string(), None::<String> );

  for i in 0..100
  {
    let mut child = TreeNode::new( format!( "branch{i}" ), None );
    for j in 0..10
    {
      child.children.push( TreeNode::new( format!( "leaf{i}-{j}" ), Some( format!( "data{i}-{j}" ) ) ) );
    }
    root.children.push( child );
  }

  assert_eq!( root.children.len(), 100 );
  assert_eq!( root.children[ 0 ].children.len(), 10 );
}

// =============================================================================
// RowBuilder Tests
// =============================================================================

#[ test ]
fn test_table_tree_builder_new()
{
  let builder = RowBuilder::new( vec![ "Name".into(), "Age".into() ] );
  let view = builder.build_view();

  assert_eq!( view.rows.len(), 0 ); // No rows added yet
  assert_eq!( view.metadata.column_names.len(), 2 );
}

#[ test ]
fn test_table_tree_builder_add_row()
{
  let mut builder = RowBuilder::new( vec![ "Name".into(), "Age".into() ] );
  builder.add_row_mut( vec![ "Alice".into(), "30".into() ] );
  let view = builder.build_view();

  assert_eq!( view.rows.len(), 1 );
  assert_eq!( view.rows[ 0 ][ 0 ].render(), "Alice" );
}

#[ test ]
fn test_table_tree_builder_row_count()
{
  let mut builder = RowBuilder::new( vec![ "A".into() ] );
  builder.add_row_mut( vec![ "1".into() ] );
  builder.add_row_mut( vec![ "2".into() ] );
  let view = builder.build_view();

  assert_eq!( view.rows.len(), 2 );
}

#[ test ]
fn test_table_tree_builder_headers()
{
  let mut builder = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into() ] );
  builder.add_row_mut( vec![ "1".into(), "2".into(), "3".into() ] );
  let view = builder.build_view();

  let column_names = &view.metadata.column_names;
  assert_eq!( column_names.len(), 3 );
  assert_eq!( column_names[ 0 ], "A" );
  assert_eq!( column_names[ 1 ], "B" );
  assert_eq!( column_names[ 2 ], "C" );
}

// IC-1 — invariant/001: row shorter than headers triggers assert!
// Enforcement: src/table_tree.rs:52 validate_row_length()
#[ test ]
#[ should_panic( expected = "row length 2 doesn't match headers length 3" ) ]
fn row_builder_panics_when_row_shorter_than_headers()
{
  // panic fires inside add_row before the return value is produced; let _ silences must_use
  let _ = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] );
}

// IC-2 — invariant/001: row longer than headers triggers assert!
// Enforcement: src/table_tree.rs:52 validate_row_length()
#[ test ]
#[ should_panic( expected = "row length 4 doesn't match headers length 3" ) ]
fn row_builder_panics_when_row_longer_than_headers()
{
  let _ = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into() ] )
    .add_row( vec![ "x".into(), "y".into(), "z".into(), "w".into() ] );
}

/// IC-3 — `invariant/001`: headers-only table renders header row and separator.
///
/// `RowBuilder::new(headers).build_view()` with no rows produces a `TableView` with
/// headers in `metadata.column_names` and `rows=[]`. The formatter renders the header
/// row and separator as a useful "empty state" display — only a truly empty table
/// (no columns at all) renders to `""`.
///
/// # Root Cause
/// Prior guard was `if rows.is_empty() { return String::new(); }`, which suppressed
/// the header even when columns were defined, hiding the table structure entirely.
///
/// # Fix Applied
/// Guard changed to `if headers.is_empty()`: only suppress output when there are no
/// columns. Non-empty headers produce header + separator regardless of row count.
///
/// # Pitfall
/// Never guard on `rows.is_empty()` alone — that silently hides the column structure
/// from callers who render empty result sets but still need to see the headers.
#[ test ]
fn empty_table_renders_to_empty_string()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .build_view();
  let output = TableFormatter::new().format( &view ).unwrap_or_default();
  assert!(
    output.contains( "Name" ),
    "headers-only table must render header row, got: {output:?}",
  );
  assert!(
    output.contains( "Value" ),
    "headers-only table must render all column names, got: {output:?}",
  );
  assert!(
    output.lines().count() <= 2,
    "headers-only table must have at most header + separator lines, got: {output:?}",
  );
}

/// IC-4 — `invariant/001` EC-3: single-row table renders without error.
///
/// Sanity check that a minimal table (1 header row + 1 data row) produces the
/// expected three non-empty output lines: column header, separator, data row.
/// Cell values must appear verbatim in the output.
#[ test ]
fn single_row_table_renders_without_error()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Score".into() ] )
    .add_row( vec![ "Alice".into(), "99".into() ] )
    .build_view();
  let output = TableFormatter::new().format( &view ).unwrap_or_default();
  assert!(
    output.contains( "Alice" ),
    "single-row output must contain first cell value: {output:?}",
  );
  assert!(
    output.contains( "99" ),
    "single-row output must contain second cell value: {output:?}",
  );
  // header line + separator + 1 data line = 3 non-empty lines
  let non_empty : Vec< &str > = output.lines().filter( | l | !l.trim().is_empty() ).collect();
  assert_eq!(
    non_empty.len(), 3,
    "single-row table must have exactly 3 non-empty lines (header+sep+data): {output:?}",
  );
}
