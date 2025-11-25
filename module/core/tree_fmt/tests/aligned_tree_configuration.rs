//! Configuration tests for TreeFormatter aligned tree functionality
//!
//! ## What This Tests
//!
//! Tests that TreeConfig options properly affect aligned tree output:
//! - `column_separator` - Spacing between columns (default: 2 spaces)
//! - `min_column_width` - Minimum width enforcement per column
//! - `show_root` - Root node visibility toggle
//!
//! ## Key Insights Captured
//!
//! 1. **Column Separator Customization**: Users can control spacing between columns
//! 2. **Minimum Width Enforcement**: Prevents columns from being too narrow
//! 3. **Root Visibility**: Some use cases want to hide root node in output
//! 4. **Mixed Column Counts**: Algorithm handles nodes with different column counts gracefully
//!
//! ## Design Rationale
//!
//! Configuration options allow the same alignment algorithm to serve different use cases:
//! - CLI tools: Compact spacing (1-2 spaces)
//! - Reports: Wider spacing for readability (pipe separator)
//! - Narrow terminals: Minimum width constraints prevent wrapping
//!
//! Split from tests/aligned_tree.rs in v0.4.0 compliance cleanup.

#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::{ TreeNode, ColumnData, TreeFormatter, TreeConfig };

// =============================================================================
// Column Count Variation Tests
// =============================================================================

#[ test ]
fn test_aligned_tree_mixed_column_counts()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  // Node with 2 columns
  root.children.push( TreeNode::new(
    "child1".to_string(),
    Some( ColumnData::new( vec![
      "name1".to_string(),
      "value1".to_string()
    ]))
  ));

  // Node with 3 columns
  root.children.push( TreeNode::new(
    "child2".to_string(),
    Some( ColumnData::new( vec![
      "name2".to_string(),
      "value2".to_string(),
      "extra2".to_string()
    ]))
  ));

  // Node with 1 column
  root.children.push( TreeNode::new(
    "child3".to_string(),
    Some( ColumnData::new( vec![
      "name3".to_string()
    ]))
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  assert!( output.contains( "name1" ) );
  assert!( output.contains( "name2" ) );
  assert!( output.contains( "name3" ) );
  assert!( output.contains( "extra2" ) );
}

// =============================================================================
// Configuration Tests
// =============================================================================

#[ test ]
fn test_aligned_tree_custom_separator()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  root.children.push( TreeNode::new(
    "child1".to_string(),
    Some( ColumnData::new( vec![
      "col1".to_string(),
      "col2".to_string()
    ]))
  ));

  let config = TreeConfig::new()
    .column_separator( " | ".to_string() );
  let formatter = TreeFormatter::with_config( config );
  let output = formatter.format_aligned( &root );

  assert!( output.contains( " | " ) );
}

#[ test ]
fn test_aligned_tree_min_column_width()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  root.children.push( TreeNode::new(
    "child1".to_string(),
    Some( ColumnData::new( vec![
      "a".to_string(),
      "b".to_string()
    ]))
  ));

  let config = TreeConfig::new()
    .min_column_width( 10 );
  let formatter = TreeFormatter::with_config( config );
  let output = formatter.format_aligned( &root );

  // With min width of 10, there should be significant spacing
  let lines : Vec< &str > = output.lines().collect();
  let data_line = lines.iter().find( | l | l.contains( "a" ) ).unwrap();

  // Check that line is longer than it would be without min width
  assert!( data_line.len() > 15 ); // "├──a" + spacing + "b" + spacing
}
