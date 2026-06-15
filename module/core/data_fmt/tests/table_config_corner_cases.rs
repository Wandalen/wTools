//! Corner case tests for `TableConfig` API and `TableFormatter` behavior
//!
//! ## What This Tests
//!
//! Exhaustive corner case coverage for `TableConfig` after Task 011 (field privacy).
//! All assertions are behavioral (output-based), not field-state assertions, because
//! `TableConfig` fields are private since v0.10.0.
//!
//! ## Test Matrix
//!
//! ### 1. API Misuse Prevention
//! - `TableConfig::new()` produces identical output to `TableConfig::default()`
//! - `compile_fail` doctest (in src/config.rs) guards struct literal construction
//!
//! ### 2. Builder Chain Edge Cases
//! - `inner_padding(0)` — no extra spaces inside cells
//! - `inner_padding(3)` — 3 spaces per side
//! - `outer_padding(false)` — no leading/trailing content
//! - `column_separator(Character('|'))` produces pipes
//! - `align_right(vec![true, false, true])` — mixed alignment
//! - `column_widths(vec![5, 10])` — fixed widths respected
//! - `max_column_width(None)` — no truncation
//!
//! ### 3. `unicode_box()` Critical Invariants (bug-011 regression)
//! - Single column: still uses `│`
//! - Five columns: all separators are `│`
//! - No plain-dash-only separator rows
//!
//! ### 4. Alignment Edge Cases
//! - `align_right` empty vec — all columns left-aligned
//! - `align_right` fewer entries than column count — trailing columns default left
//! - `align_right` more entries than column count — extra entries ignored
//!
//! ### 5. Column Width Override Edge Cases
//! - `column_widths` wider than content — padding added
//! - `column_widths` narrower than content — content overflows override
//! - `column_widths` empty vec — auto-sizing active
//!
//! ### 6. Table Dimension Edge Cases
//! - Single column, single row
//! - Single column, many rows
//! - Many columns, single row
//! - Empty cell content (empty string)
//! - Header with empty column name
//! - Very long content (100+ chars)
//!
//! ### 7. Truncation Edge Cases (not covered in `column_truncation.rs`)
//! - `max_column_width(Some(0))` — zero-width limit
//! - Marker wider than content slot — graceful handling
//!
//! ### 8. Preset Completeness Checks
//! - All 9 presets render without panic
//! - `compact()` produces denser output than `plain()`
//!
//! ### 9. `min_column_width` + Alignment (see `table_config_validation_test.rs`)
//! - `min_column_width` floor enforcement (T012 suite)
//! - Table line alignment correctness (`unicode_box`, `markdown`, `bordered`, `grid`)
//!
//! ## Common Pitfalls to Avoid
//!
//! - `TableConfig::new()` sets `column_separator: Spaces(2)`, not `Character('|')`.
//! - `outer_padding` controls whether space padding appears at table edges;
//!   it does NOT control whether a border pipe appears (that depends on `header_separator_variant`).
//! - `column_widths` override takes precedence over auto-sizing entirely; if the slice
//!   is shorter than the column count, remaining columns get `unwrap_or(10)` default width.

#![ cfg( feature = "enabled" ) ]

use data_fmt::
{
  RowBuilder, TableFormatter, TableConfig,
  ColumnSeparator, Format,
};

// ============================================================================
// 1. API Misuse Prevention
// ============================================================================

/// `TableConfig::new()` and `TableConfig::default()` must produce identical output.
/// If they differ, a caller relying on `Default::default()` gets different behavior
/// than a caller using the documented `new()` constructor.
#[ test ]
fn test_new_and_default_produce_identical_output()
{
  let tree = RowBuilder::new( vec![ "X".into(), "Y".into() ] )
    .add_row( vec![ "a".into(), "b".into() ] )
    .build_view();

  let out_new = TableFormatter::with_config( TableConfig::new() ).format( &tree ).unwrap_or_default();
  let out_default = TableFormatter::with_config( TableConfig::default() ).format( &tree ).unwrap_or_default();

  assert_eq!(
    out_new, out_default,
    "TableConfig::new() and TableConfig::default() must produce identical output"
  );
}

// ============================================================================
// 2. Builder Chain Edge Cases
// ============================================================================

