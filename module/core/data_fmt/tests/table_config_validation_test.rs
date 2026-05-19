//! `min_column_width` floor enforcement and table alignment correctness tests
//!
//! ## What This Tests
//!
//! ### 9. `min_column_width` Floor Enforcement (Task 012)
//! - T012-P01: short content raised to floor
//! - T012-P02: min and max both honored together
//! - T012-P03: min=0 (default) is a no-op (regression guard)
//! - T012-P04: content at exact floor width — no over-expansion (regression guard)
//! - T012-N01: min wins over max for short content (floor applied after max cap)
//! - T012-N02: content > floor — content width wins (regression guard)
//! - T012-N03: `column_widths` override bypasses `min_column_width`
//! - T012-N04: empty cell content — floor applied when content shorter than floor
//! - T012-N05: very large min value — no panic
//!
//! ### 10. Alignment: all lines in a table must have equal char-widths
//! - `unicode_box()` all lines same display width (regression for issue-align)
//! - `markdown()` all lines same display width
//! - `bordered()` (`AsciiGrid`) all lines same display width
//! - `grid()` (`AsciiGrid` with borders) all lines same display width

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

// ============================================================================
// 9. min_column_width Floor Enforcement (Task 012)
// ============================================================================

/// T012-P01: `min_column_width(10)` on short content → all columns padded to ≥ 10.
/// `plain()` `inner_padding=0` makes column width equal to rendered line length for single
/// column tables, so `line.len() >= 10` directly verifies the floor is applied.
#[ test ]
fn test_min_column_width_raises_short_content_to_floor()
{
  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "abc".into() ] )
    .build_view();

  let output = TableFormatter::with_config(
    TableConfig::plain().min_column_width( 10 )
  ).format( &tree ).unwrap_or_default();

  let data_line = output.lines().find( | l | l.contains( "abc" ) )
    .expect( "must have data row with 'abc'" );

  assert!(
    data_line.len() >= 10,
    "min_column_width(10) must pad 3-char content to ≥ 10; line={data_line:?}; output:\n{output}"
  );
}

/// T012-P02: `min_column_width(5)` + `max_column_width(Some(20))` — both limits honored.
/// Content "ab" is 2 chars: floor raises to 5, cap at 20 leaves it at 5.
#[ test ]
fn test_min_column_width_with_max_column_width_both_honored()
{
  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "ab".into() ] )
    .build_view();

  let output = TableFormatter::with_config(
    TableConfig::plain()
      .min_column_width( 5 )
      .max_column_width( Some( 20 ) )
  ).format( &tree ).unwrap_or_default();

  let data_line = output.lines().find( | l | l.contains( "ab" ) )
    .expect( "must have data row with 'ab'" );

  // Floor raises 2-char content to 5; max_column_width(20) doesn't interfere
  assert!(
    data_line.len() >= 5,
    "min=5 + max=20 must raise 2-char content to ≥ 5; line={data_line:?}; output:\n{output}"
  );
  // Cap at 20 doesn't over-expand
  assert!(
    data_line.len() < 20,
    "min=5 + max=20 on 2-char content must not over-expand to 20; line={data_line:?}; output:\n{output}"
  );
}

/// T012-P03: `min_column_width(0)` (default) — no change in output.
/// Regression guard: default behavior must be preserved; floor of 0 is a no-op.
#[ test ]
fn test_min_column_width_zero_is_no_op()
{
  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "xyz".into() ] )
    .build_view();

  let output_default = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();
  let output_zero    = TableFormatter::with_config(
    TableConfig::plain().min_column_width( 0 )
  ).format( &tree ).unwrap_or_default();

  assert_eq!(
    output_default, output_zero,
    "min_column_width(0) must produce same output as default config"
  );
}

