//! Tests for core data structures (TreeNode, RowBuilder, TableShapedView)

#![ cfg( feature = "integration" ) ]
#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::{ TreeNode, RowBuilder, TableShapedView };

// =============================================================================
// TreeNode Tests
// =============================================================================

/// Test TreeNode creation with data
#[ test ]
fn test_tree_node_with_data()
{
  let node = TreeNode::new( "root".to_string(), Some( 42u64 ) );

  assert_eq!( node.name, "root" );
  assert_eq!( node.data, Some( 42u64 ) );
  assert!( node.children.is_empty() );
}

/// Test TreeNode creation without data (directory node)
#[ test ]
fn test_tree_node_directory()
{
  let node : TreeNode< u64 > = TreeNode::new( "dir".to_string(), None );

  assert_eq!( node.name, "dir" );
  assert_eq!( node.data, None );
  assert!( node.children.is_empty() );
}

/// Test adding children to TreeNode
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

/// Test deep nesting in TreeNode
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

/// Test TreeNode clone
#[ test ]
fn test_tree_node_clone()
{
  let original = TreeNode::new( "node".to_string(), Some( 100u64 ) );
  let cloned = original.clone();

  assert_eq!( original.name, cloned.name );
  assert_eq!( original.data, cloned.data );
}

/// Test TreeNode debug formatting
#[ test ]
fn test_tree_node_debug()
{
  let node = TreeNode::new( "test".to_string(), Some( 42u64 ) );
  let debug_str = format!( "{:?}", node );

  assert!( debug_str.contains( "TreeNode" ) );
}

/// Test empty TreeNode
#[ test ]
fn test_tree_node_empty()
{
  let node : TreeNode< u64 > = TreeNode::new( "empty".to_string(), None );

  assert_eq!( node.name, "empty" );
  assert_eq!( node.data, None );
  assert!( node.children.is_empty() );
}

/// Test single TreeNode
#[ test ]
fn test_tree_node_single()
{
  let node = TreeNode::new( "single".to_string(), Some( "value".to_string() ) );

  assert_eq!( node.name, "single" );
  assert_eq!( node.data, Some( "value".to_string() ) );
}

/// Test wide TreeNode (many children)
#[ test ]
fn test_tree_node_wide()
{
  let mut root = TreeNode::new( "root".to_string(), None::<u64> );

  for i in 0..10
  {
    root.children.push( TreeNode::new( format!( "child{}", i ), Some( i as u64 ) ) );
  }

  assert_eq!( root.children.len(), 10 );
}

/// Test large TreeNode
#[ test ]
fn test_tree_node_large()
{
  let mut root = TreeNode::new( "root".to_string(), None::<String> );

  for i in 0..100
  {
    let mut child = TreeNode::new( format!( "branch{}", i ), None );
    for j in 0..10
    {
      child.children.push( TreeNode::new( format!( "leaf{}-{}", i, j ), Some( format!( "data{}-{}", i, j ) ) ) );
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
  let tree = builder.build();

  let headers = tree.extract_headers();
  assert!( headers.is_none() ); // No rows added yet
  assert_eq!( tree.children.len(), 0 );
}

#[ test ]
fn test_table_tree_builder_add_row()
{
  let mut builder = RowBuilder::new( vec![ "Name".into(), "Age".into() ] );
  builder.add_row_mut( vec![ "Alice".into(), "30".into() ] );
  let tree = builder.build();

  let rows = tree.to_rows();
  assert_eq!( rows.len(), 1 );
  assert_eq!( rows[ 0 ][ 0 ], "Alice" );
}

#[ test ]
fn test_table_tree_builder_row_count()
{
  let mut builder = RowBuilder::new( vec![ "A".into() ] );
  builder.add_row_mut( vec![ "1".into() ] );
  builder.add_row_mut( vec![ "2".into() ] );
  let tree = builder.build();

  assert_eq!( tree.children.len(), 2 );
}

#[ test ]
fn test_table_tree_builder_headers()
{
  let mut builder = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into() ] );
  builder.add_row_mut( vec![ "1".into(), "2".into(), "3".into() ] );
  let tree = builder.build();

  let headers = tree.extract_headers().unwrap();
  assert_eq!( headers.len(), 3 );
  assert_eq!( headers, vec![ "A", "B", "C" ] );
}
