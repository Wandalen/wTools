//! Variant 012: Tree Hierarchical spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ TreeBuilder, TreeFormatter, TreeNode };

/// VT-1: output uses Unicode box-drawing connectors
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_012_vt_01_unicode_box_drawing_connectors()
{
  let tree = TreeBuilder::new( "Root" )
    .insert( &[ "Alice" ], 1 )
    .insert( &[ "Bob" ], 2 )
    .build();

  let out = TreeFormatter::new().format( &tree, ToString::to_string );

  assert!( out.contains( "├" ) || out.contains( "└" ), "box-drawing connectors present" );
}

/// VT-2: hierarchical indentation increases with depth
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_012_vt_02_indentation_increases_with_depth()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "child", "grandchild" ], 1 )
    .build();

  let out = TreeFormatter::new().format( &tree, ToString::to_string );
  let lines : Vec< &str > = out.lines().collect();

  // Find child and grandchild lines
  let child_line = lines.iter().find( | l | l.contains( "child" ) && !l.contains( "grandchild" ) );
  let grandchild_line = lines.iter().find( | l | l.contains( "grandchild" ) );

  assert!( child_line.is_some(), "child line exists" );
  assert!( grandchild_line.is_some(), "grandchild line exists" );

  // Grandchild has more leading whitespace/connectors
  let child_indent = child_line.unwrap().len() - child_line.unwrap().trim_start().len();
  let gc_indent = grandchild_line.unwrap().len() - grandchild_line.unwrap().trim_start().len();
  assert!( gc_indent > child_indent, "grandchild indented deeper than child" );
}

/// VT-3: leaf nodes display data via render closure
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_012_vt_03_leaf_displays_data()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "src", "main.rs" ], 150i64 )
    .build();

  let out = TreeFormatter::new().format( &tree, | v | format!( "{v} lines" ) );

  assert!( out.contains( "150 lines" ), "leaf data rendered via closure" );
  // Directory "src" has no data rendered (only leaf nodes have data)
  let src_line = out.lines().find( | l | l.contains( "src" ) && !l.contains( "main" ) );
  if let Some( line ) = src_line
  {
    assert!( !line.contains( "lines" ), "directory node has no data rendered" );
  }
}

/// VT-4: single-node tree produces root-only output
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_012_vt_04_single_node_root_only()
{
  let tree : TreeNode< i32 > = TreeNode::new( "Root".into(), None );
  let out = TreeFormatter::new().format( &tree, ToString::to_string );

  assert!( out.contains( "Root" ), "root name present" );
  assert!( !out.contains( "├" ), "no branch connector" );
  assert!( !out.contains( "└" ), "no last-branch connector" );
  assert!( !out.contains( "│" ), "no vertical connector" );
}
