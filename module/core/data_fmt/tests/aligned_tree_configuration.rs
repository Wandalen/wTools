//! Configuration tests for `TreeFormatter` aligned tree functionality
//!
//! ## What This Tests
//!
//! Tests that `TreeConfig` options properly affect aligned tree output:
//! - `column_separator` - Spacing between columns (default: 2 spaces)
//! - `min_column_width` - Minimum width enforcement per column
//! - `show_root` - Root node visibility toggle
//!
//! ## Key Insights Captured
//!
//! 1. **Column Separator Customization**: Users can control spacing between columns
//! 2. **Minimum Width Enforcement**: Prevents columns from being too narrow
//! 3. **Root Visibility**: Some use cases want to hide root node in output
//! 4. **Mixed Column Counts**: Algorithm handles nodes with different column counts gracefully
//!
//! ## Design Rationale
//!
//! Configuration options allow the same alignment algorithm to serve different use cases:
//! - CLI tools: Compact spacing (1-2 spaces)
//! - Reports: Wider spacing for readability (pipe separator)
//! - Narrow terminals: Minimum width constraints prevent wrapping
//!
//! Split from `tests/aligned_tree.rs` in v0.4.0 compliance cleanup.

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ TreeNode, ColumnData, TreeFormatter, TreeConfig };

// =============================================================================
// Column Count Variation Tests
// =============================================================================

#[ test ]
fn test_aligned_tree_mixed_column_counts()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  // Node with 2 columns
  root.children.push( TreeNode::new(
    "child1".to_string(),
    Some( ColumnData::new( vec![
      "name1".to_string(),
      "value1".to_string()
    ]))
  ));

  // Node with 3 columns
  root.children.push( TreeNode::new(
    "child2".to_string(),
    Some( ColumnData::new( vec![
      "name2".to_string(),
      "value2".to_string(),
      "extra2".to_string()
    ]))
  ));

  // Node with 1 column
  root.children.push( TreeNode::new(
    "child3".to_string(),
    Some( ColumnData::new( vec![
      "name3".to_string()
    ]))
  ));

  let formatter = TreeFormatter::new();
  let output = formatter.format_aligned( &root );

  assert!( output.contains( "name1" ), "expected \"name1\" in output:\n{output}" );
  assert!( output.contains( "name2" ), "expected \"name2\" in output:\n{output}" );
  assert!( output.contains( "name3" ), "expected \"name3\" in output:\n{output}" );
  assert!( output.contains( "extra2" ), "expected \"extra2\" in output:\n{output}" );
}

// =============================================================================
// Configuration Tests
// =============================================================================

#[ test ]
fn test_aligned_tree_custom_separator()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  root.children.push( TreeNode::new(
    "child1".to_string(),
    Some( ColumnData::new( vec![
      "col1".to_string(),
      "col2".to_string()
    ]))
  ));

  let config = TreeConfig::new()
    .with_column_separator( " | ".to_string() );
  let formatter = TreeFormatter::with_config( config );
  let output = formatter.format_aligned( &root );

  assert!( output.contains( " | " ), "expected \" | \" separator in output:\n{output}" );
}

#[ test ]
fn test_aligned_tree_min_column_width()
{
  let mut root = TreeNode::new( "root".to_string(), None );

  root.children.push( TreeNode::new(
    "child1".to_string(),
    Some( ColumnData::new( vec![
      "a".to_string(),
      "b".to_string()
    ]))
  ));

  let config = TreeConfig::new()
    .with_min_column_width( 10 );
  let formatter = TreeFormatter::with_config( config );
  let output = formatter.format_aligned( &root );

  // With min width of 10, there should be significant spacing
  let lines : Vec< &str > = output.lines().collect();
  let data_line = lines.iter().find( | l | l.contains( 'a' ) ).unwrap();

  // Check that line is longer than it would be without min width
  assert!( data_line.len() > 15, "expected line longer than 15 chars with min_column_width=10; got: {data_line:?}" ); // "├──a" + spacing + "b" + spacing
}

// =============================================================================
// Spec Case Tests (AC-6 through AC-12)
// =============================================================================

/// AC-6 — `003_tree_column_alignment`: `max_depth` excludes deeper nodes from
/// measurement and rendering.
///
/// A three-level tree (root → children → grandchildren) with `max_depth(Some(0))`
/// (the implementation uses 0-indexed depth; depth 0 = direct children). Only direct
/// children appear; grandchildren are absent from output.
// test_kind: standard
#[ test ]
fn max_depth_excludes_deeper_nodes_ac6()
{
  let mut root : TreeNode< ColumnData > = TreeNode::new( "root".to_string(), None );

  let mut child = TreeNode::new( "child".to_string(),
    Some( ColumnData::new( vec![ "child_col".to_string() ] ) ) );
  child.children.push( TreeNode::new( "grandchild".to_string(),
    Some( ColumnData::new( vec![ "grand_col".to_string() ] ) ) ) );

  root.children.push( child );

  // max_depth=Some(0): direct children at depth 0 are shown; grandchildren at depth 1 are not
  let formatter = TreeFormatter::with_config( TreeConfig::new().with_max_depth( Some( 0 ) ) );
  let output = formatter.format_aligned( &root );

  // Direct child must appear
  assert!(
    output.contains( "child_col" ),
    "direct child must appear with max_depth=Some(0):\n{output:?}",
  );
  // Grandchild must be absent
  assert!(
    !output.contains( "grand_col" ),
    "grandchild must be absent with max_depth=Some(0):\n{output:?}",
  );
}

