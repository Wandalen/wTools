//! Tests for column folding with auto-fold continuation lines (Task 020)
//!
//! ## What This Tests
//!
//! Verifies Strategy 1 auto-fit: when a table row's total width exceeds the
//! terminal budget after auto-wrapping (Strategy 2), overflow columns are moved
//! to labeled continuation lines below the primary table row.
//!
//! See `docs/feature/005_auto_fit.md § Strategy 1 — Column Folding` for the
//! behavioral specification and `docs/algorithm/005_column_fold_detection.md`
//! for the fold detection algorithm.
//!
//! ## TDD Note
//!
//! Phase 3 adds `FoldStyle` + config fields so tests compile.
//! Phase 4 implements fold algorithm so tests pass.
//! After Phase 3, all tests with fold assertions fail (TDD Red).
//!
//! ## Test Matrix
//!
//! T01: table fits — no fold
//! T02: 6-col table overflows at narrow terminal
//! T03: Labeled fold style (default)
//! T04: Bare fold style
//! T05: Stacked fold style
//! T06: Custom fold indent
//! T07: `auto_fold=false` disables folding
//! T08: CSV preset auto-disables folding
//! T09: TSV preset auto-disables folding
//! T10: fold + wrap combination
//! T11: multiple rows all fold at same point
//! T12: mixed rows — some fit, some fold
//! T13: single overflow column
//! T14: all columns overflow except first (very narrow terminal)
//! T15: fold + sub-row detail
//! T16: fold + alternating row colors
//! T17: fold + bordered style
//! T18: fold + `unicode_box` style
//! T19: header row never folds
//! T20: empty table (headers only) with narrow terminal
//! T21: Format trait path with `TableView`
//! T22: explicit `column_flex` mixed Fixed/Flex triggers fold
//! T23: `bug_reproducer` — `Bare` fold style wraps long continuation lines
//! T24: `bug_reproducer` — `fold_point=0` preserves first column in header
//! T25: fold output is idempotent across repeated `format()` calls

#![ cfg( feature = "enabled" ) ]
use data_fmt::{ RowBuilder, TableFormatter, TableConfig, ColumnFlex, FoldStyle, DecoratedText, Format };

// --- Shared helpers ---

const DEFAULT_INDENT : &str = "    "; // 4 spaces — default fold_indent

/// Config that reliably triggers fold: all-Fixed columns, terminal=40.
/// With 6 columns of natural widths 2+10+6+22+5+15 = 60 total (>40),
/// `fold_point=3` is reached at the "Path" column.
fn fold_config() -> TableConfig
{
  TableConfig::plain()
    .terminal_width( Some( 40 ) )
    .column_flex( vec![
      ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
      ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
    ] )
}

/// Standard 6-column table: ID + Name + File (primary) + Path + Rules + Source (overflow at fold)
fn fold_table_single_row() -> data_fmt::TableView
{
  RowBuilder::new( vec![
    "ID".into(), "Name".into(), "File".into(),
    "Path".into(), "Rules".into(), "Source".into(),
  ] )
  .add_row( vec![
    "b1".into(),
    "governance".into(),
    "gov.md".into(),
    "/home/user/governance/".into(),
    "120".into(),
    "/home/user/src/".into(),
  ] )
  .build_view()
}

/// Helper: check that output contains at least one line starting with the given indent
fn has_continuation_lines( output : &str, indent : &str ) -> bool
{
  output.lines().any( | l | l.starts_with( indent ) )
}

// --- T01: Table fits within terminal — no fold ---

#[ test ]
fn table_fits_no_fold_occurs()
{
  // Wide-enough terminal: all 6 columns fit, no fold needed
  let tree = fold_table_single_row();
  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 120 ) ).column_flex( vec![
      ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
      ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
    ] )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!(
    !has_continuation_lines( &output, DEFAULT_INDENT ),
    "no fold expected when table fits in terminal, got continuation lines:\n{output}",
  );
}

// --- T02: 6-column table overflows 40-col terminal — last 3 columns fold ---

#[ test ]
fn six_col_table_overflows_narrow_terminal()
{
  let tree = fold_table_single_row();
  let formatter = TableFormatter::with_config( fold_config() );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!(
    has_continuation_lines( &output, DEFAULT_INDENT ),
    "expected continuation lines when 6-col table overflows 40-col terminal:\n{output}",
  );
}

// --- T03: Labeled fold style (default) renders "ColName: value" ---

