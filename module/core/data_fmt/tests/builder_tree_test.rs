//! `TreeBuilder` spec tests (BL-9..BL-16)
//!
//! Covers single-leaf, nested children, intermediate nodes, batch construction,
//! empty path filtering, single-element paths, multiple siblings, batch equivalence.

#![ cfg( feature = "enabled" ) ]

use data_fmt::TreeBuilder;

/// BL-9: basic single-leaf tree
// test_kind: spec_case(BL-9)
#[ test ]
fn builder_002_bl_09_basic_single_leaf_tree()
{
  let tree = TreeBuilder::new( "project" )
    .insert( &[ "readme.md" ], 42 )
    .build();

  assert_eq!( tree.name, "project" );
  assert!( tree.data.is_none(), "root should have no data" );
  assert_eq!( tree.children.len(), 1, "root should have exactly 1 child" );

  let child = &tree.children[ 0 ];
  assert_eq!( child.name, "readme.md" );
  assert_eq!( child.data, Some( 42 ) );
  assert!( child.children.is_empty(), "leaf should have no children" );
}

/// BL-10: nested children via multi-segment path
// test_kind: spec_case(BL-10)
#[ test ]
fn builder_002_bl_10_nested_children_via_multi_segment_path()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "src", "main.rs" ], 100 )
    .build();

  assert_eq!( tree.children.len(), 1 );
  let src = &tree.children[ 0 ];
  assert_eq!( src.name, "src" );
  assert!( src.data.is_none(), "intermediate node should have no data" );
  assert_eq!( src.children.len(), 1 );

  let main_rs = &src.children[ 0 ];
  assert_eq!( main_rs.name, "main.rs" );
  assert_eq!( main_rs.data, Some( 100 ) );
}

/// BL-11: intermediate nodes created automatically
// test_kind: spec_case(BL-11)
#[ test ]
fn builder_002_bl_11_intermediate_nodes_created_automatically()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "a", "b", "c", "leaf.txt" ], 1 )
    .build();

  // Walk the path: root -> a -> b -> c -> leaf.txt
  let a = &tree.children[ 0 ];
  assert_eq!( a.name, "a" );
  assert!( a.data.is_none(), "node 'a' is intermediate" );
  assert_eq!( a.children.len(), 1 );

  let b = &a.children[ 0 ];
  assert_eq!( b.name, "b" );
  assert!( b.data.is_none(), "node 'b' is intermediate" );

  let c = &b.children[ 0 ];
  assert_eq!( c.name, "c" );
  assert!( c.data.is_none(), "node 'c' is intermediate" );

  let leaf = &c.children[ 0 ];
  assert_eq!( leaf.name, "leaf.txt" );
  assert_eq!( leaf.data, Some( 1 ), "only leaf should have data" );
}

/// BL-12: batch construction via `from_items`
// test_kind: spec_case(BL-12)
#[ test ]
fn builder_002_bl_12_batch_construction_via_from_items()
{
  #[ derive( Clone ) ]
  struct Item { path : String, _size : u64 }

  let items = vec![
    Item { path : "src/main.rs".into(), _size : 100 },
    Item { path : "src/lib.rs".into(), _size : 200 },
    Item { path : "tests/test.rs".into(), _size : 50 },
  ];

  let tree = TreeBuilder::from_items(
    &items,
    | item | item.path.split( '/' ).map( ToString::to_string ).collect(),
    Clone::clone,
  );

  assert_eq!( tree.name, "root", "from_items uses default root name 'root'" );
  // src/ and tests/ should be top-level children
  assert_eq!( tree.children.len(), 2, "should have src and tests directories" );

  // All 3 items should appear as leaves
  let mut leaf_count = 0;
  fn count_leaves< T >( node : &data_fmt::TreeNode< T >, count : &mut usize )
  {
    if node.data.is_some() { *count += 1; }
    for child in &node.children { count_leaves( child, count ); }
  }
  count_leaves( &tree, &mut leaf_count );
  assert_eq!( leaf_count, 3, "all 3 items should appear as leaf nodes" );
}