/// T012-P04: `min_column_width(8)` with content exactly 8 chars — no over-expansion.
/// The floor should not add extra width when content already matches the floor exactly.
#[ test ]
fn test_min_column_width_at_exact_match_no_over_expansion()
{
  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "12345678".into() ] )   // exactly 8 chars
    .build_view();

  let output_with_floor = TableFormatter::with_config(
    TableConfig::plain().min_column_width( 8 )
  ).format( &tree ).unwrap_or_default();

  let output_no_floor = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();

  // Content is already 8 chars; floor of 8 changes nothing
  assert_eq!(
    output_with_floor, output_no_floor,
    "min_column_width(8) with 8-char content must produce same output as no floor"
  );
}

/// T012-N01: `min_column_width(5)` + `max_column_width(Some(3))` — min wins over max.
/// Content "a" (1 char) → max caps to 1 (already ≤ 3), floor raises to 5.
/// Floor is applied after max cap, so min always wins over max for short content.
#[ test ]
fn test_min_column_width_wins_over_max_column_width_for_short_content()
{
  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "a".into() ] )
    .build_view();

  let output = TableFormatter::with_config(
    TableConfig::plain()
      .min_column_width( 5 )
      .max_column_width( Some( 3 ) )
  ).format( &tree ).unwrap_or_default();

  let data_line = output.lines().find( | l | l.contains( 'a' ) )
    .expect( "must have data row" );

  assert!(
    data_line.len() >= 5,
    "min=5 must win over max=3 for 1-char content; line={data_line:?}; output:\n{output}"
  );
}

/// T012-N02: `min_column_width(10)` with content 15 chars — content width wins.
/// Content is wider than the floor, so the column stays at content width (15).
/// Regression guard: floor must not shrink content-driven widths.
#[ test ]
fn test_min_column_width_does_not_shrink_wider_content()
{
  let content = "x".repeat( 15 );
  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ content.clone().into() ] )
    .build_view();

  let output = TableFormatter::with_config(
    TableConfig::plain().min_column_width( 10 )
  ).format( &tree ).unwrap_or_default();

  let data_line = output.lines().find( | l | l.contains( &content ) )
    .expect( "must have data row with 15-char content" );

  assert!(
    data_line.len() >= 15,
    "min=10 must not shrink 15-char content; line={data_line:?}; output:\n{output}"
  );
}

/// T012-N03: `column_widths([2])` override + `min_column_width(10)` — override wins.
/// When `column_widths` override is set, `min_column_width` is bypassed entirely.
/// This tests the documented behavioral contract: override = exact widths, no limits.
#[ test ]
fn test_column_widths_override_bypasses_min_column_width()
{
  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "a".into() ] )
    .build_view();

  // Override forces width=2; floor of 10 must NOT apply
  let output_override = TableFormatter::with_config(
    TableConfig::plain()
      .column_widths( vec![ 2 ] )
      .min_column_width( 10 )
  ).format( &tree ).unwrap_or_default();

  // No override (min=10 applies): column = 10
  let output_floor_only = TableFormatter::with_config(
    TableConfig::plain().min_column_width( 10 )
  ).format( &tree ).unwrap_or_default();

  // Override output should be shorter (width=2) than floor-only (width=10)
  let override_line = output_override.lines().find( | l | l.contains( 'a' ) )
    .expect( "must have data row" );
  let floor_line = output_floor_only.lines().find( | l | l.contains( 'a' ) )
    .expect( "must have data row" );

  assert!(
    override_line.len() < floor_line.len(),
    "column_widths override must bypass min_column_width; override_line={override_line:?} floor_line={floor_line:?}"
  );
}

