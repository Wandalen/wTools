//! Acceptance criteria tests for column folding with auto-fold (Task 020)
//!
//! ## What This Tests
//!
//! Additional acceptance criteria (AC) tests verifying `FoldStyle` behavior,
//! fold point computation, continuation line structure, and edge cases not
//! covered by the primary TDD test matrix (T01-T25 in `auto_fold_test.rs`).
//!
//! See `docs/feature/005_auto_fit.md § Strategy 1 — Column Folding` for the
//! behavioral specification and `docs/algorithm/005_column_fold_detection.md`.
//!
//! ## Test Matrix
//!
//! CF AC-6: `FoldStyle::Bare` joins all overflow values on single continuation line
//! CF AC-7: rows with no overflow content produce no continuation lines
//! CF AC-8: single overflow column produces exactly one continuation line per row
//! CF AC-9 and beyond: additional acceptance criteria as implementation progresses

#![ cfg( feature = "enabled" ) ]
use data_fmt::{ RowBuilder, TableFormatter, TableConfig, ColumnFlex, FoldStyle, Format };

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