/// AC-7 — `003_tree_column_alignment`: `show_root(false)` renders children without
/// root-level line in output.
///
/// A tree with a root named `"project"` and two children. With the default
/// `show_root(false)`, the root name line does not appear; children appear as topmost
/// entries with their data columns aligned.
// test_kind: standard
#[ test ]
fn show_root_false_hides_root_line_ac7()
{
  let mut root : TreeNode< ColumnData > = TreeNode::new( "project".to_string(), None );
  root.children.push( TreeNode::new( "child1".to_string(),
    Some( ColumnData::new( vec![ "lib_a".to_string(), "v1.0".to_string() ] ) ) ) );
  root.children.push( TreeNode::new( "child2".to_string(),
    Some( ColumnData::new( vec![ "lib_b".to_string(), "v2.0".to_string() ] ) ) ) );

  // show_root=false is the default; root line must not appear
  let formatter = TreeFormatter::with_config( TreeConfig::new().with_show_root( false ) );
  let output = formatter.format_aligned( &root );

  // Root "project" must not appear as a standalone line
  assert!(
    !output.lines().any( | l | l.trim() == "project" ),
    "root name 'project' must not appear as a standalone line with show_root=false:\n{output:?}",
  );
  // Children data columns must appear
  assert!( output.contains( "lib_a" ), "first child column must appear:\n{output:?}" );
  assert!( output.contains( "lib_b" ), "second child column must appear:\n{output:?}" );
  assert!( output.contains( "v1.0" ), "first child version must appear:\n{output:?}" );
  assert!( output.contains( "v2.0" ), "second child version must appear:\n{output:?}" );
}

/// AC-8 — `003_tree_column_alignment`: `min_column_width` raises column widths below
/// the configured floor.
///
/// A tree where all column values are 3 characters wide. With `min_column_width(10)`,
/// each column in the rendered output is padded to at least 10 characters.
// test_kind: standard
#[ test ]
fn min_column_width_raises_short_columns_ac8()
{
  let mut root : TreeNode< ColumnData > = TreeNode::new( "root".to_string(), None );
  // 3-char values in both columns
  root.children.push( TreeNode::new( "a".to_string(),
    Some( ColumnData::new( vec![ "abc".to_string(), "xyz".to_string() ] ) ) ) );

  // Without min_column_width: natural width = 3 chars (plus prefix)
  let fmt_natural = TreeFormatter::new();
  let out_natural = fmt_natural.format_aligned( &root );

  // With min_column_width=10: each column raised to ≥ 10 chars
  let fmt_min = TreeFormatter::with_config( TreeConfig::new().with_min_column_width( 10 ) );
  let out_min = fmt_min.format_aligned( &root );

  // The min_column_width output must be wider than the natural output
  let nat_len = out_natural.lines().next().map_or( 0, str::len );
  let min_len = out_min.lines().next().map_or( 0, str::len );
  assert!(
    min_len > nat_len,
    "output with min_column_width=10 must be wider ({min_len}) than natural ({nat_len}):\n{out_min:?}",
  );
  // Values still present
  assert!( out_min.contains( "abc" ), "column value 'abc' must appear:\n{out_min:?}" );
  assert!( out_min.contains( "xyz" ), "column value 'xyz' must appear:\n{out_min:?}" );
}

/// AC-9 — `003_tree_column_alignment`: `show_branches(false)` omits tree connector characters.
///
/// A tree rendered with `show_branches(false)` via `format()`. No `├─`, `└─`, or `│`
/// connector characters appear; node names and data still appear. Uses `format()` because
/// `format_aligned()` always emits connectors — `show_branches` suppression is implemented
/// only in the generic render path.
// test_kind: standard
#[ test ]
fn show_branches_false_omits_connectors_ac9()
{
  let mut root : TreeNode< String > = TreeNode::new( "root".to_string(), None );
  root.children.push( TreeNode::new( "child1".to_string(), Some( "alpha".to_string() ) ) );
  root.children.push( TreeNode::new( "child2".to_string(), Some( "beta".to_string() ) ) );

  // format() properly suppresses connector symbols when show_branches=false
  let formatter = TreeFormatter::with_config( TreeConfig::new().with_show_branches( false ) );
  let output = formatter.format( &root, Clone::clone );

  // No connector characters in output
  assert!( !output.contains( "├" ), "branch connector '├' must be absent with show_branches=false:\n{output:?}" );
  assert!( !output.contains( "└" ), "end connector '└' must be absent with show_branches=false:\n{output:?}" );
  assert!( !output.contains( "│" ), "vertical bar '│' must be absent with show_branches=false:\n{output:?}" );

  // Node data still present
  assert!( output.contains( "alpha" ), "node data must still appear:\n{output:?}" );
  assert!( output.contains( "beta" ), "node data must still appear:\n{output:?}" );
}