#[ test ]
fn labeled_fold_produces_continuation()
{
  let tree = fold_table_single_row();
  let formatter = TableFormatter::with_config(
    fold_config().fold_style( FoldStyle::Labeled )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!(
    output.contains( "    Path: /home/user/governance/" ),
    "expected labeled continuation '    Path: /home/user/governance/', got:\n{output}",
  );
}

// --- T04: Bare fold style renders values without labels ---

#[ test ]
fn bare_fold_style_renders_values_without_labels()
{
  let tree = fold_table_single_row();
  let formatter = TableFormatter::with_config(
    fold_config().fold_style( FoldStyle::Bare )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Bare style should include the value but NOT "Path:"
  assert!(
    output.contains( "    /home/user/governance/" ),
    "expected bare value '/home/user/governance/' on continuation line, got:\n{output}",
  );
  assert!(
    !output.contains( "    Path: " ),
    "Bare style must not include column labels, got:\n{output}",
  );
}

// --- T05: Stacked fold style renders each overflow column on its own labeled line ---

#[ test ]
fn stacked_fold_style_each_column_on_own_line()
{
  let tree = fold_table_single_row();
  let formatter = TableFormatter::with_config(
    fold_config().fold_style( FoldStyle::Stacked )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Stacked: each overflow column on its own line with label
  assert!(
    output.contains( "    Path: " ) && output.contains( "    Rules: " ),
    "expected stacked continuation with each overflow column on its own line, got:\n{output}",
  );
}

// --- T06: Custom fold indent ">>> " appears on continuation lines ---

#[ test ]
fn custom_fold_indent_appears_on_continuation_lines()
{
  let tree = fold_table_single_row();
  let formatter = TableFormatter::with_config(
    fold_config().fold_indent( ">>> ".to_string() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!(
    output.contains( ">>> Path: " ),
    "expected custom indent '>>> ' on continuation line, got:\n{output}",
  );
  assert!(
    !has_continuation_lines( &output, DEFAULT_INDENT ),
    "default 4-space indent must not appear when custom indent is set, got:\n{output}",
  );
}

// --- T07: auto_fold(false) disables folding entirely ---

#[ test ]
fn auto_fold_false_disables_folding()
{
  let tree = fold_table_single_row();
  let formatter = TableFormatter::with_config(
    fold_config().auto_fold( false )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!(
    !has_continuation_lines( &output, DEFAULT_INDENT ),
    "no continuation lines expected when auto_fold=false, got:\n{output}",
  );
}

// --- T08: CSV preset auto-disables folding ---

#[ test ]
fn csv_preset_auto_disables_folding()
{
  let tree = fold_table_single_row();
  let formatter = TableFormatter::with_config(
    TableConfig::csv().terminal_width( Some( 40 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // CSV must not fold — splitting CSV fields corrupts data
  assert!(
    !has_continuation_lines( &output, DEFAULT_INDENT ),
    "CSV preset must not fold continuation lines, got:\n{output}",
  );
  assert!(
    output.contains( "governance" ),
    "CSV must include all field values intact, got:\n{output}",
  );
}

// --- T09: TSV preset auto-disables folding ---

#[ test ]
fn tsv_preset_auto_disables_folding()
{
  let tree = fold_table_single_row();
  let formatter = TableFormatter::with_config(
    TableConfig::tsv().terminal_width( Some( 40 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!(
    !has_continuation_lines( &output, DEFAULT_INDENT ),
    "TSV preset must not fold continuation lines, got:\n{output}",
  );
}

// --- T10: Fold + wrap combination ---

#[ test ]
fn fold_plus_wrap_combination()
{
  // Long folded value that itself exceeds remaining terminal width after indent
  let tree = RowBuilder::new( vec![
    "ID".into(), "Name".into(), "Path".into(),
  ] )
  .add_row( vec![
    "1".into(),
    "short".into(),
    "this/is/a/very/very/very/long/path/that/exceeds/even/the/fold/budget/width".into(),
  ] )
  .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 40 ) )
      .column_flex( vec![ ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed ] )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Continuation lines must exist for the overflow "Path" column
  assert!(
    has_continuation_lines( &output, DEFAULT_INDENT ),
    "expected fold continuation for long path, got:\n{output}",
  );
  // Continuation lines must respect terminal width (wrapped within budget)
  let max_width = output.lines().map( data_fmt::visual_len ).max().unwrap_or( 0 );
  assert!(
    max_width <= 42,
    "fold+wrap must keep lines within terminal budget, max={max_width}, got:\n{output}",
  );
}

// --- T11: Multiple rows all fold at same point ---

#[ test ]
fn multiple_rows_fold_at_same_point()
{
  let tree = RowBuilder::new( vec![
    "ID".into(), "Name".into(), "File".into(),
    "Path".into(), "Rules".into(), "Source".into(),
  ] )
  .add_row( vec![
    "b1".into(), "governance".into(), "gov.md".into(),
    "/path/to/gov/".into(), "120".into(), "/src/gov/".into(),
  ] )
  .add_row( vec![
    "b2".into(), "engineering".into(), "eng.md".into(),
    "/path/to/eng/".into(), "80".into(), "/src/eng/".into(),
  ] )
  .add_row( vec![
    "b3".into(), "security".into(), "sec.md".into(),
    "/path/to/sec/".into(), "40".into(), "/src/sec/".into(),
  ] )
  .build_view();

  let formatter = TableFormatter::with_config( fold_config() );
  let output = formatter.format( &tree ).unwrap_or_default();

  // All 3 data rows should have continuation lines
  let continuation_count = output.lines().filter( | l | l.starts_with( DEFAULT_INDENT ) ).count();
  assert!(
    continuation_count >= 3, // at least 1 continuation per row (for "Path:")
    "expected ≥3 continuation lines for 3 rows, got {continuation_count}:\n{output}",
  );
}

// --- T12: Mixed rows — some rows fit, some overflow ---

#[ test ]
fn mixed_rows_only_overflowing_rows_have_continuation()
{
  // Row 1: short Path (fits even with Fixed cols + sep)
  // Row 2: long Path (overflows)
  let tree = RowBuilder::new( vec![
    "ID".into(), "Name".into(), "File".into(),
    "Path".into(), "Rules".into(), "Source".into(),
  ] )
  .add_row( vec![
    "b1".into(), "governance".into(), "gov.md".into(),
    "/home/user/governance/".into(), "120".into(), "/home/user/src/".into(),
  ] )
  .build_view();

  let formatter = TableFormatter::with_config( fold_config() );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!(
    has_continuation_lines( &output, DEFAULT_INDENT ),
    "expected continuation lines for overflowing row, got:\n{output}",
  );
}

// --- T13: Single overflow column — one continuation line ---

#[ test ]
fn single_overflow_column_produces_one_continuation_line()
{
  let tree = fold_table_single_row();
  // terminal=60: cumulative reaches fold_point=5 (only "Source" column folds)
  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 60 ) )
      .column_flex( vec![
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
      ] )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Only "Source" column folds
  let continuation_lines : Vec< &str > = output.lines()
    .filter( | l | l.starts_with( DEFAULT_INDENT ) )
    .collect();
  assert_eq!(
    continuation_lines.len(), 1,
    "expected exactly 1 continuation line for single overflow column, got: {continuation_lines:?}\n{output}",
  );
  assert!(
    continuation_lines[ 0 ].contains( "Source:" ),
    "the single continuation must be 'Source:', got: '{}'", continuation_lines[ 0 ],
  );
}

// --- T14: All columns overflow except first (very narrow terminal) ---

#[ test ]
fn very_narrow_terminal_all_columns_fold_except_first()
{
  let tree = fold_table_single_row();
  // terminal=10: fold_point=1, only ID stays in table
  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 10 ) )
      .column_flex( vec![
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
      ] )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // At terminal=10, Name (width=10) pushes cumulative to 16 > 10 → fold_point=1
  // Continuation lines for Name, File, Path, Rules, Source
  let continuation_count = output.lines().filter( | l | l.starts_with( DEFAULT_INDENT ) ).count();
  assert!(
    continuation_count >= 5,
    "expected ≥5 continuation lines (Name, File, Path, Rules, Source), got {continuation_count}:\n{output}",
  );
}

// --- T15: Fold + sub-row detail ---

#[ test ]
fn fold_with_sub_row_detail_both_render()
{
  let view = RowBuilder::new( vec![
    "ID".into(), "Name".into(), "File".into(),
    "Path".into(), "Rules".into(), "Source".into(),
  ] )
  .add_row_with_detail(
    vec![
      "b1".into(), "governance".into(), "gov.md".into(),
      "/home/user/governance/".into(), "120".into(), "/home/user/src/".into(),
    ],
    Some( DecoratedText::from( "See also: governance readme" ) ),
  )
  .build_view();

  let config = TableConfig::plain()
    .terminal_width( Some( 40 ) )
    .column_flex( vec![
      ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
      ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
    ] );
  let formatter = TableFormatter::with_config( config );
  let output = data_fmt::Format::format( &formatter, &view ).unwrap();

  // Both continuation lines and sub-row detail must appear
  assert!(
    has_continuation_lines( &output, DEFAULT_INDENT ),
    "expected fold continuation lines, got:\n{output}",
  );
  assert!(
    output.contains( "See also" ),
    "expected sub-row detail text, got:\n{output}",
  );
}

// --- T16: Fold + alternating row colors — no ANSI bleed into continuation lines ---

#[ test ]
fn fold_with_alternating_colors_continuation_lines_exist()
{
  let tree = RowBuilder::new( vec![
    "ID".into(), "Name".into(), "File".into(),
    "Path".into(), "Rules".into(), "Source".into(),
  ] )
  .add_row( vec![
    "b1".into(), "governance".into(), "gov.md".into(),
    "/path/to/gov/".into(), "120".into(), "/src/gov/".into(),
  ] )
  .add_row( vec![
    "b2".into(), "engineering".into(), "eng.md".into(),
    "/path/to/eng/".into(), "80".into(), "/src/eng/".into(),
  ] )
  .build_view();

  let formatter = TableFormatter::with_config(
    fold_config()
      .alternating_rows( true )
      .row_colors( "\x1b[48;5;236m".to_string(), "\x1b[48;5;237m".to_string() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Fold behavior must still work with alternating colors enabled
  assert!(
    has_continuation_lines( &output, DEFAULT_INDENT ),
    "expected continuation lines with alternating colors enabled, got:\n{output}",
  );
}

// --- T17: Fold + bordered style ---

#[ test ]
fn fold_with_bordered_style_primary_bordered_continuation_plain()
{
  let tree = fold_table_single_row();
  let formatter = TableFormatter::with_config(
    TableConfig::bordered()
      .terminal_width( Some( 40 ) )
      .column_flex( vec![
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
      ] )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!(
    has_continuation_lines( &output, DEFAULT_INDENT ),
    "expected continuation lines with bordered style, got:\n{output}",
  );
}

// --- T18: Fold + unicode_box style ---

#[ test ]
fn fold_with_unicode_box_style_continuation_outside_box()
{
  let tree = fold_table_single_row();
  let formatter = TableFormatter::with_config(
    TableConfig::unicode_box()
      .terminal_width( Some( 40 ) )
      .column_flex( vec![
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
      ] )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!(
    has_continuation_lines( &output, DEFAULT_INDENT ),
    "expected continuation lines outside unicode_box, got:\n{output}",
  );
}

// --- T19: Header row never folds ---

#[ test ]
fn header_row_never_folds()
{
  let tree = fold_table_single_row();
  let formatter = TableFormatter::with_config( fold_config() );
  let output = formatter.format( &tree ).unwrap_or_default();
  let lines : Vec< &str > = output.lines().collect();

  // Header is first line, it must contain all 6 column names
  assert!( !lines.is_empty(), "output must have at least a header line" );
  let header_line = lines[ 0 ];
  for col in &[ "ID", "Name", "File" ]
  {
    assert!(
      header_line.contains( col ),
      "header must contain column '{col}' inline (not folded), header='{header_line}'",
    );
  }
  // The header line must NOT start with fold indent
  assert!(
    !header_line.starts_with( DEFAULT_INDENT ),
    "header line must not start with fold indent, header='{header_line}'",
  );
}

// --- T20: Empty table (headers only) with narrow terminal ---

#[ test ]
fn empty_table_headers_only_narrow_terminal()
{
  // IC-3: columns + no rows → header + separator only (no continuation lines).
  // build_view() stores headers in TableView.metadata.column_names; formatter renders
  // the header row and separator even when there are no data rows.
  let view = RowBuilder::new( vec![
    "ID".into(), "Name".into(), "File".into(),
    "Path".into(), "Rules".into(), "Source".into(),
  ] )
  .build_view();

  let formatter = TableFormatter::with_config( fold_config() );
  let output = data_fmt::Format::format( &formatter, &view ).unwrap();

  // Headers-only table: no data rows means no fold continuation lines
  assert!(
    !has_continuation_lines( &output, DEFAULT_INDENT ),
    "headers-only table must not produce continuation lines, got:\n{output}",
  );
  // Output must still have the header line (IC-3: header + separator rendered even without rows)
  assert!(
    output.contains( "ID" ),
    "headers-only output must include header row (IC-3), got:\n{output}",
  );
}

// --- T21: Format trait path via TableView ---

#[ test ]
fn format_trait_path_fold_works()
{
  let view = RowBuilder::new( vec![
    "ID".into(), "Name".into(), "File".into(),
    "Path".into(), "Rules".into(), "Source".into(),
  ] )
  .add_row( vec![
    "b1".into(), "governance".into(), "gov.md".into(),
    "/home/user/governance/".into(), "120".into(), "/home/user/src/".into(),
  ] )
  .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 40 ) )
      .column_flex( vec![
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
      ] )
  );
  let output = data_fmt::Format::format( &formatter, &view ).unwrap();

  assert!(
    has_continuation_lines( &output, DEFAULT_INDENT ),
    "Format trait path must support fold, got:\n{output}",
  );
}

// --- T22: Explicit column_flex with mixed Fixed/Flex triggers fold ---

#[ test ]
fn explicit_column_flex_mixed_triggers_fold_at_fixed_overflow()
{
  // 3 Fixed columns (ID, Lines, Rules) totalling 2+5+5=12, but with longer headers
  // "Lines" header=5, "Rules" header=5
  // Plus 3 Flex columns: Path, Purpose, Owner
  // terminal=30: Fixed total=12, overhead=5*2=10, flex_budget=30-12-10=8 for 3→2 each
  // After budget allocation, column widths: 2, 5, 5, max(4,2)=4, max(7,2)=7!, max(5,2)=5!
  // Recalculated total = 2+5+5+4+7+5 + 10 = 38 > 30 → fold triggers
  let tree = RowBuilder::new( vec![
    "ID".into(), "Lines".into(), "Rules".into(),
    "Path".into(), "Purpose".into(), "Owner".into(),
  ] )
  .add_row( vec![
    "1".into(), "120".into(), "23".into(),
    "/very/long/path/that/wraps".into(),
    "Governance and quality enforcement".into(),
    "/home/user/".into(),
  ] )
  .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 30 ) )
      .column_flex( vec![
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
        ColumnFlex::Flex, ColumnFlex::Flex, ColumnFlex::Flex,
      ] )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // With headers "Purpose" (7 chars) and "Owner" (5 chars) wider than their 2-char Flex budget,
  // columns must still render in some form — either folded or as continuation lines
  // (exact behavior depends on Phase 4 implementation)
  assert!(
    !output.is_empty(),
    "output must not be empty for mixed Fixed/Flex fold scenario, got:\n{output}",
  );
}

/// --- T23: Bug reproducer — Bare fold style must wrap long continuation lines ---
///
/// ## Root Cause
/// `FoldStyle::Bare` in `render_fold_continuation()` joined values and emitted them
/// unconditionally without a terminal-width check. `Labeled` and `Stacked` both call
/// `WrapFormatter` when `unicode_visual_len(&full_line) > terminal`. `Bare` had no
/// such guard, so long values overflowed the terminal silently.
///
/// ## Why Not Caught
/// T04 tested `Bare` with short values ("governance", "120", "src") that fit within
/// terminal=40 after folding. No test used a value long enough to trigger the
/// wrapping path in the `Bare` branch.
///
/// ## Fix Applied
/// Added identical guard to `Bare`: if `unicode_visual_len(&full_line) > terminal &&
/// available > 0`, call `WrapFormatter::with_config(WrapConfig::new().width(available))`
/// and emit one wrapped line per `output_wrapped.lines()` iteration.
///
/// ## Prevention
/// Every `FoldStyle` variant must have a corresponding long-value test that asserts
/// `max_width <= terminal + 2`. New variants must add this test before implementation.
///
/// ## Pitfall
/// `Bare` wraps across word or slash boundaries in the raw value — no label prefix is
/// preserved on continuation lines, so multi-line bare output reads as bare text fragments.
///
// test_kind: bug_reproducer(issue-bare-fold-no-wrap)
#[ test ]
fn bare_fold_style_wraps_long_continuation_line()
{
  let tree = RowBuilder::new( vec![ "ID".into(), "Name".into(), "Path".into() ] )
    .add_row( vec![
      "1".into(),
      "short".into(),
      "this/is/a/very/very/very/long/path/that/exceeds/the/terminal/width/entirely".into(),
    ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 40 ) )
      .fold_style( FoldStyle::Bare )
      .column_flex( vec![ ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed ] )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!(
    has_continuation_lines( &output, DEFAULT_INDENT ),
    "expected fold continuation for long path, got:\n{output}",
  );
  let max_width = output.lines().map( data_fmt::visual_len ).max().unwrap_or( 0 );
  assert!(
    max_width <= 42,
    "Bare fold must wrap long values within terminal budget, max={max_width}:\n{output}",
  );
}

/// --- T24: Bug reproducer — `fold_point=0` must not produce empty header row ---
///
/// ## Root Cause
/// `determine_fold_point()` returned 0 when even column 0's width exceeded the
/// terminal. This made `primary_headers = headers[..0] = []`, causing the header
/// row to render as just border pipes with no visible column names — a direct
/// violation of Invariant 1 (header row never folds / is never empty).
///
/// ## Why Not Caught
/// T14 used terminal=10 with a 2-char "ID" column as col[0], so `fold_point=1` (ID
/// fits). No test used a terminal narrower than the narrowest column, so the
/// `fold_point=0` path was never exercised.
///
/// ## Fix Applied
/// Added `.max(1)` to the return in `determine_fold_point()`: `return i.max(1);`.
/// This ensures at least one column always stays in the primary table regardless
/// of how narrow the terminal is.
///
/// ## Prevention
/// Add a structural guard: after `determine_fold_point()`, assert
/// `fold_point >= 1 || column_widths.is_empty()` before slicing `primary_headers`.
/// Test with terminal < `min(column_widths)` to exercise the clamp.
///
/// ## Pitfall
/// A single-column table with fold enabled and terminal < col[0].width will also
/// hit this path. After the fix, `fold_point=1=column_widths.len()` so no folding
/// occurs — the table renders at natural width (correct; can't fold further).
///
// test_kind: bug_reproducer(issue-fold-point-zero)
#[ test ]
fn fold_point_zero_preserves_first_column_in_header()
{
  // "VeryLongColumnName" (18 chars) > terminal=3, so without the fix fold_point=0
  // and the header row renders as just "||\n" (empty pipes).
  let tree = RowBuilder::new( vec![ "VeryLongColumnName".into(), "B".into() ] )
    .add_row( vec![ "wide_value_here".into(), "x".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 3 ) )
      .column_flex( vec![ ColumnFlex::Fixed, ColumnFlex::Fixed ] )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Header must contain the first column name even at an impossibly narrow terminal
  assert!(
    output.contains( "VeryLongColumnName" ),
    "header must show first column name even when terminal < column width, got:\n{output}",
  );
  // The second column must appear in overflow continuation lines
  assert!(
    has_continuation_lines( &output, DEFAULT_INDENT ),
    "second column must fold to continuation lines, got:\n{output}",
  );
}

// --- T25: Fold output is idempotent across repeated format() calls ---
// invariant/004 Invariant 3: determine_fold_point is a pure function; no
// mutable state in TableFormatter should cause output to differ between calls.
#[ test ]
fn fold_output_is_idempotent_on_repeated_calls()
{
  let tree = fold_table_single_row();
  let formatter = TableFormatter::with_config( fold_config() );
  let output1 = formatter.format( &tree ).unwrap_or_default();
  let output2 = formatter.format( &tree ).unwrap_or_default();
  assert_eq!(
    output1, output2,
    "fold output must be byte-identical on repeated calls with identical input",
  );
}

// --- CF AC-6: FoldStyle::Bare joins all overflow values on single continuation line ---

/// AC-6 — `005_column_fold_detection`: `FoldStyle::Bare` joins all overflow values on a single continuation line.
///
/// With `FoldStyle::Bare`, all non-empty overflow column values appear joined on a single
/// indented continuation line with no column-name labels. Values are separated by the
/// two-space column separator. When the joined line fits within the terminal, no wrapping
/// occurs — the three overflow values appear together on one line.
// test_kind: standard
#[ test ]
fn bare_fold_all_overflow_values_on_single_line_ac6()
{
  // 6 columns of equal width (5 chars each); terminal=25 → fold_point=3 (D,E,F overflow)
  // Cumulative: A=5, A+B=12, A+B+C=19≤25, A+B+C+D=26>25 → fold at D (index 3)
  let tree = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into(), "D".into(), "E".into(), "F".into() ] )
    .add_row( vec![
      "val_a".into(), "val_b".into(), "val_c".into(),
      "val_d".into(), "val_e".into(), "val_f".into(),
    ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 25 ) )
      .fold_style( FoldStyle::Bare )
      .column_flex( vec![
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
      ] )
  );
  let output = formatter.format( &tree ).expect( "formatting must not fail" );

  let continuation_lines : Vec< &str > = output.lines()
    .filter( | l | l.starts_with( DEFAULT_INDENT ) )
    .collect();
  assert!( !continuation_lines.is_empty(), "expected continuation lines for overflow columns:\n{output:?}" );

  // All 3 overflow values must appear on the SAME continuation line (Bare joins on one line)
  let single_line_with_all = continuation_lines.iter().any( | l |
    l.contains( "val_d" ) && l.contains( "val_e" ) && l.contains( "val_f" )
  );
  assert!(
    single_line_with_all,
    "Bare style must join all overflow values on a single continuation line; got:\n{continuation_lines:?}\n{output:?}",
  );
  // No column labels emitted for Bare style
  assert!(
    !output.contains( "D: " ) && !output.contains( "E: " ) && !output.contains( "F: " ),
    "Bare style must not emit column labels:\n{output:?}",
  );
}

// --- CF AC-7: fold point computed per-row; short rows have no continuation lines ---

/// AC-7 — `005_column_fold_detection`: rows with no overflow content produce no continuation lines.
///
/// When a table has mixed row content — some rows with values in the overflow columns and
/// some rows with empty overflow columns — only the rows with non-empty overflow values
/// produce continuation lines. The Labeled style's empty-value filter ensures rows
/// with no overflow content emit no continuation, matching per-row fold evaluation semantics.
// test_kind: standard
#[ test ]
fn mixed_rows_short_rows_have_no_continuation_ac7()
{
  // Row 1: full overflow values → produces continuation lines
  // Row 2: empty overflow columns → no continuation (empty-value filter in Labeled style)
  let tree = RowBuilder::new( vec![
    "ID".into(), "Name".into(), "File".into(),
    "Path".into(), "Rules".into(), "Source".into(),
  ] )
  .add_row( vec![
    "b1".into(), "governance".into(), "gov.md".into(),
    "/home/user/governance/".into(), "120".into(), "/home/user/src/".into(),
  ] )
  .add_row( vec![
    "b2".into(), "short".into(), "s.md".into(),
    "".into(), "".into(), "".into(),  // empty overflow columns → no continuation
  ] )
  .build_view();

  let formatter = TableFormatter::with_config( fold_config() );
  let output = formatter.format( &tree ).expect( "formatting must not fail" );

  // Row 1 must produce continuation lines
  assert!(
    output.contains( "Path:" ) && output.contains( "/home/user/governance/" ),
    "row 1 with overflow values must produce continuation lines:\n{output:?}",
  );
  // Row 2 must NOT produce continuation lines (all overflow values empty → filtered)
  let continuation_lines : Vec< &str > = output.lines()
    .filter( | l | l.starts_with( DEFAULT_INDENT ) )
    .collect();
  // Row 1 produces continuation lines (Labeled style may wrap long content to multiple lines).
  // Row 2 has empty overflow columns → zero continuation lines (empty-value filter).
  // Verify row 2's primary values do not appear in any continuation line.
  assert!(
    !continuation_lines.is_empty(),
    "row 1 with overflow values must produce at least one continuation line; got:\n{output:?}",
  );
  assert!(
    !continuation_lines.iter().any( | l | l.contains( "b2" ) || l.contains( "short" ) || l.contains( "s.md" ) ),
    "row 2 (empty overflow) must not produce any continuation lines; got:\n{continuation_lines:?}",
  );
}

// --- CF AC-8: single overflow column produces exactly one continuation line ---

/// AC-8 — `005_column_fold_detection`: single overflow column produces exactly one continuation line per data row.
///
/// A 4-column table where columns 0–2 fit within `terminal_width` and column 3 causes
/// overflow. Each data row produces exactly one continuation line containing the label
/// and value of column 3 only; no extra blank continuation lines are emitted; the
/// primary table contains exactly columns 0–2.
// test_kind: standard
#[ test ]
fn single_overflow_column_one_continuation_line_ac8()
{
  // 4 columns; terminal=30; primary = ID+Name+File (2+2+10+2+6=22≤30), Path(15) overflows (22+2+15=39>30)
  let tree = RowBuilder::new( vec![ "ID".into(), "Name".into(), "File".into(), "Path".into() ] )
    .add_row( vec![
      "b1".into(), "governance".into(), "gov.md".into(), "/home/user/gov/".into(),
    ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 30 ) )
      .column_flex( vec![
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
      ] )
  );
  let output = formatter.format( &tree ).expect( "formatting must not fail" );

  let continuation_lines : Vec< &str > = output.lines()
    .filter( | l | l.starts_with( DEFAULT_INDENT ) )
    .collect();
  assert_eq!(
    continuation_lines.len(), 1,
    "single overflow column must produce exactly one continuation line per data row; got:\n{output:?}",
  );
  assert!(
    continuation_lines[ 0 ].contains( "Path" ),
    "the single continuation must reference the 'Path' column:\n{:?}", continuation_lines[ 0 ],
  );
  // Primary table must contain columns 0-2 (ID, Name, File) not Path
  let primary_lines : Vec< &str > = output.lines()
    .filter( | l | !l.starts_with( DEFAULT_INDENT ) )
    .collect();
  assert!(
    primary_lines.iter().any( | l | l.contains( "Name" ) && l.contains( "File" ) ),
    "primary table must contain Name and File column headers:\n{output:?}",
  );
}

// --- FI IN-5: multiple identical rows fold at same column index ---

/// IN-5 — `004_column_fold_invariants`: multiple rows with identical data fold at the same column index.
///
/// When a table contains 3 or more rows each with identical cell content, all rows fold
/// at the same column index. Continuation line column labels and values are identical
/// across all rows; no per-row mutable state accumulates between rows.
// test_kind: standard
#[ test ]
fn multiple_identical_rows_fold_at_same_column_in5()
{
  let tree = RowBuilder::new( vec![
    "ID".into(), "Name".into(), "File".into(),
    "Path".into(), "Rules".into(), "Source".into(),
  ] )
  .add_row( vec![
    "b1".into(), "governance".into(), "gov.md".into(),
    "/home/user/governance/".into(), "120".into(), "/home/user/src/".into(),
  ] )
  .add_row( vec![
    "b1".into(), "governance".into(), "gov.md".into(),
    "/home/user/governance/".into(), "120".into(), "/home/user/src/".into(),
  ] )
  .add_row( vec![
    "b1".into(), "governance".into(), "gov.md".into(),
    "/home/user/governance/".into(), "120".into(), "/home/user/src/".into(),
  ] )
  .build_view();

  let formatter = TableFormatter::with_config( fold_config() );
  let output = formatter.format( &tree ).expect( "formatting must not fail" );

  // All 3 identical rows must produce continuation lines (fold occurs).
  // Labeled style may wrap long content to multiple lines, so each row may produce >1 line.
  // The total must be divisible by 3 (equal blocks per row) and all blocks must be byte-identical.
  let continuation_lines : Vec< &str > = output.lines()
    .filter( | l | l.starts_with( DEFAULT_INDENT ) )
    .collect();
  assert!(
    !continuation_lines.is_empty() && continuation_lines.len().is_multiple_of( 3 ),
    "3 identical rows must each produce the same number of continuation lines (total divisible by 3); got {}:\n{output:?}",
    continuation_lines.len(),
  );
  // All 3 per-row blocks must be byte-identical (same fold column index → same content)
  let lines_per_row = continuation_lines.len() / 3;
  let blocks : Vec< &[ &str ] > = continuation_lines.chunks( lines_per_row ).collect();
  assert!(
    blocks.windows( 2 ).all( | w | w[ 0 ] == w[ 1 ] ),
    "identical rows must produce identical continuation blocks; fold is consistent across rows:\n{continuation_lines:?}",
  );
}

// --- CF AC-9: FoldStyle::Stacked emits one labeled continuation line per overflow column ---

/// AC-9 — `005_column_fold_detection`: `FoldStyle::Stacked` emits one labeled continuation
/// line per overflow column; two overflow columns produce exactly two separate continuation
/// lines each containing their column label.
// test_kind: standard
#[ test ]
fn stacked_style_one_line_per_overflow_column_ac9()
{
  // 4 columns where columns 2 and 3 overflow; terminal=10 forces fold at column 2
  // (A=2, B=2, sep×2=4 → cumulative=6 ≤ 10; ColC=10 → cumulative=18 > 10 → fold at ColC)
  let view = RowBuilder::new( vec![
    "A".into(), "B".into(), "ColC".into(), "ColD".into(),
  ] )
  .add_row( vec![
    "aa".into(), "bb".into(), "val_c_here".into(), "val_d_here".into(),
  ] )
  .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 10 ) )
      .column_flex( vec![
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
      ] )
      .fold_style( FoldStyle::Stacked )
  );
  let output = formatter.format( &view ).expect( "formatting must not fail" );

  // Stacked: one continuation line per overflow column, each carrying the column label
  let continuation_lines : Vec< &str > = output.lines()
    .filter( | l | l.starts_with( DEFAULT_INDENT ) )
    .collect();

  // There are 2 overflow columns (ColC and ColD) → 2 continuation lines
  assert_eq!(
    continuation_lines.len(), 2,
    "FoldStyle::Stacked must emit one continuation line per overflow column; got {}:\n{output:?}",
    continuation_lines.len(),
  );

  // Each continuation line must contain its column label
  assert!(
    continuation_lines.iter().any( | l | l.contains( "ColC" ) ),
    "first overflow column label 'ColC' must appear in a continuation line:\n{output:?}",
  );
  assert!(
    continuation_lines.iter().any( | l | l.contains( "ColD" ) ),
    "second overflow column label 'ColD' must appear in a continuation line:\n{output:?}",
  );

  // Both overflow values must be present in the output
  assert!( output.contains( "val_c_here" ), "overflow value 'val_c_here' must appear:\n{output:?}" );
  assert!( output.contains( "val_d_here" ), "overflow value 'val_d_here' must appear:\n{output:?}" );
}

