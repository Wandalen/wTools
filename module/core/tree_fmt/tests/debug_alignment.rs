//! Debug alignment calculation

#![ cfg( feature = "integration" ) ]
#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::{ TreeNode, ColumnData, TreeFormatter, visual_len };

#[ test ]
fn test_debug_prefix_visual_length()
{
  // Test visual_len on prefixes with Unicode
  let prefix1 = "";
  let prefix2 = "│   ";
  let prefix3 = "│   │   ";

  println!( "Prefix visual lengths:" );
  println!( "  Empty: {}", visual_len( prefix1 ) );
  println!( "  '│   ': {}", visual_len( prefix2 ) );
  println!( "  '│   │   ': {}", visual_len( prefix3 ) );

  // Character by character
  println!( "\nBreakdown of '│   ':" );
  for (i, ch) in prefix2.chars().enumerate() {
    println!( "  [{}] {:?} (U+{:04X})", i, ch, ch as u32 );
  }

  assert_eq!( visual_len( prefix2 ), 4 );
  assert_eq!( visual_len( prefix3 ), 8 );
}

#[ test ]
fn test_debug_alignment_calculation()
{
  let mut root = TreeNode::new( "workspace".to_string(), None );

  // Simple two-level tree with TWO children at first level (to get │)
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

  // Add second top-level node to force │ in first node's children
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
  println!( "\nLine analysis:" );
  for (i, line) in lines.iter().enumerate() {
    println!( "Line {}: len={}, bytes={}", i, visual_len(line), line.len() );
    println!( "  Content: {:?}", line );

    if let Some(pos) = line.find(" v") {
      println!( "  Version at position: {}", pos );
    }
  }
}
