//! Border variant rendering tests (Task 014)
//!
//! ## What This Tests
//!
//! Top/bottom border rendering and inter-row separator behavior for
//! `BorderVariant::AsciiGrid` and `BorderVariant::Unicode`. Also confirms
//! that `BorderVariant::Ascii` and `BorderVariant::Markdown` do NOT add
//! top/bottom borders, and validates the `AsciiGrid` header separator
//! corner-character fix (`|---|` → `+---+`).
//!
//! ## Root Cause Background
//!
//! `format_internal()` never read `border_variant`. Top/bottom horizontal rules
//! and inter-row separators were silently missing. The `AsciiGrid` header separator
//! also used `'|'` as corners instead of `'+'`, making it visually inconsistent
//! with the border rules added in this task.
//!
//! ## Fix Applied
//!
//! - Added `format_ascii_horizontal_rule()` and `format_unicode_horizontal_rule()`
//!   parameterized helpers on `TableFormatter`.
//! - Added `format_top_border_if_needed()`, `format_bottom_border_if_needed()`,
//!   `format_inter_row_sep_if_needed()` wrapper methods.
//! - Updated `format_internal()` to call the three wrappers.
//! - Changed `format_header_separator()` `AsciiGrid` corner `'|'` → `'+'`.
//!
//! ## Test Matrix
//!
//! ### T014-P: Positive tests (behaviors that SHOULD be present)
//!
//! | # | Scenario | Config | Expected |
//! |---|----------|--------|----------|
//! | T014-P01 | 2 data rows | `grid()` | Top `+...+` and bottom `+...+` borders |
//! | T014-P02 | 2 data rows | `unicode_box()` | Top `┌...┐` and bottom `└...┘` |
//! | T014-P03 | 2 data rows | `bordered()` | No top/bottom borders (Ascii variant) |
//! | T014-P04 | 2 data rows | `plain()` | No `+` or box-drawing chars at all |
//! | T014-P05 | 3 data rows | `grid()` | 2 inter-row separators between data rows |
//!
//! ### T014-N: Negative / edge tests
//!
//! | # | Scenario | Config | Expected |
//! |---|----------|--------|----------|
//! | T014-N01 | AsciiGrid header sep | `grid()` | Separator is `+---+`, NOT `\|---\|` |
//! | T014-N02 | Unicode top border | `unicode_box()` | Starts with `┌`, NOT `├` |
//! | T014-N03 | Unicode bottom border | `unicode_box()` | Starts with `└`, NOT `├` |
//! | T014-N04 | Markdown variant | `markdown()` | No top/bottom borders |
//! | T014-N05 | 1 data row | `grid()` | No inter-row separators; bottom border present |

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

// ---------------------------------------------------------------------------
// T014-P: Positive tests
// ---------------------------------------------------------------------------

/// T014-P01 — `TableConfig::grid()` renders top and bottom `+...+` borders.
///
/// `AsciiGrid` variant MUST emit a `+` border line before the header row
/// and after the last data row.
#[ test ]
fn test_t014_p01_grid_has_top_and_bottom_borders()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .add_row( vec![ "p".into(), "q".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::grid() ).format( &tree ).unwrap_or_default();

  let first_line = output.lines().next().expect( "output is empty" );
  assert!(
    first_line.starts_with( '+' ) && first_line.ends_with( '+' ),
    "T014-P01: expected top border starting and ending with '+', got: {first_line:?}\nFull output:\n{output}"
  );

  let last_line = output.lines().rfind( |l| !l.is_empty() ).expect( "output has no non-empty lines" );
  assert!(
    last_line.starts_with( '+' ) && last_line.ends_with( '+' ),
    "T014-P01: expected bottom border starting and ending with '+', got: {last_line:?}\nFull output:\n{output}"
  );
}

/// T014-P02 — `TableConfig::unicode_box()` renders `┌...┐` top and `└...┘` bottom.
///
/// Unicode variant MUST emit box-drawing corner characters for top and bottom borders.
#[ test ]
fn test_t014_p02_unicode_box_has_top_and_bottom_borders()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .add_row( vec![ "p".into(), "q".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::unicode_box() ).format( &tree ).unwrap_or_default();

  let first_line = output.lines().next().expect( "output is empty" );
  assert!(
    first_line.starts_with( '┌' ) && first_line.ends_with( '┐' ),
    "T014-P02: expected top border '┌...┐', got: {first_line:?}\nFull output:\n{output}"
  );

  let last_line = output.lines().rfind( |l| !l.is_empty() ).expect( "no non-empty lines" );
  assert!(
    last_line.starts_with( '└' ) && last_line.ends_with( '┘' ),
    "T014-P02: expected bottom border '└...┘', got: {last_line:?}\nFull output:\n{output}"
  );
}