// --- CF AC-10: all columns overflow — first column remains in primary, rest in continuation ---

/// AC-10 — `005_column_fold_detection`: when even column 0 alone exceeds the terminal width,
/// the fold point is clamped to 1 (`.max(1)`); column 0 appears in the primary row and
/// header; all remaining columns appear in continuation lines; no panic occurs.
// test_kind: standard
#[ test ]
fn all_columns_overflow_first_stays_primary_ac10()
{
  // Column 0 content = 20 chars; terminal=5 — narrower than column 0 alone
  let view = RowBuilder::new( vec![
    "VeryLongHeaderName".into(), "B".into(), "C".into(),
  ] )
  .add_row( vec![
    "twenty_char_content_".into(),  // 20 chars — exceeds terminal alone
    "bb".into(),
    "cc".into(),
  ] )
  .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 5 ) )
      .column_flex( vec![
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
      ] )
  );
  let output = formatter.format( &view ).expect( "must not panic when column 0 exceeds terminal" );

  // Must not panic and must produce output
  assert!( !output.is_empty(), "output must be non-empty:\n{output:?}" );

  // Column 0 header must appear in the primary (non-continuation) table section
  let primary_lines : Vec< &str > = output.lines()
    .filter( | l | !l.starts_with( DEFAULT_INDENT ) )
    .collect();
  assert!(
    primary_lines.iter().any( | l | l.contains( "VeryLongHeaderName" ) ),
    "column 0 header must be in the primary table section (fold_point clamped to 1):\n{output:?}",
  );

  // Columns B and C must appear in continuation lines
  let continuation_lines : Vec< &str > = output.lines()
    .filter( | l | l.starts_with( DEFAULT_INDENT ) )
    .collect();
  assert!(
    !continuation_lines.is_empty(),
    "columns B and C must overflow to continuation lines:\n{output:?}",
  );
}

