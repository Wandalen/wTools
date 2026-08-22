//! Tests for `TreeConfig::with_heading()` / `TreeConfig::with_footer()` — the same
//! `Heading` titled-rule pattern as `table_heading_test.rs` / `table_footer_test.rs`,
//! applied to `TreeFormatter::format()`. Unlike Table, tree output has no fixed
//! column width, so the rule fills to the widest rendered line's display width
//! instead of a precomputed `table_width`. FT-19..FT-24 continue the ID sequence
//! from `tests/docs/feature/007_table_heading.md`.

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ TreeNode, TreeFormatter, TreeConfig, Heading };

fn simple_tree() -> TreeNode< String >
{
  let mut root = TreeNode::new( "root".to_string(), None );
  root.children.push( TreeNode::new( "alpha".to_string(), Some( "1".to_string() ) ) );
  root.children.push( TreeNode::new( "beta".to_string(), Some( "2".to_string() ) ) );
  root
}

fn display_width( s : &str ) -> usize
{
  s.chars().count()
}

/// FT-19 — `feature/007`: title-only heading renders titled rule before tree output.
// test_kind: standard
#[ test ]
fn title_only_heading_renders_before_tree_ft19()
{
  let config = TreeConfig::new().with_heading( Heading::new( "Project" ) );
  let output = TreeFormatter::with_config( config ).format( &simple_tree(), String::clone );

  let lines : Vec< &str > = output.lines().collect();
  let first_line = *lines.first().unwrap_or( &"" );
  assert!(
    first_line.starts_with( "─── Project" ),
    "first line must be the heading; got: '{first_line}'",
  );
  assert!( output.contains( "alpha" ), "tree body must still render; got:\n{output}" );
}

/// FT-20 — `feature/007`: title-only footer renders titled rule after tree output.
// test_kind: standard
#[ test ]
fn title_only_footer_renders_after_tree_ft20()
{
  let config = TreeConfig::new().with_footer( Heading::new( "2 items" ) );
  let output = TreeFormatter::with_config( config ).format( &simple_tree(), String::clone );

  let lines : Vec< &str > = output.lines().collect();
  let last_line = *lines.last().unwrap_or( &"" );
  assert!(
    last_line.starts_with( "─── 2 items" ),
    "last line must be the footer; got: '{last_line}'",
  );
  assert!( output.contains( "beta" ), "tree body must still render; got:\n{output}" );
}

/// FT-21 — `feature/007`: heading/footer fill to the widest rendered tree line, not a
/// fixed table-style width — tree lines are ragged (vary by depth), so the rule must
/// track the actual body content rather than any config-declared width.
// test_kind: standard
#[ test ]
fn heading_fills_to_widest_tree_line_ft21()
{
  let config = TreeConfig::new().with_heading( Heading::new( "H" ) );
  let output = TreeFormatter::with_config( config ).format( &simple_tree(), String::clone );

  let lines : Vec< &str > = output.lines().collect();
  let body_lines = &lines[ 1.. ];
  let expected_width = body_lines.iter().map( | l | display_width( l ) ).max().unwrap_or( 0 );
  let heading_width = display_width( lines[ 0 ] );

  assert_eq!(
    heading_width, expected_width,
    "heading must fill to the widest body line ({expected_width}), got {heading_width}",
  );
}

/// FT-22 — `feature/007`: no heading/footer configured produces byte-identical output
/// to a `TreeConfig` built without them (passthrough — Invariant 1 equivalent for Tree).
// test_kind: standard
#[ test ]
fn no_heading_no_footer_output_unchanged_ft22()
{
  let plain = TreeFormatter::with_config( TreeConfig::new() ).format( &simple_tree(), String::clone );
  let baseline = TreeFormatter::new().format( &simple_tree(), String::clone );

  assert_eq!( plain, baseline, "TreeConfig::new() with no heading/footer must match TreeFormatter::new() baseline" );
  assert!( !plain.starts_with( '─' ), "no heading configured — first line must not be a rule line" );
}

/// FT-23 — `feature/007`: heading and footer coexist on tree output without interfering.
// test_kind: standard
#[ test ]
fn heading_and_footer_coexist_on_tree_ft23()
{
  let config = TreeConfig::new()
    .with_heading( Heading::new( "Top" ) )
    .with_footer( Heading::new( "End" ) );
  let output = TreeFormatter::with_config( config ).format( &simple_tree(), String::clone );

  let lines : Vec< &str > = output.lines().collect();
  let first_line = *lines.first().unwrap_or( &"" );
  let last_line = *lines.last().unwrap_or( &"" );

  assert!( first_line.starts_with( "─── Top" ), "first line must be heading; got: '{first_line}'" );
  assert!( last_line.starts_with( "─── End" ), "last line must be footer; got: '{last_line}'" );
  assert_eq!(
    display_width( first_line ), display_width( last_line ),
    "heading and footer must fill to the same width",
  );
  assert!( output.contains( "alpha" ) && output.contains( "beta" ), "tree body unaffected; got:\n{output}" );
}

/// FT-24 — `feature/007`: heading applies even on the leaf-only-root early-return path
/// (root has data and no children) — proves the wrap applies uniformly across all three
/// of `TreeFormatter::format()`'s internal branches, not only the "standard" multi-child path.
// test_kind: standard
#[ test ]
fn heading_applies_to_leaf_only_root_ft24()
{
  let leaf_root : TreeNode< String > = TreeNode::new( "solo".to_string(), Some( "data".to_string() ) );
  let config = TreeConfig::new().with_heading( Heading::new( "Leaf" ) );
  let output = TreeFormatter::with_config( config ).format( &leaf_root, String::clone );

  let lines : Vec< &str > = output.lines().collect();
  assert_eq!( lines.len(), 2, "expected exactly heading line + leaf line; got:\n{output}" );
  assert!( lines[ 0 ].starts_with( "─── Leaf" ), "first line must be heading; got: '{}'", lines[ 0 ] );
  assert!( lines[ 1 ].contains( "solo" ) && lines[ 1 ].contains( "data" ), "second line must be the leaf; got: '{}'", lines[ 1 ] );
}
