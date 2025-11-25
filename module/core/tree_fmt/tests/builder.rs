//! Tests for TreeBuilder

#![ cfg( feature = "integration" ) ]
#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::TreeBuilder;
use std::path::PathBuf;

// Phase 2: TreeBuilder Tests (15 tests)
// =============================================================================

/// Test TreeBuilder creation
#[ test ]
fn test_tree_builder_new()
{
  use tree_fmt::TreeBuilder;

  let builder : TreeBuilder< i32 > = TreeBuilder::new( "root" );
  let tree = builder.build();

  assert_eq!( tree.name, "root" );
  assert!( tree.children.is_empty() );
}

/// Test build from single path
#[ test ]
fn test_tree_builder_single_path()
{
  use tree_fmt::TreeBuilder;

  let tree = TreeBuilder::new( "root" )
    .insert( &[ "dir", "file.txt" ], 100 )
    .build();

  assert_eq!( tree.children.len(), 1 );
  assert_eq!( tree.children[ 0 ].name, "dir" );
  assert_eq!( tree.children[ 0 ].children.len(), 1 );
  assert_eq!( tree.children[ 0 ].children[ 0 ].name, "file.txt" );
  assert_eq!( tree.children[ 0 ].children[ 0 ].data, Some( 100 ) );
}

/// Test build from multiple paths at root
#[ test ]
fn test_tree_builder_multiple_root_paths()
{
  use tree_fmt::TreeBuilder;

  let tree = TreeBuilder::new( "root" )
    .insert( &[ "file1.txt" ], 10 )
    .insert( &[ "file2.txt" ], 20 )
    .insert( &[ "file3.txt" ], 30 )
    .build();

  assert_eq!( tree.children.len(), 3 );
  assert_eq!( tree.children[ 0 ].data, Some( 10 ) );
  assert_eq!( tree.children[ 1 ].data, Some( 20 ) );
  assert_eq!( tree.children[ 2 ].data, Some( 30 ) );
}

/// Test build from nested paths (3 levels)
#[ test ]
fn test_tree_builder_nested_paths()
{
  use tree_fmt::TreeBuilder;

  let tree = TreeBuilder::new( "root" )
    .insert( &[ "src", "lib", "mod.rs" ], 100 )
    .insert( &[ "src", "bin", "main.rs" ], 50 )
    .build();

  assert_eq!( tree.children.len(), 1 );
  assert_eq!( tree.children[ 0 ].name, "src" );
  assert_eq!( tree.children[ 0 ].children.len(), 2 );
}

/// Test build with duplicate paths (should merge)
#[ test ]
fn test_tree_builder_duplicate_paths()
{
  use tree_fmt::TreeBuilder;

  let tree = TreeBuilder::new( "root" )
    .insert( &[ "dir", "file1.txt" ], 10 )
    .insert( &[ "dir", "file2.txt" ], 20 )
    .build();

  assert_eq!( tree.children.len(), 1 );
  assert_eq!( tree.children[ 0 ].name, "dir" );
  assert_eq!( tree.children[ 0 ].children.len(), 2 );
}

/// Test build with path containing spaces
#[ test ]
fn test_tree_builder_paths_with_spaces()
{
  use tree_fmt::TreeBuilder;

  let tree = TreeBuilder::new( "root" )
    .insert( &[ "my dir", "my file.txt" ], 42 )
    .build();

  assert_eq!( tree.children[ 0 ].name, "my dir" );
  assert_eq!( tree.children[ 0 ].children[ 0 ].name, "my file.txt" );
}

/// Test build from items with path extractor
#[ test ]
fn test_tree_builder_from_items()
{
  use tree_fmt::TreeBuilder;

  #[ derive( Clone ) ]
  #[ allow( dead_code ) ]
  struct FileItem
  {
    path : String,
    size : u64,
  }

  let items = vec![
    FileItem { path : "src/main.rs".to_string(), size : 100 },
    FileItem { path : "src/lib.rs".to_string(), size : 200 },
    FileItem { path : "tests/test.rs".to_string(), size : 50 },
  ];

  let tree = TreeBuilder::from_items( &items, | item | {
    item.path.split( '/' ).map( | s | s.to_string() ).collect()
  }, | item | item.clone() );

  assert_eq!( tree.children.len(), 2 );
}

