//! `TreeFormatter`: Hierarchical outline display (box-drawing tree)
//!
//! Use when: Nested structures, file systems, org charts, dependency trees, parent-child data
//! Output: Tree structure with ├── └── │ box-drawing characters
//!
//! NOTE: For table-shaped data, `TreeFormatter` also implements the `Format` trait.
//! See `unified_formats.rs` example for format-agnostic code patterns.

use tree_fmt::{ TreeNode, TreeFormatter };

fn main()
{
  println!( "=== TreeFormatter: Hierarchical Tree Display ===" );
  println!( "Best for: Nested structures, file systems, organizational charts\n" );

  // Create hierarchical data: Project directory structure
  let mut root = TreeNode::new( "project/".to_string(), None::<u32> );

  // src/ branch
  let mut src = TreeNode::new( "src/".to_string(), None );
  src.children.push( TreeNode::new( "main.rs".to_string(), Some( 150 ) ) );
  src.children.push( TreeNode::new( "lib.rs".to_string(), Some( 320 ) ) );
  src.children.push( TreeNode::new( "utils.rs".to_string(), Some( 85 ) ) );

  // tests/ branch
  let mut tests = TreeNode::new( "tests/".to_string(), None );
  tests.children.push( TreeNode::new( "integration.rs".to_string(), Some( 240 ) ) );
  tests.children.push( TreeNode::new( "unit.rs".to_string(), Some( 180 ) ) );

  // examples/ branch
  let mut examples = TreeNode::new( "examples/".to_string(), None );
  examples.children.push( TreeNode::new( "basic.rs".to_string(), Some( 45 ) ) );

  root.children.push( src );
  root.children.push( tests );
  root.children.push( examples );
  root.children.push( TreeNode::new( "Cargo.toml".to_string(), Some( 15 ) ) );
  root.children.push( TreeNode::new( "readme.md".to_string(), Some( 28 ) ) );

  // Render as tree
  let formatter = TreeFormatter::new();
  let output = formatter.format( &root, | lines | format!( "{lines} lines" ) );

  println!( "{output}" );

  println!( "\n✓ Characteristics:" );
  println!( "  • Hierarchical visualization with parent-child relationships" );
  println!( "  • Box-drawing characters (├── └── │) show structure" );
  println!( "  • Indentation indicates nesting level" );
  println!( "  • Natural for file systems and tree data" );
}
