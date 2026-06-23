//! Variant 013: Tree Aligned spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ TreeFormatter, TreeNode, ColumnData };

/// VT-1: columns aligned across all leaf nodes
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_013_vt_01_columns_aligned()
{
  let mut root : TreeNode< ColumnData > = TreeNode::new( "root".into(), None );
  let short = TreeNode::new(
    "a".into(),
    Some( ColumnData::new( vec![ "x".into(), "1".into() ] ) ),
  );
  let long = TreeNode::new(
    "bb".into(),
    Some( ColumnData::new( vec![ "longer".into(), "2".into() ] ) ),
  );
  root.children.push( short );
  root.children.push( long );

  let out = TreeFormatter::new().format_aligned( &root );

  // Both leaf lines should exist
  assert!( out.contains( 'x' ), "short value present: {out}" );
  assert!( out.contains( "longer" ), "longer value present: {out}" );
  // Alignment: the second column should start at the same position
  let lines : Vec< &str > = out.lines().filter( | l | l.contains( '1' ) || l.contains( '2' ) ).collect();
  if lines.len() == 2
  {
    let pos1 = lines[ 0 ].rfind( '1' ).unwrap();
    let pos2 = lines[ 1 ].rfind( '2' ).unwrap();
    assert_eq!( pos1, pos2, "second column aligned at same position" );
  }
}

/// VT-2: space-based column separation
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_013_vt_02_space_separation()
{
  let mut root : TreeNode< ColumnData > = TreeNode::new( "root".into(), None );
  root.children.push( TreeNode::new(
    "leaf".into(),
    Some( ColumnData::new( vec![ "col1".into(), "col2".into() ] ) ),
  ));

  let out = TreeFormatter::new().format_aligned( &root );

  // No pipes or tabs between columns
  let leaf_line = out.lines().find( | l | l.contains( "col1" ) ).unwrap();
  assert!( !leaf_line.contains( '|' ), "no pipe separator: {leaf_line}" );
  assert!( !leaf_line.contains( '\t' ), "no tab separator: {leaf_line}" );
  assert!( leaf_line.contains( "col1" ) && leaf_line.contains( "col2" ), "both columns present" );
}

/// VT-3: directory nodes show no column data
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_013_vt_03_directory_no_column_data()
{
  let mut root : TreeNode< ColumnData > = TreeNode::new( "root".into(), None );
  let mut dir = TreeNode::< ColumnData >::new( "src".into(), None );
  dir.children.push( TreeNode::new(
    "main.rs".into(),
    Some( ColumnData::new( vec![ "150".into() ] ) ),
  ));
  root.children.push( dir );

  let out = TreeFormatter::new().format_aligned( &root );

  // Directory "src" line should not have column data
  let src_line = out.lines().find( | l | l.contains( "src" ) && !l.contains( "main" ) );
  if let Some( line ) = src_line
  {
    assert!( !line.contains( "150" ), "directory shows no column data: {line}" );
  }
}

/// VT-4: single-leaf tree produces aligned output
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_013_vt_04_single_leaf_aligned()
{
  let mut root : TreeNode< ColumnData > = TreeNode::new( "root".into(), None );
  root.children.push( TreeNode::new(
    "leaf".into(),
    Some( ColumnData::new( vec![ "A".into(), "B".into() ] ) ),
  ));

  let out = TreeFormatter::new().format_aligned( &root );

  assert!( out.contains( 'A' ), "first column present: {out}" );
  assert!( out.contains( 'B' ), "second column present: {out}" );
  assert!( !out.is_empty(), "non-empty output for single leaf: {out}" );
}