/// Test build from empty items
#[ test ]
fn test_tree_builder_empty()
{
  use tree_fmt::TreeBuilder;

  let items : Vec< String > = vec![];
  let tree = TreeBuilder::from_items( &items, | _item | vec![], | item | item.clone() );

  assert_eq!( tree.children.len(), 0 );
}

/// Test build with deep nesting (5 levels)
#[ test ]
fn test_tree_builder_deep_nesting()
{
  use tree_fmt::TreeBuilder;

  let tree = TreeBuilder::new( "root" )
    .insert( &[ "a", "b", "c", "d", "e.txt" ], 42 )
    .build();

  let mut current = &tree;
  for _ in 0..5
  {
    assert_eq!( current.children.len(), 1 );
    current = &current.children[ 0 ];
  }
  assert_eq!( current.data, Some( 42 ) );
}

/// Test build with wide structure (10 directories)
#[ test ]
fn test_tree_builder_wide()
{
  use tree_fmt::TreeBuilder;

  let mut builder = TreeBuilder::new( "root" );
  for i in 0..10
  {
    builder = builder.insert( &[ &format!( "dir{}", i ), "file.txt" ], i );
  }
  let tree = builder.build();

  assert_eq!( tree.children.len(), 10 );
}

/// Test build preserves insertion order
#[ test ]
fn test_tree_builder_insertion_order()
{
  use tree_fmt::TreeBuilder;

  let tree = TreeBuilder::new( "root" )
    .insert( &[ "z.txt" ], 1 )
    .insert( &[ "a.txt" ], 2 )
    .insert( &[ "m.txt" ], 3 )
    .build();

  assert_eq!( tree.children[ 0 ].name, "z.txt" );
  assert_eq!( tree.children[ 1 ].name, "a.txt" );
  assert_eq!( tree.children[ 2 ].name, "m.txt" );
}

/// Test build with unicode paths
#[ test ]
fn test_tree_builder_unicode()
{
  use tree_fmt::TreeBuilder;

  let tree = TreeBuilder::new( "root" )
    .insert( &[ "文档", "测试.txt" ], 42 )
    .insert( &[ "документы", "тест.txt" ], 100 )
    .build();

  assert_eq!( tree.children.len(), 2 );
  assert!( tree.children[ 0 ].name.contains( "文" ) || tree.children[ 0 ].name.contains( "док" ) );
}

/// Test build with empty path components (should skip)
#[ test ]
fn test_tree_builder_empty_components()
{
  use tree_fmt::TreeBuilder;

  let tree = TreeBuilder::new( "root" )
    .insert( &[ "", "dir", "", "file.txt" ], 42 )
    .build();

  // Should skip empty components
  assert_eq!( tree.children.len(), 1 );
  assert_eq!( tree.children[ 0 ].name, "dir" );
}

/// Test build from PathBuf items
#[ test ]
fn test_tree_builder_from_pathbufs()
{
  use tree_fmt::TreeBuilder;

  let paths = vec![
    PathBuf::from( "src/main.rs" ),
    PathBuf::from( "src/lib.rs" ),
    PathBuf::from( "tests/test.rs" ),
  ];

  let tree = TreeBuilder::from_items( &paths, | path | {
    path.components().map( | c | c.as_os_str().to_string_lossy().to_string() ).collect()
  }, | path | path.clone() );

  assert!( tree.children.len() > 0 );
}

/// Test builder chaining
#[ test ]
fn test_tree_builder_chaining()
{
  use tree_fmt::TreeBuilder;

  let tree = TreeBuilder::new( "root" )
    .insert( &[ "file1.txt" ], 10 )
    .insert( &[ "file2.txt" ], 20 )
    .build();

  assert_eq!( tree.children.len(), 2 );
}

