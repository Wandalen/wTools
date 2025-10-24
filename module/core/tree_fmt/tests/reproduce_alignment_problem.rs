//! Test that reproduces the original alignment problem and demonstrates the solution
//!
//! This test simulates the exact scenario from the original request:
//! - Crate list with name, version, and path
//! - Deep dependency tree
//! - Demonstrates alignment vs non-alignment

#![ cfg( feature = "integration" ) ]
#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::{ TreeNode, ColumnData, TreeFormatter, TreeConfig, visual_len };

/// Calculate visual position of a substring in a string
fn visual_position( line : &str, target : &str ) -> Option< usize >
{
  let byte_pos = line.find( target )?;
  let before = &line[ ..byte_pos ];
  Some( visual_len( before ) )
}

/// Build the exact tree structure from the original problem statement
fn build_crate_dependency_tree() -> TreeNode< ColumnData >
{
  let mut root = TreeNode::new( "workspace".to_string(), None );

  // api_ollama - standalone crate
  root.children.push( TreeNode::new(
    "api_ollama".to_string(),
    Some( ColumnData::new( vec![
      "api_ollama".to_string(),
      "v0.1.0".to_string(),
      "(api/ollama)".to_string()
    ]))
  ));

  // as_curl - crate with dependencies
  let mut as_curl = TreeNode::new(
    "as_curl".to_string(),
    Some( ColumnData::new( vec![
      "as_curl".to_string(),
      "v0.1.0".to_string(),
      "(module/as_curl)".to_string()
    ]))
  );

  as_curl.children.push( TreeNode::new(
    "dep1".to_string(),
    Some( ColumnData::new( vec![
      "dep1".to_string(),
      "v2.0.0".to_string(),
      "(path/to/dep1)".to_string()
    ]))
  ));

  as_curl.children.push( TreeNode::new(
    "dep2".to_string(),
    Some( ColumnData::new( vec![
      "dep2".to_string(),
      "v1.5.0".to_string(),
      "(path/to/dep2)".to_string()
    ]))
  ));

  root.children.push( as_curl );

  // unikit - another standalone crate
  root.children.push( TreeNode::new(
    "unikit".to_string(),
    Some( ColumnData::new( vec![
      "unikit".to_string(),
      "v0.1.0".to_string(),
      "(service/unikit)".to_string()
    ]))
  ));

  root
}

#[ test ]
fn test_reproduce_original_problem_aligned_solution()
{
  println!( "\n=== REPRODUCING ORIGINAL ALIGNMENT PROBLEM ===" );

  let tree = build_crate_dependency_tree();

  // SOLUTION: Use TreeFormatter
  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &tree );

  println!( "\n✅ DESIRED OUTPUT (with TreeFormatter):" );
  println!( "{}", output );

  // Verify alignment by checking VISUAL column positions
  let lines : Vec< &str > = output.lines().collect();

  // Find VISUAL positions of version strings (should all be at same column)
  let version_positions : Vec< usize > = lines.iter()
    .filter_map( | line |
      visual_position( line, "v0." )
        .or_else( || visual_position( line, "v1." ) )
        .or_else( || visual_position( line, "v2." ) )
    )
    .collect();

  println!( "\nVersion column VISUAL positions: {:?}", version_positions );

  // All version strings should start at the same column position
  if version_positions.len() > 1
  {
    let first_pos = version_positions[ 0 ];
    for pos in &version_positions
    {
      assert_eq!(
        *pos, first_pos,
        "Version column not aligned! Found positions: {:?}",
        version_positions
      );
    }
    println!( "✅ Version column aligned at position {}", first_pos );
  }

  // Verify path column alignment (VISUAL positions)
  let path_positions : Vec< usize > = lines.iter()
    .filter_map( | line | visual_position( line, "(" ) )
    .collect();

  println!( "Path column VISUAL positions: {:?}", path_positions );

  if path_positions.len() > 1
  {
    let first_pos = path_positions[ 0 ];
    for pos in &path_positions
    {
      assert_eq!(
        *pos, first_pos,
        "Path column not aligned! Found positions: {:?}",
        path_positions
      );
    }
    println!( "✅ Path column aligned at position {}", first_pos );
  }

  // Verify specific content
  assert!( output.contains( "api_ollama" ) );
  assert!( output.contains( "as_curl" ) );
  assert!( output.contains( "unikit" ) );
  assert!( output.contains( "dep1" ) );
  assert!( output.contains( "dep2" ) );

  // Verify tree structure symbols present
  assert!( output.contains( "├──" ) || output.contains( "└──" ) );
  assert!( output.contains( "│" ) || output.lines().count() > 3 );

  println!( "\n✅ TEST PASSED: All columns properly aligned!" );
}