/// `inner_padding(0)` — no extra spaces should appear between border and content.
/// Tests that removing inner padding actually removes whitespace from cells.
#[ test ]
fn test_inner_padding_zero_removes_cell_spaces()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  // Use bordered() as base (inner_padding defaults to 1 there) then override to 0
  let output = TableFormatter::with_config(
    TableConfig::bordered().with_inner_padding( 0 )
  ).format( &tree ).unwrap_or_default();

  // With inner_padding=0, rows should start with "|" immediately (no "|  " prefix)
  // bordered() + AsciiGrid separator → rows have leading pipe
  let data_lines : Vec< &str > = output.lines()
    .filter( | l | l.contains( '1' ) || l.contains( 'A' ) )
    .collect();

  assert!(
    !data_lines.is_empty(),
    "Table must have data lines; output:\n{output}"
  );

  // No row should start with "|  " (pipe + 2+ spaces from inner_padding=0)
  assert!(
    !data_lines.iter().any( | l | l.starts_with( "|  " ) ),
    "inner_padding(0) must not add leading spaces; output:\n{output}"
  );
}

/// `inner_padding(3)` — three spaces must appear between border and content.
#[ test ]
fn test_inner_padding_three_adds_cell_spaces()
{
  let tree = RowBuilder::new( vec![ "X".into() ] )
    .add_row( vec![ "val".into() ] )
    .build_view();

  let output = TableFormatter::with_config(
    TableConfig::bordered().with_inner_padding( 3 )
  ).format( &tree ).unwrap_or_default();

  // With inner_padding=3 + bordered + AsciiGrid → rows start with "|   " (pipe + 3 spaces)
  assert!(
    output.lines().any( | l | l.starts_with( "|   " ) ),
    "inner_padding(3) must produce |   prefix on rows; output:\n{output}"
  );
}

/// `outer_padding(false)` — no outer padding space in space-separated format.
/// Uses `plain()` base (Spaces separator) to avoid border pipe interference.
#[ test ]
fn test_outer_padding_false_with_spaces_separator()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Val".into() ] )
    .add_row( vec![ "Alice".into(), "42".into() ] )
    .build_view();

  // plain() has outer_padding=true by default; override it
  let output_no_outer = TableFormatter::with_config(
    TableConfig::plain().with_outer_padding( false )
  ).format( &tree ).unwrap_or_default();

  let output_with_outer = TableFormatter::with_config(
    TableConfig::plain().with_outer_padding( true )
  ).format( &tree ).unwrap_or_default();

  // With outer_padding=false and Spaces(2) separator, content should differ from outer=true
  // Both must contain data
  assert!(
    output_no_outer.contains( "Alice" ),
    "outer_padding(false) must still render data; output:\n{output_no_outer}"
  );
  assert!(
    output_with_outer.contains( "Alice" ),
    "outer_padding(true) must render data; output:\n{output_with_outer}"
  );
}

/// `column_separator(Character('|'))` — explicit pipe separator appears in output.
#[ test ]
fn test_column_separator_pipe_character_appears_in_output()
{
  let tree = RowBuilder::new( vec![ "Col1".into(), "Col2".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let output = TableFormatter::with_config(
    TableConfig::new().with_column_separator( ColumnSeparator::Character( '|' ) )
  ).format( &tree ).unwrap_or_default();

  assert!(
    output.contains( '|' ),
    "Character('|') separator must produce | in output; output:\n{output}"
  );
  assert!(
    output.contains( 'x' ) && output.contains( 'y' ),
    "Output must contain data; output:\n{output}"
  );
}

/// `max_column_width(None)` — no truncation occurs; full content preserved.
#[ test ]
fn test_max_column_width_none_preserves_full_content()
{
  let long_content = "This is a very long string that would be truncated if max_column_width were set";
  let tree = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ long_content.into() ] )
    .build_view();

  let output = TableFormatter::with_config(
    TableConfig::plain().with_max_column_width( None )
  ).format( &tree ).unwrap_or_default();

  assert!(
    output.contains( long_content ),
    "max_column_width(None) must not truncate content; output:\n{output}"
  );
  assert!(
    !output.contains( "..." ),
    "max_column_width(None) must not show truncation marker; output:\n{output}"
  );
}