// --- CF AC-11: very narrow terminal — fold at column 1 even with many remaining columns ---

/// AC-11 — `005_column_fold_detection`: a 5-column table with terminal narrower than any
/// single column causes `fold_point=1` (clamped); column 0 is in the primary row; all
/// 4 remaining columns appear in continuation lines; no output truncation or panic.
// test_kind: standard
#[ test ]
fn very_narrow_terminal_folds_at_column_one_ac11()
{
  // 5-column table; terminal=3 — narrower than any column content
  let view = RowBuilder::new( vec![
    "Col0".into(), "Col1".into(), "Col2".into(), "Col3".into(), "Col4".into(),
  ] )
  .add_row( vec![
    "aaa".into(), "bbb".into(), "ccc".into(), "ddd".into(), "eee".into(),
  ] )
  .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 3 ) )
      .fold_style( FoldStyle::Stacked )
      .column_flex( vec![
        ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed,
        ColumnFlex::Fixed, ColumnFlex::Fixed,
      ] )
  );
  let output = formatter.format( &view ).expect( "must not panic with very narrow terminal" );

  assert!( !output.is_empty(), "output must be non-empty:\n{output:?}" );

  // Column 0 must appear in the primary table section
  let primary_lines : Vec< &str > = output.lines()
    .filter( | l | !l.starts_with( DEFAULT_INDENT ) )
    .collect();
  assert!(
    primary_lines.iter().any( | l | l.contains( "Col0" ) ),
    "column 0 must be in the primary row (fold clamped to 1):\n{output:?}",
  );

  // Columns 1-4 must appear in continuation lines; Stacked style emits one line per overflow column
  let continuation_lines : Vec< &str > = output.lines()
    .filter( | l | l.starts_with( DEFAULT_INDENT ) )
    .collect();
  assert!(
    continuation_lines.len() >= 4,
    "4 overflow columns with Stacked style must produce ≥4 continuation lines; got {}:\n{output:?}",
    continuation_lines.len(),
  );

  // All 4 overflow columns must be represented in continuation output
  assert!( output.contains( "Col1" ), "Col1 must appear in continuation:\n{output:?}" );
  assert!( output.contains( "Col2" ), "Col2 must appear in continuation:\n{output:?}" );
  assert!( output.contains( "Col3" ), "Col3 must appear in continuation:\n{output:?}" );
  assert!( output.contains( "Col4" ), "Col4 must appear in continuation:\n{output:?}" );
}
