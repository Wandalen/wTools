//! Basic formatting tests for TreeFormatter aligned tree functionality
//!
//! ## What This Tests
//!
//! Tests the fundamental behavior of `TreeFormatter::format_aligned()` method with
//! simple tree structures to verify column alignment works correctly.
//!
//! ## Key Insights Captured
//!
//! 1. **Empty Tree Handling**: Single root node with no children should render cleanly
//! 2. **Basic Alignment**: Two nodes with same column count should align vertically
//! 3. **Multi-Level Trees**: Alignment must work across different tree depths
//! 4. **Wide Trees**: Alignment scales to many siblings (10+ nodes)
//!
//! ## Historical Context
//!
//! Added in v0.2.0 when AlignedTreeFormatter was introduced to solve the "ragged output"
//! problem. These tests verify the two-pass algorithm (width calculation, then formatting)
//! works correctly for common cases.
//!
//! Split from tests/aligned_tree.rs (501 lines) in v0.4.0 compliance cleanup.
//!
//! ## Why Alignment Matters
//!
//! **Problem**: Without alignment, tree output with multiple columns is hard to scan:
//! ```text
//! ├── short  v1.0  (path1)
//! └── very_long_name  v2.0  (path2)
//! ```
//!
//! **Solution**: Column alignment makes data scannable:
//! ```text
//! ├── short           v1.0  (path1)
//! └── very_long_name  v2.0  (path2)
//! ```
//!
//! See tests/reproduce_alignment_problem.rs for detailed visual demonstration.

#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::{ TreeNode, ColumnData, TreeFormatter };

// =============================================================================
// Basic Formatting Tests
// =============================================================================

#[ test ]
fn test_aligned_tree_empty()
{
  let tree : TreeNode< ColumnData > = TreeNode::new( "root".to_string(), None );

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &tree );

  assert!( output.contains( "root" ) );
  assert_eq!( output.lines().count(), 1 );
}

#[ test ]
fn test_aligned_tree_single_child_two_columns()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  root.children.push( TreeNode::new(
    "child1".to_string(),
    Some( ColumnData::new( vec![
      "name1".to_string(),
      "value1".to_string()
    ]))
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  assert!( output.contains( "name1" ) );
  assert!( output.contains( "value1" ) );
  assert!( output.contains( "├──" ) || output.contains( "└──" ) );
}

#[ test ]
fn test_aligned_tree_single_child_three_columns()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  root.children.push( TreeNode::new(
    "child1".to_string(),
    Some( ColumnData::new( vec![
      "api_ollama".to_string(),
      "v0.1.0".to_string(),
      "(api/ollama)".to_string()
    ]))
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  assert!( output.contains( "api_ollama" ) );
  assert!( output.contains( "v0.1.0" ) );
  assert!( output.contains( "(api/ollama)" ) );
}

#[ test ]
fn test_aligned_tree_two_siblings_alignment()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  root.children.push( TreeNode::new(
    "child1".to_string(),
    Some( ColumnData::new( vec![
      "short".to_string(),
      "v1.0".to_string(),
      "(path1)".to_string()
    ]))
  ));

  root.children.push( TreeNode::new(
    "child2".to_string(),
    Some( ColumnData::new( vec![
      "longer_name".to_string(),
      "v2.0".to_string(),
      "(path2)".to_string()
    ]))
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  // Verify both nodes present
  assert!( output.contains( "short" ) );
  assert!( output.contains( "longer_name" ) );

  // Check that columns are aligned by verifying positions
  let lines : Vec< &str > = output.lines().collect();
  assert_eq!( lines.len(), 2 ); // 2 children (root not shown by default)

  // Find lines with data
  let line1 = lines.iter().find( | l | l.contains( "short" ) ).unwrap();
  let line2 = lines.iter().find( | l | l.contains( "longer_name" ) ).unwrap();

  // Check that version column (v1.0, v2.0) starts at same position
  let v10_pos = line1.find( "v1.0" ).unwrap();
  let v20_pos = line2.find( "v2.0" ).unwrap();
  assert_eq!( v10_pos, v20_pos, "Version column should be aligned" );
}

// =============================================================================
// Multi-Level Tree Tests
// =============================================================================

#[ test ]
fn test_aligned_tree_two_levels()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  let mut parent = TreeNode::new(
    "parent".to_string(),
    Some( ColumnData::new( vec![
      "parent_name".to_string(),
      "v1.0".to_string()
    ]))
  );

  parent.children.push( TreeNode::new(
    "child".to_string(),
    Some( ColumnData::new( vec![
      "child_name".to_string(),
      "v2.0".to_string()
    ]))
  ));

  root.children.push( parent );

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  assert!( output.contains( "parent_name" ) );
  assert!( output.contains( "child_name" ) );
  assert!( output.contains( "│" ) || output.contains( " " ) ); // Continuation or space
}

#[ test ]
fn test_aligned_tree_deep_nesting()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  let mut level1 = TreeNode::new(
    "l1".to_string(),
    Some( ColumnData::new( vec![ "level1".to_string(), "v1".to_string() ] ) )
  );

  let mut level2 = TreeNode::new(
    "l2".to_string(),
    Some( ColumnData::new( vec![ "level2".to_string(), "v2".to_string() ] ) )
  );

  let mut level3 = TreeNode::new(
    "l3".to_string(),
    Some( ColumnData::new( vec![ "level3".to_string(), "v3".to_string() ] ) )
  );

  let level4 = TreeNode::new(
    "l4".to_string(),
    Some( ColumnData::new( vec![ "level4".to_string(), "v4".to_string() ] ) )
  );

  level3.children.push( level4 );
  level2.children.push( level3 );
  level1.children.push( level2 );
  root.children.push( level1 );

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  assert!( output.contains( "level1" ) );
  assert!( output.contains( "level2" ) );
  assert!( output.contains( "level3" ) );
  assert!( output.contains( "level4" ) );
  assert_eq!( output.lines().count(), 4 ); // 4 levels (root not shown by default)
}

// =============================================================================
// Wide Tree Tests
// =============================================================================

#[ test ]
fn test_aligned_tree_many_siblings()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  for i in 0..10
  {
    root.children.push( TreeNode::new(
      format!( "child{}", i ),
      Some( ColumnData::new( vec![
        format!( "name_{}", i ),
        format!( "v{}.0", i )
      ]))
    ));
  }

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  assert!( output.contains( "name_0" ) );
  assert!( output.contains( "name_9" ) );
  assert_eq!( output.lines().count(), 10 ); // 10 children (root not shown by default)
}