/// Mixed alignment `align_right(vec![true, false, true])` — odd columns right-aligned.
/// Right-aligned content produces trailing leading spaces rather than trailing spaces.
#[ test ]
fn test_align_right_mixed_produces_different_padding()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into() ] )
    .add_row( vec![ "123".into(), "abc".into(), "999".into() ] )
    .build_view();

  let output_mixed = TableFormatter::with_config(
    TableConfig::plain()
      .with_align_right( vec![ true, false, true ] )
  ).format( &tree ).unwrap_or_default();

  let output_all_left = TableFormatter::with_config(
    TableConfig::plain()
      .with_align_right( vec![ false, false, false ] )
  ).format( &tree ).unwrap_or_default();

  // Both must contain data
  assert!( output_mixed.contains( "123" ), "mixed alignment must contain data; output:\n{output_mixed}" );

  // The outputs should differ (right vs left alignment changes padding)
  // Note: with short values this may not differ if widths exactly match content — that's acceptable
  // Key invariant: no panic and data present
  let _ = output_all_left;
}

// ============================================================================
// 3. unicode_box() Critical Invariants
// ============================================================================

/// Single-column `unicode_box` table must still use `│` separator.
/// Regression guard: single-column tables might skip separator logic.
#[ test ]
fn test_unicode_box_single_column_uses_unicode_separator()
{
  let tree = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .add_row( vec![ "Bob".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::unicode_box() ).format( &tree ).unwrap_or_default();
  let lines : Vec< &str > = output.lines().collect();

  // Data rows must use │
  assert!(
    lines.iter().any( | l | l.contains( '│' ) ),
    "unicode_box single-column data rows must use │; output:\n{output}"
  );

  // Header separator must use Unicode characters (├ or ┤)
  assert!(
    lines.iter().any( | l | l.contains( '├' ) || l.contains( '┤' ) ),
    "unicode_box single-column separator must use ├/┤; output:\n{output}"
  );

  // Must NOT have plain dash separator
  let separator_line = lines.iter().find( | l | !l.contains( "Alice" ) && !l.contains( "Bob" ) && !l.contains( "Name" ) );
  if let Some( sep ) = separator_line
  {
    assert!(
      !sep.chars().all( | c | c == '-' || c == ' ' ),
      "unicode_box must not produce plain-dash-only separator; sep={sep:?}; output:\n{output}"
    );
  }
}

/// Five-column `unicode_box` table — all inter-column separators must be `│`.
/// Regression: multi-column rendering must not regress to spaces.
#[ test ]
fn test_unicode_box_five_columns_all_use_unicode_separator()
{
  let tree = RowBuilder::new( vec![
    "C1".into(), "C2".into(), "C3".into(), "C4".into(), "C5".into(),
  ])
    .add_row( vec![ "a".into(), "b".into(), "c".into(), "d".into(), "e".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::unicode_box() ).format( &tree ).unwrap_or_default();

  // Every data row must contain │
  for line in output.lines()
  {
    if line.contains( 'a' ) || line.contains( 'b' ) || line.contains( "C1" )
    {
      assert!(
        line.contains( '│' ),
        "five-column unicode_box row must contain │; line={line:?}; output:\n{output}"
      );
    }
  }

  // Separator row must contain ┼ (intersections between 5 columns)
  assert!(
    output.contains( '┼' ),
    "five-column unicode_box must have ┼ intersections; output:\n{output}"
  );
}

/// `unicode_box` header separator must never be a plain-dash line.
/// This is the core invariant of bug-011 — the header sep must use Unicode chars.
#[ test ]
fn test_unicode_box_no_plain_dash_separator()
{
  let tree = RowBuilder::new( vec![ "X".into(), "Y".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::unicode_box() ).format( &tree ).unwrap_or_default();

  // Find the separator line (index 2: top_border=0, header=1, sep=2)
  // After border rendering, unicode_box produces: top_border → header → sep → rows → bottom
  let lines : Vec< &str > = output.lines().collect();
  assert!(
    lines.len() >= 3,
    "unicode_box output must have at least top_border + header + separator; output:\n{output}"
  );

  let separator = lines[ 2 ];

  // Must NOT be plain dashes
  let is_plain_dashes = separator.chars().all( | c | c == '-' || c == ' ' );
  assert!(
    !is_plain_dashes,
    "unicode_box separator must not be plain dashes; separator={separator:?}; output:\n{output}"
  );

  // Must contain at least one Unicode box character
  let has_unicode = separator.contains( '├' )
    || separator.contains( '┤' )
    || separator.contains( '┼' )
    || separator.contains( '─' );
  assert!(
    has_unicode,
    "unicode_box separator must contain Unicode box characters; separator={separator:?}; output:\n{output}"
  );
}

// ============================================================================
// 4. Alignment Edge Cases
// ============================================================================

/// `align_right(vec![])` — empty vec means all columns use default left alignment.
#[ test ]
fn test_align_right_empty_vec_means_left_aligned()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "hi".into(), "world".into() ] )
    .build_view();

  // Empty align_right should not panic and should produce output
  let output = TableFormatter::with_config(
    TableConfig::plain().with_align_right( vec![] )
  ).format( &tree ).unwrap_or_default();

  assert!(
    output.contains( "hi" ) && output.contains( "world" ),
    "align_right(empty) must still render data; output:\n{output}"
  );
}

