//! Reproduce the exact alignment issue seen in willbe3 output

#![ cfg( feature = "integration" ) ]
#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::{ TreeNode, ColumnData, TreeFormatter, TreeConfig, visual_len };

/// Calculate visual position of a substring in a string
fn visual_position( line : &str, target : &str ) -> Option< usize >
{
  // Find byte position first
  let byte_pos = line.find( target )?;

  // Calculate visual position by counting chars up to byte_pos
  let before = &line[ ..byte_pos ];
  Some( visual_len( before ) )
}

/// Build the exact tree from willbe3 .crates.list format::tree output
fn build_willbe3_crate_tree() -> TreeNode< ColumnData >
{
  let mut root = TreeNode::new( "workspace".to_string(), None );

  // mindful with dependencies
  let mut mindful = TreeNode::new(
    "mindful".to_string(),
    Some( ColumnData::new( vec![
      "mindful".to_string(),
      "0.1.0".to_string(),
      "module/mindful".to_string()
    ]))
  );

  mindful.children.push( TreeNode::new(
    "mindful_core".to_string(),
    Some( ColumnData::new( vec![
      "mindful_core".to_string(),
      "0.1.0".to_string(),
      "module/mindful_core".to_string()
    ]))
  ));

  mindful.children.push( TreeNode::new(
    "tree_fmt".to_string(),
    Some( ColumnData::new( vec![
      "tree_fmt".to_string(),
      "0.2.0".to_string(),
      "module/tree_fmt".to_string()
    ]))
  ));

  root.children.push( mindful );

  // wflow with deep nesting
  let mut wflow = TreeNode::new(
    "wflow".to_string(),
    Some( ColumnData::new( vec![
      "wflow".to_string(),
      "0.2.0".to_string(),
      "module/wflow".to_string()
    ]))
  );

  let mut wflow_core = TreeNode::new(
    "wflow_core".to_string(),
    Some( ColumnData::new( vec![
      "wflow_core".to_string(),
      "0.2.0".to_string(),
      "module/wflow_core".to_string()
    ]))
  );

  let mut file_finder = TreeNode::new(
    "file_finder".to_string(),
    Some( ColumnData::new( vec![
      "file_finder".to_string(),
      "0.1.0".to_string(),
      "module/file_finder".to_string()
    ]))
  );

  file_finder.children.push( TreeNode::new(
    "lang_detect".to_string(),
    Some( ColumnData::new( vec![
      "lang_detect".to_string(),
      "0.1.0".to_string(),
      "module/lang_detect".to_string()
    ]))
  ));

  wflow_core.children.push( file_finder );

  wflow_core.children.push( TreeNode::new(
    "lang_detect".to_string(),
    Some( ColumnData::new( vec![
      "lang_detect".to_string(),
      "0.1.0".to_string(),
      "module/lang_detect".to_string()
    ]))
  ));

  wflow_core.children.push( TreeNode::new(
    "tree_fmt".to_string(),
    Some( ColumnData::new( vec![
      "tree_fmt".to_string(),
      "0.2.0".to_string(),
      "module/tree_fmt".to_string()
    ]))
  ));

  wflow.children.push( wflow_core );

  root.children.push( wflow );

  root
}

#[ test ]
fn test_reproduce_willbe3_alignment_issue()
{
  println!( "\n=== REPRODUCING WILLBE3 ALIGNMENT ISSUE ===" );

  let tree = build_willbe3_crate_tree();

  let config = TreeConfig::new()
    .column_separator( "  ".to_string() );
  let formatter = TreeFormatter::with_config( config );

  let output = formatter.format_aligned( &tree );

  println!( "\nGenerated output:" );
  println!( "{}", output );

  // Verify alignment programmatically
  let lines : Vec< &str > = output.lines().collect();

  // Find all lines with version numbers
  let version_lines : Vec< &str > = lines.iter()
    .filter( | l | l.contains( "0.1.0" ) || l.contains( "0.2.0" ) )
    .copied()
    .collect();

  println!( "\nAnalyzing {} version lines:", version_lines.len() );

  // Find VISUAL position of version column (0.1.0 or 0.2.0) in each line
  let version_positions : Vec< ( usize, usize ) > = version_lines.iter()
    .enumerate()
    .filter_map( | ( idx, line ) | {
      visual_position( line, "0.1.0" )
        .or_else( || visual_position( line, "0.2.0" ) )
        .map( | pos | ( idx, pos ) )
    })
    .collect();

  println!( "\nVersion column positions (VISUAL, not byte):" );
  for ( idx, pos ) in &version_positions
  {
    println!( "  Line {}: visual position {} - {}", idx, pos, version_lines[ *idx ].trim() );
  }

  // Check if all positions are the same
  if version_positions.len() > 1
  {
    let first_pos = version_positions[ 0 ].1;
    let all_aligned = version_positions.iter().all( | ( _, pos ) | *pos == first_pos );

    if all_aligned
    {
      println!( "\n✅ All version columns aligned at position {}", first_pos );
    }
    else
    {
      println!( "\n❌ Version columns NOT aligned!" );
      println!( "   Positions: {:?}", version_positions.iter().map( | ( _, p ) | p ).collect::< Vec< _ > >() );

      // Show which positions differ
      for ( idx, pos ) in &version_positions
      {
        if *pos != first_pos
        {
          println!( "   Line {} differs: position {} instead of {}", idx, pos, first_pos );
        }
      }
    }

    assert!( all_aligned, "Version columns should be aligned at same position!" );
  }

  // Check path column alignment too (VISUAL position)
  let path_positions : Vec< ( usize, usize ) > = version_lines.iter()
    .enumerate()
    .filter_map( | ( idx, line ) | {
      visual_position( line, "module/" )
        .map( | pos | ( idx, pos ) )
    })
    .collect();

  if path_positions.len() > 1
  {
    let first_pos = path_positions[ 0 ].1;
    let all_aligned = path_positions.iter().all( | ( _, pos ) | *pos == first_pos );

    println!( "\nPath column positions:" );
    for ( idx, pos ) in &path_positions
    {
      println!( "  Line {}: position {}", idx, pos );
    }

    if all_aligned
    {
      println!( "✅ All path columns aligned at position {}", first_pos );
    }
    else
    {
      println!( "❌ Path columns NOT aligned!" );
    }

    assert!( all_aligned, "Path columns should be aligned at same position!" );
  }

  println!( "\n✅ TEST PASSED: All columns properly aligned across all depths!" );
}

#[ test ]
fn test_manual_inspection_willbe3_format()
{
  println!( "\n=== MANUAL INSPECTION: WILLBE3 FORMAT ===" );
  println!( "Compare this output with actual willbe3 output\n" );

  let tree = build_willbe3_crate_tree();

  let config = TreeConfig::new()
    .column_separator( "  ".to_string() );
  let formatter = TreeFormatter::with_config( config );

  let output = formatter.format_aligned( &tree );

  println!( "{}", output );

  println!( "\nExpected behavior:" );
  println!( "- Version column (0.1.0, 0.2.0) should align vertically" );
  println!( "- Path column (module/...) should align vertically" );
  println!( "- Alignment should work across ALL depth levels" );
}