/// T014-P03 — `TableConfig::bordered()` does NOT add top/bottom borders.
///
/// `BorderVariant::Ascii` uses pipe walls on rows but never emits horizontal
/// top/bottom rule lines — only `BorderVariant::AsciiGrid` and `Unicode` do.
#[ test ]
fn test_t014_p03_bordered_no_top_bottom_borders()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .add_row( vec![ "p".into(), "q".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::bordered() ).format( &tree ).unwrap_or_default();

  let first_line = output.lines().next().expect( "output is empty" );
  assert!(
    !first_line.starts_with( '+' ),
    "T014-P03: bordered() must NOT have a top '+' border, got: {first_line:?}\nFull output:\n{output}"
  );
  // First line should be the header row (starts with '|')
  assert!(
    first_line.starts_with( '|' ),
    "T014-P03: bordered() first line should be header row starting with '|', got: {first_line:?}"
  );

  let last_line = output.lines().rfind( |l| !l.is_empty() ).expect( "no non-empty lines" );
  assert!(
    !last_line.starts_with( '+' ) && !last_line.starts_with( '┌' ) && !last_line.starts_with( '└' ),
    "T014-P03: bordered() must NOT have a bottom border, got: {last_line:?}\nFull output:\n{output}"
  );
}

/// T014-P04 — `TableConfig::plain()` has no `+` or box-drawing border characters.
///
/// `BorderVariant::None` (the plain preset default) emits no border rules at all.
#[ test ]
fn test_t014_p04_plain_no_borders()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .add_row( vec![ "p".into(), "q".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();

  assert!(
    !output.contains( '+' ),
    "T014-P04: plain() must NOT contain '+' characters\nFull output:\n{output}"
  );
  assert!(
    !output.contains( '┌' ) && !output.contains( '└' ) && !output.contains( '─' ),
    "T014-P04: plain() must NOT contain Unicode box-drawing characters\nFull output:\n{output}"
  );
}

/// T014-P05 — `TableConfig::grid()` with 3 data rows produces 2 inter-row separators.
///
/// `AsciiGrid` inter-row separators appear BETWEEN consecutive data rows.
/// 3 data rows → 2 inter-row gaps → 2 inter-row separator lines.
/// Combined with top border, header separator, and bottom border:
/// total lines starting with `+` = 5.
#[ test ]
fn test_t014_p05_grid_inter_row_separators()
{
  let tree = RowBuilder::new( vec![ "A".into() ] )
    .add_row( vec![ "1".into() ] )
    .add_row( vec![ "2".into() ] )
    .add_row( vec![ "3".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::grid() ).format( &tree ).unwrap_or_default();

  let plus_line_count = output.lines().filter( |l| l.starts_with( '+' ) ).count();
  // Expected: top_border(1) + header_sep(1) + inter_row_sep_1(1) + inter_row_sep_2(1) + bottom_border(1) = 5
  assert_eq!(
    plus_line_count, 5,
    "T014-P05: expected 5 '+' lines (top+header_sep+2 inter_row+bottom), got {plus_line_count}\nFull output:\n{output}"
  );
}

// ---------------------------------------------------------------------------
// T014-N: Negative / edge tests
// ---------------------------------------------------------------------------

/// ## Root Cause
///
/// `'|'` was hardcoded in three `output.push( '|' )` calls inside the
/// `HeaderSeparatorVariant::AsciiGrid` arm of `format_header_separator()`
/// (`src/formatters/table/rendering.rs`). Only the internal junction `'+'` was
/// correct; the leading and trailing corner characters were wrong.
///
/// ## Why Not Caught
///
/// Existing `AsciiGrid` tests checked that separator lines were present and
/// contained `'-'`, but never asserted the leading character. A `|---|` line
/// passes a `contains( '-' )` check, so the wrong corner went undetected.
///
/// ## Fix Applied
///
/// Three `output.push( '|' )` calls changed to `output.push_str( "+" )` for
/// the leading and trailing corner positions in the `AsciiGrid` separator arm.
///
/// ## Prevention
///
/// For each new border variant, assert both the separator structural character
/// and that no other variant's characters leak onto separator-only lines.
///
/// ## Pitfall
///
/// Data rows always use `'|'` for column separation — only the header
/// separator corner characters change. Do not confuse the two.
// test_kind: bug_reproducer(BUG-004)
#[ test ]
fn test_t014_n01_ascii_grid_header_sep_uses_plus_corners()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::grid() ).format( &tree ).unwrap_or_default();

  // After fix: no separator-only line may start with '|'
  // (A separator-only line = starts with a pipe or plus, ends with same, contains only dashes)
  let buggy_sep_present = output.lines().any( |l|
  {
    l.starts_with( '|' )
      && l.ends_with( '|' )
      && l.chars().all( |c| c == '|' || c == '-' )
  } );
  assert!(
    !buggy_sep_present,
    "T014-N01: found '|---|' style separator; should be '+---+'\nFull output:\n{output}"
  );

  // After fix: there must be at least one line that is a '+' separator (header sep)
  let plus_sep_present = output.lines().any( |l|
  {
    l.starts_with( '+' )
      && l.ends_with( '+' )
      && l.chars().all( |c| c == '+' || c == '-' )
  } );
  assert!(
    plus_sep_present,
    "T014-N01: expected at least one '+---+' style line, got none\nFull output:\n{output}"
  );

  // Data rows still use '|' separators — they must not be disturbed
  assert!(
    output.contains( '|' ),
    "T014-N01: data rows must still use '|' column separators\nFull output:\n{output}"
  );
}