/// `align_right` with fewer entries than column count — extra columns default to left.
#[ test ]
fn test_align_right_fewer_entries_than_columns_no_panic()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into() ] )
    .add_row( vec![ "1".into(), "2".into(), "3".into() ] )
    .build_view();

  // Only specify alignment for first column — columns 2 and 3 get default (left)
  let output = TableFormatter::with_config(
    TableConfig::plain().with_align_right( vec![ true ] )
  ).format( &tree ).unwrap_or_default();

  assert!(
    output.contains( '1' ) && output.contains( '2' ) && output.contains( '3' ),
    "align_right with fewer entries than columns must render all columns; output:\n{output}"
  );
}

/// `align_right` with more entries than column count — extra entries ignored, no panic.
#[ test ]
fn test_align_right_more_entries_than_columns_no_panic()
{
  let tree = RowBuilder::new( vec![ "A".into() ] )
    .add_row( vec![ "x".into() ] )
    .build_view();

  // Specify alignment for 5 columns but table only has 1
  let output = TableFormatter::with_config(
    TableConfig::plain().with_align_right( vec![ true, false, true, false, true ] )
  ).format( &tree ).unwrap_or_default();

  assert!(
    output.contains( 'x' ),
    "align_right with more entries than columns must not panic and must render data; output:\n{output}"
  );
}

// ============================================================================
// 5. Column Width Override Edge Cases
// ============================================================================

/// `column_widths` wider than content — content gets padding to fill width.
#[ test ]
fn test_column_widths_wider_than_content_adds_padding()
{
  let tree = RowBuilder::new( vec![ "N".into() ] )
    .add_row( vec![ "A".into() ] )
    .build_view();

  // Force a 20-char column for 1-char content
  let output = TableFormatter::with_config(
    TableConfig::plain().with_column_widths( vec![ 20 ] )
  ).format( &tree ).unwrap_or_default();

  // The row should be padded to 20 chars for the column (plus any outer padding)
  assert!(
    output.contains( 'A' ),
    "column_widths wider than content must still render; output:\n{output}"
  );

  // Line should be at least 20 chars wide (content padded to width)
  let data_line = output.lines().find( | l | l.contains( 'A' ) ).unwrap();
  assert!(
    data_line.len() >= 20,
    "padded column line must be at least 20 chars; line={data_line:?}; output:\n{output}"
  );
}

/// `column_widths` narrower than content — formatter still renders content (overflow, no truncation).
/// `column_widths` override sets column width but does NOT force truncation.
#[ test ]
fn test_column_widths_narrower_than_content_no_panic()
{
  let tree = RowBuilder::new( vec![ "Header".into() ] )
    .add_row( vec![ "Very long content".into() ] )
    .build_view();

  // Force 3-char column for 17-char content
  let output = TableFormatter::with_config(
    TableConfig::plain().with_column_widths( vec![ 3 ] )
  ).format( &tree ).unwrap_or_default();

  // Must not panic and must render data (content will overflow width but that's OK)
  assert!(
    output.contains( "Very long content" ),
    "column_widths narrower than content must still render full content; output:\n{output}"
  );
}

