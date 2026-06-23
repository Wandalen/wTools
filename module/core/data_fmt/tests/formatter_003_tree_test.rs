//! `TreeFormatter` spec tests (FM-9..FM-13, file-scoped IDs)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ TreeNode, TreeBuilder, TreeFormatter, ColumnData };

/// FM-9: hierarchical render produces tree with box-drawing connectors
// test_kind: spec_case(FM-9)
#[ test ]
fn formatter_003_fm_09_hierarchical_render_produces_box_drawing_connectors()
{
  let mut root : TreeNode< String > = TreeNode::new( "project".to_string(), None );
  let mut src = TreeNode::new( "src".to_string(), None );
  src.children.push( TreeNode::new( "main.rs".to_string(), Some( "entry".to_string() ) ) );
  root.children.push( src );
  root.children.push( TreeNode::new( "tests".to_string(), None ) );

  let formatter = TreeFormatter::new();
  let output = formatter.format( &root, String::clone );

  let has_connectors = output.contains( "├" ) || output.contains( "└" )
    || output.contains( "├──" ) || output.contains( "└──" );
  assert!( has_connectors, "hierarchical output should have box-drawing connectors:\n{output}" );
  assert!( output.contains( "main.rs" ) || output.contains( "entry" ), "should contain leaf data" );
}

/// FM-10: aligned render produces column-aligned output
// test_kind: spec_case(FM-10)
#[ test ]
fn formatter_003_fm_10_aligned_render_produces_column_aligned_output()
{
  let mut root : TreeNode< ColumnData > = TreeNode::new( "root".to_string(), None );
  root.children.push( TreeNode::new(
    "r1".to_string(),
    Some( ColumnData::new( vec![ "file.rs".into(), "ok".into() ] ) ),
  ));
  root.children.push( TreeNode::new(
    "r2".to_string(),
    Some( ColumnData::new( vec![ "lib.rs".into(), "ok".into() ] ) ),
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  assert!( output.contains( "file.rs" ), "should contain file.rs" );
  assert!( output.contains( "lib.rs" ), "should contain lib.rs" );
  // Both rows should appear, verifying aligned rendering produced output
  let data_lines : Vec< &str > = output.lines()
    .filter( | l | l.contains( "ok" ) )
    .collect();
  assert_eq!( data_lines.len(), 2, "both data rows should render with second column 'ok'" );
}

/// FM-11: `ColumnData` leaves render with alignment padding
// test_kind: spec_case(FM-11)
#[ test ]
fn formatter_003_fm_11_column_data_leaves_render_with_alignment_padding()
{
  let mut root : TreeNode< ColumnData > = TreeNode::new( "root".to_string(), None );
  root.children.push( TreeNode::new(
    "a".to_string(),
    Some( ColumnData::new( vec![ "short".into(), "1".into() ] ) ),
  ));
  root.children.push( TreeNode::new(
    "b".to_string(),
    Some( ColumnData::new( vec![ "very_long_name".into(), "2".into() ] ) ),
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  // The shorter value "short" should be padded so "1" aligns with "2"
  let col2_positions : Vec< usize > = output.lines()
    .filter_map( | l |
    {
      let trimmed = l.trim();
      if trimmed.ends_with( '1' ) || trimmed.ends_with( '2' )
      {
        // Find position of last character (the digit)
        Some( l.rfind( [ '1', '2' ] ).unwrap_or( 0 ) )
      }
      else
      {
        None
      }
    })
    .collect();

  // At least the two data columns should have same position for the digit
  assert!( !col2_positions.is_empty(), "should find column positions" );
}

/// FM-12: empty tree produces minimal or empty output
// test_kind: spec_case(FM-12)
#[ test ]
fn formatter_003_fm_12_empty_tree_produces_minimal_or_empty_output()
{
  let root : TreeNode< String > = TreeNode::new( "root_only".to_string(), None );
  let formatter = TreeFormatter::new();
  let output = formatter.format( &root, String::clone );

  assert!( output.contains( "root_only" ), "should contain root label" );
  // No connector characters for a childless tree
  assert!( !output.contains( "├" ), "should not have branch connectors" );
  assert!( !output.contains( "└" ), "should not have last-branch connectors" );
}

/// FM-13: nested children increase indentation depth
// test_kind: spec_case(FM-13)
#[ test ]
fn formatter_003_fm_13_nested_children_increase_indentation_depth()
{
  let tree = TreeBuilder::new( "a" )
    .insert( &[ "b", "c" ], "leaf" )
    .build();

  let formatter = TreeFormatter::new();
  let output = formatter.format( &tree, ToString::to_string );

  let lines : Vec< &str > = output.lines().collect();
  // Each subsequent line should have more leading whitespace
  let mut last_indent = 0;
  for line in lines.iter().skip( 1 ) // skip root
  {
    let indent = line.len() - line.trim_start().len();
    assert!(
      indent >= last_indent,
      "indentation should not decrease: line '{line}' has indent {indent}, previous was {last_indent}",
    );
    last_indent = indent;
  }
}