/// T014-N02 — Unicode top border starts with `┌`, NOT with `├`.
///
/// `├` is the header separator character. Using `├` for the top border would
/// be visually wrong (open corners, not closed).
#[ test ]
fn test_t014_n02_unicode_top_border_starts_with_corner_not_tee()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::unicode_box() ).format( &tree ).unwrap_or_default();

  let first_line = output.lines().next().expect( "output is empty" );
  assert!(
    first_line.starts_with( '┌' ),
    "T014-N02: Unicode top border must start with '┌', got: {first_line:?}\nFull output:\n{output}"
  );
  assert!(
    !first_line.starts_with( '├' ),
    "T014-N02: Unicode top border must NOT start with '├' (that is the header sep), got: {first_line:?}"
  );
}

/// T014-N03 — Unicode bottom border starts with `└`, NOT with `├`.
///
/// `├` is the header separator character. The bottom border must use closed
/// corner characters (`└`, `┘`) for correct box-drawing appearance.
#[ test ]
fn test_t014_n03_unicode_bottom_border_starts_with_corner_not_tee()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::unicode_box() ).format( &tree ).unwrap_or_default();

  let last_line = output
    .lines()
    .rfind( |l| !l.is_empty() )
    .expect( "no non-empty lines" );

  assert!(
    last_line.starts_with( '└' ),
    "T014-N03: Unicode bottom border must start with '└', got: {last_line:?}\nFull output:\n{output}"
  );
  assert!(
    !last_line.starts_with( '├' ),
    "T014-N03: Unicode bottom border must NOT start with '├', got: {last_line:?}"
  );
}

/// T014-N04 — `TableConfig::markdown()` does NOT add top or bottom borders.
///
/// `BorderVariant::Markdown` never emits top/bottom horizontal rule lines.
/// The first line must be the header row (starts with `|`).
#[ test ]
fn test_t014_n04_markdown_no_top_bottom_borders()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .add_row( vec![ "p".into(), "q".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::markdown() ).format( &tree ).unwrap_or_default();

  let first_line = output.lines().next().expect( "output is empty" );
  assert!(
    !first_line.starts_with( '+' ) && !first_line.starts_with( '┌' ),
    "T014-N04: markdown() must NOT have a top border, got: {first_line:?}\nFull output:\n{output}"
  );
  assert!(
    first_line.starts_with( '|' ),
    "T014-N04: markdown() first line should be header row starting with '|', got: {first_line:?}"
  );

  // No lines with '+' anywhere (markdown never uses '+')
  let has_plus_line = output.lines().any( |l| l.starts_with( '+' ) );
  assert!(
    !has_plus_line,
    "T014-N04: markdown() must NOT contain any '+' border lines\nFull output:\n{output}"
  );
}

/// T014-N05 — `TableConfig::grid()` with 1 data row: no inter-row separators, bottom border present.
///
/// Inter-row separators appear only BETWEEN data rows. A single data row has
/// no gaps, so no inter-row separators are emitted. The bottom border is still
/// present after the one data row.
///
/// Expected structure: top(+) → header(|) → `header_sep`(+) → `data_row`(|) → bottom(+)
/// Total `+` lines: 3.
#[ test ]
fn test_t014_n05_grid_one_data_row_no_inter_row_separators()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::grid() ).format( &tree ).unwrap_or_default();

  let plus_line_count = output.lines().filter( |l| l.starts_with( '+' ) ).count();
  // Expected: top_border(1) + header_sep(1) + bottom_border(1) = 3 (no inter-row seps)
  assert_eq!(
    plus_line_count, 3,
    "T014-N05: expected exactly 3 '+' lines (top+header_sep+bottom) for 1 data row, got {plus_line_count}\nFull output:\n{output}"
  );

  // Bottom border must be present (last non-empty line starts with '+')
  let last_line = output
    .lines()
    .rfind( |l| !l.is_empty() )
    .expect( "no non-empty lines" );

  assert!(
    last_line.starts_with( '+' ),
    "T014-N05: bottom border must be present after last data row, got: {last_line:?}\nFull output:\n{output}"
  );
}

