//! Tests for terminal width detection three-tier fallback (Task 021)
//!
//! ## What This Tests
//!
//! Verifies `resolve_terminal_width` behavior through real `TableFormatter` rendering.
//! All but test 1 use explicit `terminal_width(Some(N))` (Tier 1 override) for
//! determinism. See `docs/feature/005_auto_fit.md § Terminal Width Detection` for the
//! full three-tier fallback specification.
//!
//! ## Tier Coverage
//!
//! - Tier 1 (explicit `Some(w)` override): tests 2–7
//! - Tier 2 (`terminal_size` crate query): verified by compilation with
//!   `--features terminal_size`; not unit-tested deterministically because
//!   `cargo nextest` redirects stdout so `terminal_size::terminal_size()` returns None
//! - Tier 3 (120-column hardcoded fallback): test 1 — valid in CI/non-TTY environments;
//!   may differ in an interactive terminal session where Tier 2 activates first

#![ cfg( feature = "enabled" ) ]
use data_fmt::{ RowBuilder, TableFormatter, TableConfig, TableView, Format };

// --- Test helpers ---

/// Build a wide 5-column table with natural width ≈ 160 chars.
/// Headers are intentionally short (≤ 7 chars) so that at a 40-col terminal the
/// column widths are budget-constrained, not header-constrained.
fn wide_table() -> TableView
{
  RowBuilder::new( vec![
    "ID".into(),
    "Author".into(),
    "Path".into(),
    "Notes".into(),
    "State".into(),
  ] )
  .add_row( vec![
    "1".into(),
    "Alice Wonderland".into(),
    "/home/user1/projects/long-project-name/src/module/file.rs".into(),
    "This is a very detailed description of the item that goes on and on".into(),
    "active".into(),
  ] )
  .add_row( vec![
    "2".into(),
    "Bob Builder".into(),
    "/home/user1/projects/another-project/src/components/widget.rs".into(),
    "Another detailed description with lots of text to ensure terminal overflow".into(),
    "inactive".into(),
  ] )
  .build_view()
}

/// Build a narrow 5-column table with natural width < 40 chars
fn narrow_table() -> TableView
{
  RowBuilder::new( vec![
    "ID".into(),
    "Val".into(),
    "Tag".into(),
    "Kind".into(),
    "Flag".into(),
  ] )
  .add_row( vec![
    "1".into(), "abc".into(), "test".into(), "alpha".into(), "on".into(),
  ] )
  .add_row( vec![
    "2".into(), "xyz".into(), "prod".into(), "beta".into(), "off".into(),
  ] )
  .build_view()
}

fn max_visual_line_width( output : &str ) -> usize
{
  output.lines().map( data_fmt::visual_len ).max().unwrap_or( 0 )
}

// --- T1: Default config falls back to 120-column budget (Tier 3) ---
// NOTE: Deterministic only in non-TTY environments (CI, cargo nextest).
// In an interactive terminal Tier 2 may activate first and use the actual terminal width.

#[ test ]
fn default_config_respects_120_column_fallback()
{
  let tree = wide_table();
  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = formatter.format( &tree ).unwrap_or_default();

  let max_width = max_visual_line_width( &output );
  assert!(
    max_width <= 122, // 2-col tolerance for separator rounding
    "expected output within 120-col budget (Tier 3 fallback), max line width = {max_width}\n{output}",
  );
}

// --- T2: Explicit terminal_width(Some(80)) constrains output to 80 columns ---

#[ test ]
fn explicit_80_column_terminal_width_constrains_output()
{
  let tree = wide_table();
  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 80 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  let max_width = max_visual_line_width( &output );
  assert!(
    max_width <= 82, // 2-col tolerance for separator rounding
    "expected output within 80-col budget (Tier 1 override), max line width = {max_width}\n{output}",
  );
}

// --- T3: Explicit terminal_width(Some(40)) wraps cells aggressively ---

#[ test ]
fn explicit_40_column_terminal_width_aggressive_wrap()
{
  let tree = wide_table();
  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 40 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  let max_width = max_visual_line_width( &output );
  assert!(
    max_width <= 42, // 2-col tolerance
    "expected output within 40-col budget, max line width = {max_width}\n{output}",
  );
  // At 40 cols with wide content, rows must span multiple lines
  let line_count = output.lines().count();
  assert!(
    line_count > 4, // header + sep + at least one wrapped data row = more than 4
    "expected wrapping to produce more than 4 lines, got {line_count}\n{output}",
  );
}

// --- T4: terminal_width(Some(0)) clamps to 1 without panic ---

#[ test ]
fn zero_terminal_width_clamped_no_panic()
{
  let tree = wide_table();
  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 0 ) )
  );
  // Must not panic — resolve_terminal_width clamps 0 → 1
  let output = formatter.format( &tree ).unwrap_or_default();
  assert!( !output.is_empty(), "output should not be empty even with zero width config" );
}

// --- T5: Narrow table fits in 80-col budget — no wrapping, output unchanged ---

#[ test ]
fn narrow_table_unaffected_by_wide_terminal_width()
{
  let output_constrained = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 80 ) )
  ).format( &narrow_table() ).unwrap_or_default();

  let output_unconstrained = TableFormatter::with_config(
    TableConfig::plain().auto_wrap( false )
  ).format( &narrow_table() ).unwrap_or_default();

  assert_eq!(
    output_constrained, output_unconstrained,
    "narrow table that fits in 80 cols should not be modified by auto-wrap",
  );
}

// --- T6: auto_wrap(false) bypasses terminal detection entirely ---

#[ test ]
fn auto_wrap_false_bypasses_terminal_detection()
{
  let tree = wide_table();
  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 80 ) ).auto_wrap( false )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  let max_width = max_visual_line_width( &output );
  assert!(
    max_width > 80,
    "with auto_wrap(false), output should exceed 80-col terminal width, got max_width = {max_width}",
  );
}

// --- T7: CSV preset bypasses auto-fit regardless of terminal_width ---

#[ test ]
fn csv_preset_bypasses_auto_fit_regardless_of_terminal_width()
{
  let tree = wide_table();
  let formatter = TableFormatter::with_config(
    TableConfig::csv().terminal_width( Some( 80 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // CSV values must not be wrapped or truncated — splitting CSV fields corrupts data
  let long_path = "/home/user1/projects/long-project-name/src/module/file.rs";
  assert!(
    output.contains( long_path ),
    "CSV output must preserve full field values, long path not found in:\n{output}",
  );
  // CSV output must use comma separators (no table box-drawing)
  let first_data_line = output.lines().nth( 1 ).unwrap_or( "" );
  assert!(
    first_data_line.contains( ',' ),
    "CSV output must use comma separators, first data line: '{first_data_line}'",
  );
}
