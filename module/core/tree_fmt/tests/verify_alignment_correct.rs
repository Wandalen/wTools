//! Verify that alignment is actually correct by measuring visual positions

#![ cfg( feature = "integration" ) ]
#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::{ TreeNode, ColumnData, TreeFormatter, visual_len };

/// Calculate visual position of a substring in a string
fn visual_position( line : &str, target : &str ) -> Option< usize >
{
  // Find byte position first
  let byte_pos = line.find( target )?;

  // Calculate visual position by counting chars up to byte_pos
  let before = &line[ ..byte_pos ];
  Some( visual_len( before ) )
}

#[ test ]
fn test_verify_willbe3_alignment_is_correct()
{
  println!( "\n=== VERIFYING ALIGNMENT WITH VISUAL POSITIONS ===" );

  let mut root = TreeNode::new( "workspace".to_string(), None );

  // Test with │ characters
  let mut parent = TreeNode::new(
    "parent".to_string(),
    Some( ColumnData::new( vec![
      "short".to_string(),
      "v1".to_string()
    ]))
  );

  parent.children.push( TreeNode::new(
    "child".to_string(),
    Some( ColumnData::new( vec![
      "longer_name".to_string(),
      "v2".to_string()
    ]))
  ));

  root.children.push( parent );

  root.children.push( TreeNode::new(
    "sibling".to_string(),
    Some( ColumnData::new( vec![
      "another".to_string(),
      "v3".to_string()
    ]))
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  println!( "\nGenerated output:" );
  println!( "{}", output );

  let lines : Vec< &str > = output.lines().collect();

  println!( "\nPosition analysis (VISUAL positions):" );
  for (i, line) in lines.iter().enumerate()
  {
    let byte_pos = line.find( " v" );
    let visual_pos = visual_position( line, " v" );

    println!( "Line {}: byte_pos={:?}, visual_pos={:?}", i, byte_pos, visual_pos );
    println!( "  {}", line );
  }

  // Get visual positions
  let visual_positions : Vec< usize > = lines.iter()
    .filter_map( | line | visual_position( line, " v" ) )
    .collect();

  println!( "\nVisual positions of ' v': {:?}", visual_positions );

  // All should be the same
  if visual_positions.len() > 1
  {
    let first = visual_positions[ 0 ];
    let all_same = visual_positions.iter().all( | &p | p == first );

    if all_same
    {
      println!( "✅ All columns aligned at visual position {}", first );
    }
    else
    {
      println!( "❌ Columns NOT aligned!" );
    }

    assert!( all_same, "All visual positions should be the same!" );
  }

  println!( "\n✅ ALIGNMENT IS CORRECT (when measured visually)!" );
}