#[ test ]
fn test_reproduce_original_problem_show_unaligned()
{
  println!( "\n=== SHOWING UNALIGNED OUTPUT (OLD BEHAVIOR) ===" );

  let tree_columnar = build_crate_dependency_tree();

  // Convert to TreeNode<String> to simulate old approach
  let tree_string = convert_to_string_tree( &tree_columnar );

  let formatter = TreeFormatter::new();
  let output = formatter.format( &tree_string, | data | format!( " {}", data ) );

  println!( "\n❌ PROBLEM: Unaligned output (old TreeFormatter):" );
  println!( "{}", output );

  // Show that columns are NOT aligned (even visually, because no TreeFormatter)
  let lines : Vec< &str > = output.lines().collect();
  let version_positions : Vec< usize > = lines.iter()
    .filter_map( | line |
      visual_position( line, "v0." )
        .or_else( || visual_position( line, "v1." ) )
        .or_else( || visual_position( line, "v2." ) )
    )
    .collect();

  println!( "\nVersion VISUAL positions (NOT aligned): {:?}", version_positions );

  // Verify that positions are different (showing the problem)
  if version_positions.len() > 1
  {
    let all_same = version_positions.windows( 2 ).all( | w | w[ 0 ] == w[ 1 ] );
    assert!(
      !all_same,
      "Expected unaligned output, but columns are aligned!"
    );
    println!( "❌ Confirmed: Version column is NOT aligned (this is the problem)" );
  }

  println!( "\n❌ TEST DEMONSTRATES: Without TreeFormatter, columns don't align" );
}

#[ test ]
fn test_side_by_side_comparison()
{
  println!( "\n=== SIDE-BY-SIDE COMPARISON ===" );

  let tree = build_crate_dependency_tree();

  // Aligned version
  let formatter_aligned = TreeFormatter::new();
  let output_aligned = formatter_aligned.format_aligned( &tree );

  // Unaligned version (for comparison)
  let tree_string = convert_to_string_tree( &tree );
  let formatter_unaligned = TreeFormatter::new();
  let output_unaligned = formatter_unaligned.format( &tree_string, | data | format!( " {}", data ) );

  println!( "\n✅ ALIGNED (TreeFormatter):" );
  println!( "{}", "-".repeat( 60 ) );
  println!( "{}", output_aligned );

  println!( "\n❌ UNALIGNED (Regular TreeFormatter):" );
  println!( "{}", "-".repeat( 60 ) );
  println!( "{}", output_unaligned );

  println!( "\n📊 COMPARISON RESULT:" );
  println!( "  - Aligned version: Columns line up vertically" );
  println!( "  - Unaligned version: Ragged, hard to scan" );
  println!( "  - TreeFormatter solves the problem!" );

  // Verify aligned output is actually aligned (VISUAL positions)
  let lines_aligned : Vec< &str > = output_aligned.lines().collect();
  let version_pos_aligned : Vec< usize > = lines_aligned.iter()
    .filter_map( | line |
      visual_position( line, "v0." )
        .or_else( || visual_position( line, "v1." ) )
        .or_else( || visual_position( line, "v2." ) )
    )
    .collect();

  if version_pos_aligned.len() > 1
  {
    let first = version_pos_aligned[ 0 ];
    assert!( version_pos_aligned.iter().all( | &p | p == first ), "Aligned version should have all versions at same VISUAL column" );
  }
}

/// Helper to convert ColumnData tree to String tree
fn convert_to_string_tree( node : &TreeNode< ColumnData > ) -> TreeNode< String >
{
  let data = node.data.as_ref().map( | col_data |
    col_data.columns.join( " " )
  );

  let mut result = TreeNode::new( node.name.clone(), data );

  for child in &node.children
  {
    result.children.push( convert_to_string_tree( child ) );
  }

  result
}
