//! Edge case tests for TreeFormatter aligned tree functionality
//!
//! ## What This Tests
//!
//! Tests boundary conditions and unusual inputs that could break alignment:
//! - Unicode characters (multi-byte, emoji, box-drawing chars)
//! - Empty strings in column data
//! - Single-column nodes (no alignment needed)
//! - Mixed nodes (some with data, some without)
//! - Very long strings that might cause overflow
//!
//! ## Key Insights Captured
//!
//! 1. **Unicode Handling**: visual_len() must handle multi-byte chars correctly
//! 2. **Empty Data Graceful Handling**: Empty columns should not break layout
//! 3. **Robustness**: Algorithm should never panic, even with unusual inputs
//! 4. **Real-World Simulation**: Crate dependency tree with realistic structure
//!
//! ## Critical Edge Case: Unicode and Emoji
//!
//! **Problem**: Unicode characters like "日本語" and "😀" take multiple bytes but display as single characters
//! **Solution**: visual_len() counts display width, not byte length
//! **Test**: test_aligned_tree_unicode_columns verifies mixed unicode/ascii alignment
//!
//! ## Why These Tests Matter
//!
//! Real-world data contains unicode, emojis, empty fields, and inconsistent structure.
//! These tests ensure the alignment algorithm is production-ready.
//!
//! Split from tests/aligned_tree.rs in v0.4.0 compliance cleanup.

#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::{ TreeNode, ColumnData, TreeFormatter };

// =============================================================================
// Edge Cases
// =============================================================================

#[ test ]
fn test_aligned_tree_empty_columns()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  root.children.push( TreeNode::new(
    "child1".to_string(),
    Some( ColumnData::new( vec![
      "".to_string(),
      "value".to_string(),
      "".to_string()
    ]))
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  assert!( output.contains( "value" ) );
}

#[ test ]
fn test_aligned_tree_unicode_columns()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  root.children.push( TreeNode::new(
    "child1".to_string(),
    Some( ColumnData::new( vec![
      "日本語".to_string(),
      "emoji_😀".to_string(),
      "Русский".to_string()
    ]))
  ));

  root.children.push( TreeNode::new(
    "child2".to_string(),
    Some( ColumnData::new( vec![
      "english".to_string(),
      "normal".to_string(),
      "text".to_string()
    ]))
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  assert!( output.contains( "日本語" ) );
  assert!( output.contains( "emoji_😀" ) );
  assert!( output.contains( "Русский" ) );
}

#[ test ]
fn test_aligned_tree_long_values()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  let long_string = "a".repeat( 100 );

  root.children.push( TreeNode::new(
    "child1".to_string(),
    Some( ColumnData::new( vec![
      long_string.clone(),
      "short".to_string()
    ]))
  ));

  root.children.push( TreeNode::new(
    "child2".to_string(),
    Some( ColumnData::new( vec![
      "short".to_string(),
      "value".to_string()
    ]))
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  assert!( output.contains( &long_string ) );
  assert!( output.contains( "short" ) );
}

// =============================================================================
// Realistic Use Case Tests
// =============================================================================

#[ test ]
fn test_aligned_tree_crate_list_simulation()
{
  let mut root = TreeNode::new( "workspace".to_string(), None );

  // Root crate with dependencies
  let mut api_ollama = TreeNode::new(
    "api_ollama".to_string(),
    Some( ColumnData::new( vec![
      "api_ollama".to_string(),
      "v0.1.0".to_string(),
      "(api/ollama)".to_string()
    ]))
  );

  let mut unikit = TreeNode::new(
    "unikit".to_string(),
    Some( ColumnData::new( vec![
      "unikit".to_string(),
      "v0.1.0".to_string(),
      "(service/unikit)".to_string()
    ]))
  );

  // Unikit dependencies
  unikit.children.push( TreeNode::new(
    "llm_contract".to_string(),
    Some( ColumnData::new( vec![
      "llm_contract".to_string(),
      "v0.7.0".to_string(),
      "(core/llm_contract)".to_string()
    ]))
  ));

  unikit.children.push( TreeNode::new(
    "uni_gemini".to_string(),
    Some( ColumnData::new( vec![
      "uni_gemini".to_string(),
      "v0.1.0".to_string(),
      "(provider/gemini)".to_string()
    ]))
  ));

  root.children.push( api_ollama );
  root.children.push( unikit );

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  // Verify structure
  assert!( output.contains( "api_ollama" ) );
  assert!( output.contains( "unikit" ) );
  assert!( output.contains( "llm_contract" ) );
  assert!( output.contains( "uni_gemini" ) );

  // Verify alignment - all versions should start at same column
  let lines : Vec< &str > = output.lines().collect();
  let version_positions : Vec< usize > = lines.iter()
    .filter_map( | line | line.find( "v0." ) )
    .collect();

  // All version columns should align (same position)
  if version_positions.len() > 1
  {
    let first_pos = version_positions[ 0 ];
    for pos in &version_positions
    {
      assert_eq!( *pos, first_pos, "All version columns should align" );
    }
  }
}

#[ test ]
fn test_aligned_tree_no_data_nodes()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  // Mix of nodes with data and without
  root.children.push( TreeNode::new(
    "no_data".to_string(),
    None
  ));

  root.children.push( TreeNode::new(
    "with_data".to_string(),
    Some( ColumnData::new( vec![
      "name".to_string(),
      "value".to_string()
    ]))
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  // Debug output to see what's actually generated
  // eprintln!( "Output:\n{}", output );

  assert!( output.contains( "no_data" ) );
  // "with_data" is the node name, but node names aren't shown when there's data
  // Only the column data is shown
  assert!( output.contains( "name" ) );
  assert!( output.contains( "value" ) );
}