/// T012-N04: empty cell content + `min_column_width(5)` — floor applied when content is shorter.
/// Column width from header "H" (1 char) and empty cell ""; floor raises width from 1 to 5.
#[ test ]
fn test_min_column_width_applied_when_content_is_empty()
{
  // Header "H" (1 char), data row with empty cell (0 chars) → column width = 1 without floor
  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "".into() ] )  // empty cell → column width = max(1, 0) = 1
    .build_view();

  let output = TableFormatter::with_config(
    TableConfig::plain().min_column_width( 5 )
  ).format( &tree ).unwrap_or_default();

  let header_line = output.lines().find( | l | l.contains( 'H' ) )
    .expect( "must have header row containing 'H'" );

  assert!(
    header_line.len() >= 5,
    "min_column_width(5) must raise header line to ≥ 5; line={header_line:?}; output:\n{output}"
  );
}

/// T012-N05: very large `min_column_width` — no panic during rendering.
/// Uses `10_000` rather than `usize::MAX` to avoid OOM; verifies no arithmetic overflow.
#[ test ]
fn test_min_column_width_large_value_no_panic()
{
  let tree = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "v".into() ] )
    .build_view();

  // Should not panic; rendering with very wide columns is allowed
  let output = TableFormatter::with_config(
    TableConfig::plain().min_column_width( 10_000 )
  ).format( &tree ).unwrap_or_default();

  assert!(
    output.contains( 'v' ),
    "min_column_width(10_000) must render data without panic; output length={}", output.len()
  );
}

// ============================================================================
// 10. Alignment: all lines in a table must have equal char-widths
// ============================================================================

/// All lines in a `unicode_box()` table must have the same display width.
///
/// ## Root Cause (Bug)
///
/// `format_header_separator()` Unicode branch used `width + 2` per column
/// (`"─".repeat(width + 2)`), adding 2 extra fill characters per column.
/// For N columns this makes the separator `2*(N-1)` chars wider than data rows.
///
/// Data rows only add `inner_padding` at the OUTER edges (before first column,
/// after last column) — middle columns have no padding around the `│` separator.
/// The separator must follow the same rule.
///
/// ## Why Not Caught
///
/// Previous tests only checked whether unicode chars were PRESENT, not whether
/// line lengths matched. No alignment test existed for multi-column unicode_box.
///
/// ## Fix Applied
///
/// Replace hardcoded `width + 2` loop in the Unicode separator branch with a
/// call to `format_unicode_horizontal_rule(output, widths, '├', '─', '┼', '┤')`,
/// which correctly handles inner_padding only at the outer edges.
///
/// ## Prevention
///
/// Any new horizontal rule helper must be validated: for N cols with widths W_i
/// and inner_padding P, expected total char-width =
/// 1 (left) + P + sum(W_i) + (N-1) (mid junctions) + P + 1 (right).
///
/// ## Pitfall
///
/// Do NOT add inner_padding around every column junction — only at the two outer
/// table edges. Middle `│` / `┼` / `+` chars are junction-only, not padded.
#[ cfg_attr( not( test ), allow( dead_code ) ) ]
#[ cfg_attr( test, test ) ]
#[ cfg_attr( test, cfg_attr( test, allow( clippy::all, warnings ) ) ) ]
fn test_unicode_box_all_lines_same_display_width()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .add_row( vec![ "Bob".into(), "25".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::unicode_box() ).format( &tree ).unwrap_or_default();
  let lines : Vec< &str > = output.lines().filter( | l | !l.is_empty() ).collect();

  // Expected structure: top_border + header + header_sep + data*2 + bottom = 6 lines
  assert!(
    lines.len() >= 5,
    "unicode_box with 2 data rows must produce at least 5 non-empty lines; got {}\n{output}",
    lines.len()
  );

  // All non-empty lines must have the same display-column count
  let widths : Vec< usize > = lines.iter().map( | l | l.chars().count() ).collect();
  let first_width = widths[ 0 ];
  for ( idx, ( &w, &line ) ) in widths.iter().zip( lines.iter() ).enumerate()
  {
    assert_eq!(
      w, first_width,
      "Line {idx} has width {w}, expected {first_width}\n  line[{idx}]: {:?}\n  line[0]:    {:?}\nFull output:\n{output}",
      line, lines[ 0 ]
    );
  }
}

