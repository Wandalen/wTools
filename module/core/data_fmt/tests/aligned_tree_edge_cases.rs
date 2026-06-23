//! Edge case tests for `TreeFormatter` aligned tree functionality
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
//! 1. **Unicode Handling**: `visual_len()` must handle multi-byte chars correctly
//! 2. **Empty Data Graceful Handling**: Empty columns should not break layout
//! 3. **Robustness**: Algorithm should never panic, even with unusual inputs
//! 4. **Real-World Simulation**: Crate dependency tree with realistic structure
//!
//! ## Critical Edge Case: Unicode and Emoji
//!
//! **Problem**: Unicode characters like "日本語" and "😀" take multiple bytes but display as single characters
//! **Solution**: `visual_len()` counts display width, not byte length
//! **Test**: `test_aligned_tree_unicode_columns` verifies mixed unicode/ascii alignment
//!
//! ## Why These Tests Matter
//!
//! Real-world data contains unicode, emojis, empty fields, and inconsistent structure.
//! These tests ensure the alignment algorithm is production-ready.
//!
//! Split from `tests/aligned_tree.rs` in v0.4.0 compliance cleanup.

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ TreeNode, ColumnData, TreeFormatter };
use unicode_width::UnicodeWidthChar;

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
      String::new(),
      "value".to_string(),
      String::new()
    ]))
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  assert!( output.contains( "value" ), "expected \"value\" in output:\n{output}" );
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

  assert!( output.contains( "日本語" ), "expected \"日本語\" in output:\n{output}" );
  assert!( output.contains( "emoji_😀" ), "expected \"emoji_😀\" in output:\n{output}" );
  assert!( output.contains( "Русский" ), "expected \"Русский\" in output:\n{output}" );
}

/// Verifies that `format_aligned` column alignment is display-width-correct for
/// emoji and CJK content where display width differs from character count.
///
/// Three rows use first-column values with identical display width (4 columns)
/// but different character counts: emoji (2 chars), CJK (2 chars), ASCII (4 chars).
/// If pass 1 used char count, columns 1 and 2 would be measured as width 2 instead
/// of 4, producing misaligned second-column positions.
#[ test ]
fn test_aligned_tree_emoji_cjk_column_alignment_correctness()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  // Row 1: emoji column (each emoji = 2 display columns, total = 4)
  root.children.push( TreeNode::new(
    "r1".to_string(),
    Some( ColumnData::new( vec![
      "🚀🎉".to_string(),
      "val_a".to_string(),
    ]))
  ));

  // Row 2: CJK column (each CJK char = 2 display columns, total = 4)
  root.children.push( TreeNode::new(
    "r2".to_string(),
    Some( ColumnData::new( vec![
      "日本".to_string(),
      "val_b".to_string(),
    ]))
  ));

  // Row 3: ASCII column (each char = 1 display column, total = 4)
  root.children.push( TreeNode::new(
    "r3".to_string(),
    Some( ColumnData::new( vec![
      "abcd".to_string(),
      "val_c".to_string(),
    ]))
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  // Measure visual position of second-column content for each data line
  let lines : Vec< &str > = output.lines().collect();

  let visual_positions : Vec< usize > = lines.iter()
    .filter_map( | line |
    {
      let byte_pos = line.find( "val_" )?;
      let before = &line[ ..byte_pos ];
      let vpos : usize = before.chars()
        .map( | c | c.width().unwrap_or( 1 ) )
        .sum();
      Some( vpos )
    })
    .collect();

  assert!(
    visual_positions.len() >= 3,
    "expected at least 3 lines with 'val_' column; found {}; output:\n{output}",
    visual_positions.len(),
  );

  let first = visual_positions[ 0 ];
  for ( i, &pos ) in visual_positions.iter().enumerate()
  {
    assert_eq!(
      pos, first,
      "column 2 visual position mismatch: line {i} at {pos}, expected {first} (line 0)\n\
       positions: {visual_positions:?}\nfull output:\n{output}",
    );
  }
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

  assert!( output.contains( &long_string ), "expected long_string in output:\n{output}" );
  assert!( output.contains( "short" ), "expected \"short\" in output:\n{output}" );
}

// =============================================================================
// Realistic Use Case Tests
// =============================================================================

#[ test ]
fn test_aligned_tree_crate_list_simulation()
{
  let mut root = TreeNode::new( "workspace".to_string(), None );

  // Root crate with dependencies
  let api_ollama = TreeNode::new(
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
  assert!( output.contains( "api_ollama" ), "expected \"api_ollama\" in output:\n{output}" );
  assert!( output.contains( "unikit" ), "expected \"unikit\" in output:\n{output}" );
  assert!( output.contains( "llm_contract" ), "expected \"llm_contract\" in output:\n{output}" );
  assert!( output.contains( "uni_gemini" ), "expected \"uni_gemini\" in output:\n{output}" );

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

  assert!( output.contains( "no_data" ), "expected \"no_data\" in output:\n{output}" );
  // "with_data" is the node name, but node names aren't shown when there's data
  // Only the column data is shown
  assert!( output.contains( "name" ), "expected \"name\" in output:\n{output}" );
  assert!( output.contains( "value" ), "expected \"value\" in output:\n{output}" );
}