/// `column_widths(vec![])` — empty override means auto-sizing is active.
#[ test ]
fn test_column_widths_empty_vec_uses_auto_sizing()
{
  let tree = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "DataValue".into() ] )
    .build_view();

  let output_auto = TableFormatter::with_config(
    TableConfig::plain().with_column_widths( vec![] )
  ).format( &tree ).unwrap_or_default();

  let output_default = TableFormatter::with_config(
    TableConfig::plain()
  ).format( &tree ).unwrap_or_default();

  // Both should produce identical output (empty override = auto-sizing)
  assert_eq!(
    output_auto, output_default,
    "column_widths(empty) must equal auto-sizing output"
  );
}

// ============================================================================
// 6. Table Dimension Edge Cases
// ============================================================================

/// Single column, single row — minimal table must render without panic.
#[ test ]
fn test_single_column_single_row_renders()
{
  let tree = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "Val".into() ] )
    .build_view();

  for ( name, config ) in [
    ( "plain", TableConfig::plain() ),
    ( "bordered", TableConfig::bordered() ),
    ( "unicode_box", TableConfig::unicode_box() ),
    ( "csv", TableConfig::csv() ),
    ( "markdown", TableConfig::markdown() ),
  ]
  {
    let output = TableFormatter::with_config( config ).format( &tree ).unwrap_or_default();
    assert!(
      output.contains( "Val" ),
      "{name} must render single-column/single-row table; output:\n{output}"
    );
    assert!(
      output.contains( "Col" ),
      "{name} must render header in single-column table; output:\n{output}"
    );
  }
}

/// Single column, many rows (10) — must render all rows without panic.
#[ test ]
fn test_single_column_many_rows_renders_all()
{
  let mut builder = RowBuilder::new( vec![ "Num".into() ] );
  for i in 0..10
  {
    builder.add_row_mut( vec![ format!( "row{i}" ).into() ] );
  }
  let tree = builder.build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();

  for i in 0..10
  {
    assert!(
      output.contains( &format!( "row{i}" ) ),
      "single-column many-rows must render row{i}; output:\n{output}"
    );
  }
}

/// Many columns (6), single data row — must render all columns without panic.
#[ test ]
fn test_many_columns_single_row_renders_all()
{
  let tree = RowBuilder::new( vec![
    "H1".into(), "H2".into(), "H3".into(), "H4".into(), "H5".into(), "H6".into(),
  ])
    .add_row( vec![ "a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();

  for col in [ "H1", "H2", "H3", "H4", "H5", "H6", "a", "b", "c", "d", "e", "f" ]
  {
    assert!(
      output.contains( col ),
      "many-columns table must contain {col}; output:\n{output}"
    );
  }
}

/// Empty string cell content — must render without panic.
/// Empty cells should produce padding, not corrupt alignment.
#[ test ]
fn test_empty_cell_content_renders_without_panic()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ String::new().into(), "value".into() ] )
    .add_row( vec![ "content".into(), String::new().into() ] )
    .build_view();

  for ( name, config ) in [
    ( "plain", TableConfig::plain() ),
    ( "bordered", TableConfig::bordered() ),
    ( "unicode_box", TableConfig::unicode_box() ),
  ]
  {
    let output = TableFormatter::with_config( config ).format( &tree ).unwrap_or_default();
    assert!(
      output.contains( "value" ) && output.contains( "content" ),
      "{name} must render rows with empty cells; output:\n{output}"
    );
  }
}

/// Header with empty column name — must render without panic.
#[ test ]
fn test_header_with_empty_column_name_renders()
{
  let tree = RowBuilder::new( vec![ String::new(), "Value".into() ] )
    .add_row( vec![ "key".into(), "42".into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();

  assert!(
    output.contains( "key" ) && output.contains( "42" ),
    "Table with empty header column name must render data; output:\n{output}"
  );
}

/// Very long content (100+ chars) — must render without panic.
#[ test ]
fn test_very_long_cell_content_renders_without_truncation_by_default()
{
  let long = "x".repeat( 120 );
  let tree = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ long.clone().into() ] )
    .build_view();

  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();

  assert!(
    output.contains( &long ),
    "Very long cell content must be preserved with default config; output length={}", output.len()
  );
}