/// All lines in a `markdown()` table must have the same display width.
///
/// ## Root Cause (Bug)
///
/// `format_header_separator()` Markdown branch used the same `width + 2`
/// per-column pattern as the Unicode branch — adding 2 extra `'-'` chars per
/// column, causing the same misalignment for N >= 2 columns.
///
/// ## Why Not Caught
///
/// Tests only checked structural characters (`|`), not line lengths.
///
/// ## Fix Applied
///
/// Replace hardcoded `width + 2` with correct outer-only padding logic, matching
/// how `format_single_line_row` pads data rows.
///
/// ## Prevention
///
/// Write alignment tests for EVERY table style that uses a header separator.
/// Run `lines.iter().map(|l| l.chars().count()).collect::<Vec<_>>()` and assert
/// all counts are equal.
///
/// ## Pitfall
///
/// Standard Markdown table spec requires at least one `-` per column but does not
/// mandate exact widths. However, visual alignment in rendered output requires
/// consistent line lengths.
#[ cfg_attr( not( test ), allow( dead_code ) ) ]
#[ cfg_attr( test, test ) ]
#[ cfg_attr( test, cfg_attr( test, allow( clippy::all, warnings ) ) ) ]
fn test_markdown_all_lines_same_display_width()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::markdown() ).format( &tree ).unwrap_or_default();
  let lines : Vec< &str > = output.lines().filter( | l | !l.is_empty() ).collect();

  assert!(
    lines.len() >= 3,
    "markdown with 1 data row must produce at least 3 non-empty lines; got {}\n{output}",
    lines.len()
  );

  let widths : Vec< usize > = lines.iter().map( | l | l.chars().count() ).collect();
  let first_width = widths[ 0 ];
  for ( idx, ( &w, &line ) ) in widths.iter().zip( lines.iter() ).enumerate()
  {
    assert_eq!(
      w, first_width,
      "Line {idx} has width {w}, expected {first_width}\n  line[{idx}]: {:?}\n  line[0]:    {:?}\nFull output:\n{output}",
      line, lines[ 0 ]
    );
  }
}

/// All lines in a `bordered()` table must have the same display width.
///
/// Regression guard: `AsciiGrid` separator alignment must remain correct.
#[ test ]
fn test_bordered_all_lines_same_display_width()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::bordered() ).format( &tree ).unwrap_or_default();
  let lines : Vec< &str > = output.lines().filter( | l | !l.is_empty() ).collect();

  assert!( lines.len() >= 3 );

  let widths : Vec< usize > = lines.iter().map( | l | l.chars().count() ).collect();
  let first_width = widths[ 0 ];
  for ( idx, ( &w, &line ) ) in widths.iter().zip( lines.iter() ).enumerate()
  {
    assert_eq!(
      w, first_width,
      "bordered line {idx} has width {w}, expected {first_width}\n  line: {line:?}\nFull output:\n{output}"
    );
  }
}

/// All lines in a `grid()` table must have the same display width.
///
/// Regression guard: `AsciiGrid` border + separator alignment must remain correct.
#[ test ]
fn test_grid_all_lines_same_display_width()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .add_row( vec![ "Bob".into(), "25".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::grid() ).format( &tree ).unwrap_or_default();
  let lines : Vec< &str > = output.lines().filter( | l | !l.is_empty() ).collect();

  assert!( lines.len() >= 5 );

  let widths : Vec< usize > = lines.iter().map( | l | l.chars().count() ).collect();
  let first_width = widths[ 0 ];
  for ( idx, ( &w, &line ) ) in widths.iter().zip( lines.iter() ).enumerate()
  {
    assert_eq!(
      w, first_width,
      "grid line {idx} has width {w}, expected {first_width}\n  line: {line:?}\nFull output:\n{output}"
    );
  }
}