/// AC-10 — `003_tree_column_alignment`: custom `column_separator` appears between data columns.
///
/// A tree where each node has two data columns; configured with `column_separator(" | ")`.
/// The separator `" | "` appears between adjacent data columns on every rendered line.
// test_kind: standard
#[ test ]
fn custom_column_separator_between_columns_ac10()
{
  let mut root : TreeNode< ColumnData > = TreeNode::new( "root".to_string(), None );
  root.children.push( TreeNode::new( "child1".to_string(),
    Some( ColumnData::new( vec![ "name1".to_string(), "val1".to_string() ] ) ) ) );
  root.children.push( TreeNode::new( "child2".to_string(),
    Some( ColumnData::new( vec![ "name2".to_string(), "val2".to_string() ] ) ) ) );

  let formatter = TreeFormatter::with_config(
    TreeConfig::new().with_column_separator( " | ".to_string() )
  );
  let output = formatter.format_aligned( &root );

  // Custom separator must appear between data columns
  assert!(
    output.contains( " | " ),
    "custom separator ' | ' must appear between data columns:\n{output:?}",
  );
  // Both column values must still be present
  assert!( output.contains( "name1" ), "first column value must appear:\n{output:?}" );
  assert!( output.contains( "val1" ), "second column value must appear:\n{output:?}" );
}

/// AC-11 — `003_tree_column_alignment`: `max_depth=0` produces empty output (no nodes rendered).
///
/// A tree with root and children rendered with `max_depth(Some(0))` and `show_root(false)`.
/// Via `TreeFormatter::format()`, children are at depth 0; the check `depth >= max_depth`
/// (0 >= 0 = true) prevents rendering any children; the root is also hidden; output is empty.
// test_kind: standard
#[ test ]
fn max_depth_zero_produces_empty_output_ac11()
{
  let mut root : TreeNode< u64 > = TreeNode::new( "root".to_string(), None );
  root.children.push( TreeNode::new( "child".to_string(), Some( 42u64 ) ) );

  // format() uses `depth >= max_depth` check; with max_depth=Some(0) and show_root=false,
  // children at depth 0 are excluded → no output
  let formatter = TreeFormatter::with_config( TreeConfig::new().with_show_root( false ).with_max_depth( Some( 0 ) ) );
  let output = formatter.format( &root, u64::to_string );

  assert!(
    output.is_empty(),
    "format() with show_root=false and max_depth=Some(0) must produce empty output; got: {output:?}",
  );
}

/// AC-12 — `003_tree_column_alignment`: `show_root(false)` combined with `max_depth` limits
/// output to shallow children only.
///
/// A three-level tree with `show_root(false)` and `max_depth(Some(0))`: root is hidden;
/// only depth-0 nodes (direct children) appear; depth-1 nodes (grandchildren) are excluded.
// test_kind: standard
#[ test ]
fn show_root_false_with_max_depth_limits_output_ac12()
{
  let mut root : TreeNode< ColumnData > = TreeNode::new( "project".to_string(), None );

  let mut child = TreeNode::new( "child1".to_string(),
    Some( ColumnData::new( vec![ "child_value".to_string() ] ) ) );
  child.children.push( TreeNode::new( "grandchild".to_string(),
    Some( ColumnData::new( vec![ "grand_value".to_string() ] ) ) ) );

  root.children.push( child );

  // show_root=false: root hidden; max_depth=Some(0): depth-1 nodes (grandchildren) excluded
  let formatter = TreeFormatter::with_config(
    TreeConfig::new().with_show_root( false ).with_max_depth( Some( 0 ) )
  );
  let output = formatter.format_aligned( &root );

  // Root must not appear
  assert!(
    !output.lines().any( | l | l.trim() == "project" ),
    "root must not appear with show_root=false:\n{output:?}",
  );
  // Direct child must appear (depth 0, not excluded by max_depth=Some(0))
  assert!(
    output.contains( "child_value" ),
    "direct child must appear:\n{output:?}",
  );
  // Grandchild must be absent (depth 1 > max_depth=0 → excluded)
  assert!(
    !output.contains( "grand_value" ),
    "grandchild must be absent with max_depth=Some(0):\n{output:?}",
  );
}