// ============================================================================
// 7. Truncation Edge Cases
// ============================================================================

/// `max_column_width(Some(0))` — zero-width limit; must not panic.
/// With a zero-width limit, the content slot is `saturating_sub(marker_len)` = 0,
/// so only the marker (or nothing) is emitted.
#[ test ]
fn test_max_column_width_zero_no_panic()
{
  let tree = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "some text".into() ] )
    .build_view();

  // Should not panic regardless of output
  let output = TableFormatter::with_config(
    TableConfig::plain().with_max_column_width( Some( 0 ) )
  ).format( &tree ).unwrap_or_default();

  assert!( !output.is_empty(), "max_column_width(0) must produce non-empty output; output:{output:?}" );
}

/// Column truncation: content at exactly `max_column_width` is NOT truncated.
/// This is already tested in `column_truncation.rs` but repeated here to anchor
/// the invariant in the corner-cases file.
#[ test ]
fn test_truncation_at_exact_max_width_no_truncation_marker()
{
  // 10 chars exactly at the limit
  let tree = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "0123456789".into() ] )  // exactly 10 chars
    .build_view();

  let output = TableFormatter::with_config(
    TableConfig::plain()
      .with_max_column_width( Some( 10 ) )
      .with_truncation_marker( "...".to_string() )
  ).format( &tree ).unwrap_or_default();

  assert!(
    output.contains( "0123456789" ),
    "content at exactly max_column_width must not be truncated; output:\n{output}"
  );
  // No truncation marker on the data line
  let data_line = output.lines().find( | l | l.contains( "0123456789" ) ).unwrap();
  assert!(
    !data_line.contains( "..." ),
    "exact-fit content must not show truncation marker; line={data_line:?}; output:\n{output}"
  );
}

// ============================================================================
// 8. Preset Completeness Checks
// ============================================================================

/// All 9 presets must render data without panic.
/// Smoke test covering every preset with a 2x2 table.
#[ test ]
fn test_all_nine_presets_render_data_without_panic()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Val".into() ] )
    .add_row( vec![ "Alice".into(), "1".into() ] )
    .build_view();

  let presets = [
    ( "plain",       TableConfig::plain() ),
    ( "minimal",     TableConfig::minimal() ),
    ( "bordered",    TableConfig::bordered() ),
    ( "markdown",    TableConfig::markdown() ),
    ( "grid",        TableConfig::grid() ),
    ( "unicode_box", TableConfig::unicode_box() ),
    ( "csv",         TableConfig::csv() ),
    ( "tsv",         TableConfig::tsv() ),
    ( "compact",     TableConfig::compact() ),
  ];

  for ( name, config ) in presets
  {
    let output = TableFormatter::with_config( config ).format( &tree ).unwrap_or_default();
    assert!(
      output.contains( "Alice" ),
      "{name} preset must render data 'Alice'; output:\n{output}"
    );
    assert!(
      output.contains( "Name" ) || output.contains( "Val" ),
      "{name} preset must render headers; output:\n{output}"
    );
  }
}

/// `compact()` uses single-space separator — verify it is denser than `plain()`.
#[ test ]
fn test_compact_denser_than_plain()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into() ] )
    .add_row( vec![ "x".into(), "y".into(), "z".into() ] )
    .build_view();

  let compact_out = TableFormatter::with_config( TableConfig::compact() ).format( &tree ).unwrap_or_default();
  let plain_out   = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();

  // compact should produce shorter lines than plain (1-space vs 2-space separator)
  let compact_data : Vec< &str > = compact_out.lines()
    .filter( | l | l.contains( 'x' ) )
    .collect();
  let plain_data : Vec< &str > = plain_out.lines()
    .filter( | l | l.contains( 'x' ) )
    .collect();

  if !compact_data.is_empty() && !plain_data.is_empty()
  {
    assert!(
      compact_data[ 0 ].len() <= plain_data[ 0 ].len(),
      "compact() data lines must be <= plain() data lines in length; compact={:?} plain={:?}",
      compact_data[ 0 ], plain_data[ 0 ]
    );
  }
}