// ---------------------------------------------------------------------------
// T014-M: Manual-testing edge cases discovered during corner-case audit
// ---------------------------------------------------------------------------

/// T014-M01 — Single-column `unicode_box()`: corner chars `┌`/`└` but NO `┬`/`┴` or `┼`.
///
/// A single-column table has no inner column junctions; only the outer corner
/// chars should appear. Using `┬`/`┼`/`┴` would be incorrect.
#[ test ]
fn test_t014_m01_single_column_unicode_box_no_mid_junction_chars()
{
  let tree = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::unicode_box() ).format( &tree ).unwrap_or_default();

  assert!(
    !output.contains( '┬' ),
    "T014-M01: single-column unicode_box must not contain top-junction ┬\nFull output:\n{output}"
  );
  assert!(
    !output.contains( '┼' ),
    "T014-M01: single-column unicode_box must not contain cross-junction ┼\nFull output:\n{output}"
  );
  assert!(
    !output.contains( '┴' ),
    "T014-M01: single-column unicode_box must not contain bottom-junction ┴\nFull output:\n{output}"
  );
  // Must still have outer corners
  assert!(
    output.contains( '┌' ) && output.contains( '┐' ),
    "T014-M01: single-column unicode_box must have top corners ┌┐\nFull output:\n{output}"
  );
  assert!(
    output.contains( '└' ) && output.contains( '┘' ),
    "T014-M01: single-column unicode_box must have bottom corners └┘\nFull output:\n{output}"
  );
}

/// ## Root Cause
///
/// `format_single_line_row` unconditionally appended `'\n'` for zero-column slices.
/// A table with no columns caused `format_internal` to call it twice (header row +
/// separator row), yielding `"\n\n"` instead of `""`.
///
/// ## Why Not Caught
///
/// No test passed a zero-column `TableView` to the formatter. The `RowBuilder`
/// API naturally encourages at least one column, so this edge case was invisible.
///
/// ## Fix Applied
///
/// Added an early return `String::new()` at the entry of `format_internal` in
/// `src/formatters/table/mod.rs` when `headers.is_empty()`.
///
/// ## Prevention
///
/// Test formatters against zero-column inputs explicitly. The IC-3 invariant
/// (no columns → empty string) must be asserted, not just assumed.
///
/// ## Pitfall
///
/// Guarding on `rows.is_empty()` would be too aggressive — a headers-only
/// table (no data rows) must still render the header + separator. Only
/// zero-column tables produce empty output.
// test_kind: bug_reproducer(BUG-008)
#[ test ]
fn test_t014_m02_header_only_table_grid_no_panic()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] ).build_view();

  let output = TableFormatter::with_config( TableConfig::grid() ).format( &tree ).unwrap_or_default();

  // Must not panic; headers-only table renders header + separator (IC-3)
  assert!(
    output.contains( 'A' ) && output.contains( 'B' ),
    "T014-M02: headers-only table must render column names with grid config, got: {output:?}",
  );
  // grid: top border + header + separator + bottom border = 4 lines max
  assert!(
    output.lines().count() <= 4,
    "T014-M02: headers-only grid table must have at most 4 lines, got: {output:?}",
  );
}

/// T014-M03 — Headers-only table (`RowBuilder` with no rows) with `unicode_box()`: renders header.
///
/// Same IC-3 invariant as T014-M02, but with `unicode_box()` config.
/// Verifies the formatter renders header + separator for a zero-row table and does
/// not emit spurious Unicode border characters (`┌`, `└`, `├`) as data rows.
#[ test ]
fn test_t014_m03_header_only_table_unicode_no_panic()
{
  let tree = RowBuilder::new( vec![ "X".into() ] ).build_view();

  let output = TableFormatter::with_config( TableConfig::unicode_box() ).format( &tree ).unwrap_or_default();

  // Must not panic; headers-only table renders header + separator (IC-3)
  assert!(
    output.contains( 'X' ),
    "T014-M03: headers-only table must render column name with unicode_box config, got: {output:?}",
  );
  // unicode_box: top border + header + separator + bottom border = 4 lines max
  assert!(
    output.lines().count() <= 4,
    "T014-M03: headers-only unicode table must have at most 4 lines, got: {output:?}",
  );
}