/// BL-13: empty path components are filtered out
// test_kind: spec_case(BL-13)
#[ test ]
fn builder_002_bl_13_empty_path_components_are_filtered_out()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "src", "", "main.rs" ], 10 )
    .build();

  // Should be root -> src -> main.rs (no empty-named node)
  assert_eq!( tree.children.len(), 1 );
  let src = &tree.children[ 0 ];
  assert_eq!( src.name, "src" );
  assert_eq!( src.children.len(), 1 );
  assert_eq!( src.children[ 0 ].name, "main.rs" );
  assert_eq!( src.children[ 0 ].data, Some( 10 ) );
}

/// BL-14: single-element path creates direct child
// test_kind: spec_case(BL-14)
#[ test ]
fn builder_002_bl_14_single_element_path_creates_direct_child()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "file.txt" ], 5 )
    .build();

  assert_eq!( tree.children.len(), 1, "root should have exactly 1 child" );
  let child = &tree.children[ 0 ];
  assert_eq!( child.name, "file.txt" );
  assert_eq!( child.data, Some( 5 ) );
  assert!( child.children.is_empty(), "no intermediate nodes should be created" );
}

/// BL-15: multiple siblings under same parent
// test_kind: spec_case(BL-15)
#[ test ]
fn builder_002_bl_15_multiple_siblings_under_same_parent()
{
  let tree = TreeBuilder::new( "root" )
    .insert( &[ "src", "a.rs" ], 1 )
    .insert( &[ "src", "b.rs" ], 2 )
    .insert( &[ "src", "c.rs" ], 3 )
    .build();

  assert_eq!( tree.children.len(), 1, "root should have one 'src' child" );
  let src = &tree.children[ 0 ];
  assert_eq!( src.name, "src" );
  assert!( src.data.is_none(), "src is an intermediate node" );
  assert_eq!( src.children.len(), 3, "src should have 3 children" );

  let names : Vec< &str > = src.children.iter().map( | c | c.name.as_str() ).collect();
  assert_eq!( names, vec![ "a.rs", "b.rs", "c.rs" ], "children in insertion order" );
  assert_eq!( src.children[ 0 ].data, Some( 1 ) );
  assert_eq!( src.children[ 1 ].data, Some( 2 ) );
  assert_eq!( src.children[ 2 ].data, Some( 3 ) );
}

/// BL-16: batch equivalence with manual inserts
// test_kind: spec_case(BL-16)
#[ test ]
fn builder_002_bl_16_batch_equivalence_with_manual_inserts()
{
  #[ derive( Clone, Debug, PartialEq ) ]
  struct Item { path : String, size : u64 }

  let items = vec![
    Item { path : "src/main.rs".into(), size : 100 },
    Item { path : "src/lib.rs".into(), size : 200 },
  ];

  // Tree A: batch construction
  let tree_a = TreeBuilder::from_items(
    &items,
    | item | item.path.split( '/' ).map( ToString::to_string ).collect(),
    Clone::clone,
  );

  // Tree B: manual inserts
  let tree_b = TreeBuilder::new( "root" )
    .insert( &[ "src", "main.rs" ], items[ 0 ].clone() )
    .insert( &[ "src", "lib.rs" ], items[ 1 ].clone() )
    .build();

  // Both should have same structure
  assert_eq!( tree_a.name, tree_b.name, "root names should match" );
  assert_eq!( tree_a.children.len(), tree_b.children.len(), "child count should match" );

  let src_a = &tree_a.children[ 0 ];
  let src_b = &tree_b.children[ 0 ];
  assert_eq!( src_a.name, src_b.name, "intermediate node names should match" );
  assert_eq!( src_a.children.len(), src_b.children.len(), "leaf count should match" );

  for ( leaf_a, leaf_b ) in src_a.children.iter().zip( src_b.children.iter() )
  {
    assert_eq!( leaf_a.name, leaf_b.name, "leaf names should match" );
    assert_eq!( leaf_a.data, leaf_b.data, "leaf data should match" );
  }
}
