//! Sub-row detail lines — integration tests
//!
//! Each test verifies exactly one aspect of the sub-row detail feature.
//! Categories: API surface, rendering, backward compatibility, config interaction,
//! and Algorithm 3 (per-line ANSI color wrapping) corner cases.
//!
//! ## Test Matrix
//!
//! ### API Surface
//! | ID  | Aspect |
//! |-----|--------|
//! | T01 | `add_row` (no detail) — no detail line produced |
//! | T02 | `add_row_with_detail(Some("text"))` — detail line emitted |
//! | T03 | `add_row_with_detail(None)` — no extra line |
//! | T04 | `add_row_with_detail(Some(""))` — empty detail suppressed |
//! | T05 | Mixed rows: first has detail, second does not |
//! | T06 | Both rows have detail |
//! | T14 | `Format::format` trait path works with sub-row |
//! | T15 | Mutable API (`add_row_mut` / `add_row_with_detail_mut`) intermixed |
//! | T16 | `build_view()` produces `row_details` vector parallel to rows |
//! | T17 | `TableView::new` (old two-arg form) backward compat — `row_details` empty |
//! | T18 | Single-row table with detail |
//!
//! ### Config Interaction
//! | ID  | Aspect |
//! |-----|--------|
//! | T07 | Custom indent `">>> "` |
//! | T08 | Empty indent `""` — detail flush-left |
//! | T09 | Detail does not affect column widths |
//! | T10 | `AsciiGrid` style with multiple detail rows |
//! | T11 | Unicode box style with detail |
//! | T12 | Bordered style with detail |
//! | T25 | CSV format includes detail lines |
//! | T26 | Markdown format — detail without pipes |
//!
//! ### Rendering
//! | ID  | Aspect |
//! |-----|--------|
//! | T13 | Multiline cell combined with detail |
//! | T19 | Multiline detail — every sub-line gets indent |
//! | T20 | Detail NOT colored by alternating-row ANSI |
//! | T21 | Grid bottom border — detail appears before it |
//! | T27 | Multiline cell + alternating color + detail (triple interaction) |
//!
//! ### Boundary & Edge Cases
//! | ID  | Aspect |
//! |-----|--------|
//! | T22 | `row_details` shorter than rows — graceful (only paired rows get details) |
//! | T23 | Whitespace-only detail rendered (not suppressed) |
//! | T24 | `row_details` longer than rows — extra details silently ignored |
//! | T28 | Zero rows, empty detail vector — header-only, no panic |
//!
//! ### Algorithm 3 — Per-Line ANSI Color Wrapping
//! | ID  | Aspect |
//! |-----|--------|
//! | T29 | `DecoratedText` single-line colored detail has ANSI color + reset |
//! | T30 | Multiline colored detail: each sub-line gets independent color+reset |
//! | T31 | Custom indent + colored detail: indent precedes ANSI code |
//! | T32 | Detail text with trailing `\n` — same output as without (Rust `lines()` strips it) |
//! | T33 | Detail text `"\n"` only — `is_empty()` false; renders one blank indented line |

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format, TableView, TableMetadata, DecoratedText };

// =============================================================================
// Helper
// =============================================================================

fn plain_output( view : &TableView ) -> String
{
  let fmt = TableFormatter::with_config( TableConfig::plain() );
  Format::format( &fmt, view ).unwrap()
}

// =============================================================================
// T01 — add_row (no detail) produces no detail line
// =============================================================================

#[ test ]
fn t01_add_row_no_detail()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ DecoratedText::from( "Alice" ) ] )
    .build_view();

  let out = plain_output( &view );
  let lines : Vec< &str > = out.lines().collect();

  // header + separator + data row = 3 lines
  assert_eq!( lines.len(), 3, "expected exactly 3 lines, got:\n{out}" );
  assert!( lines[ 2 ].contains( "Alice" ), "expected \"Alice\" in row 2; got:\n{}", lines[ 2 ] );
}

// =============================================================================
// T02 — add_row_with_detail(Some("text")) emits detail line
// =============================================================================

#[ test ]
fn t02_detail_some_text()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "extra info" ) ) )
    .build_view();

  let out = plain_output( &view );
  let lines : Vec< &str > = out.lines().collect();

  // header + separator + data row + detail = 4 lines
  assert_eq!( lines.len(), 4, "expected 4 lines, got:\n{out}" );
  assert_eq!( lines[ 3 ], "  extra info" );
}

// =============================================================================
// T03 — add_row_with_detail(None) produces no extra line
// =============================================================================

#[ test ]
fn t03_detail_none_produces_no_line()
{
  let view_detail = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], None )
    .build_view();

  let view_plain = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ DecoratedText::from( "Alice" ) ] )
    .build_view();

  let out_detail = plain_output( &view_detail );
  let out_plain = plain_output( &view_plain );

  assert_eq!( out_detail, out_plain, "None detail must produce identical output to add_row" );
}

// =============================================================================
// T04 — add_row_with_detail(Some("")) suppresses empty detail
// =============================================================================

#[ test ]
fn t04_empty_string_detail_suppressed()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "" ) ) )
    .build_view();

  let out = plain_output( &view );
  let lines : Vec< &str > = out.lines().collect();

  assert_eq!( lines.len(), 3, "empty detail must be suppressed, got:\n{out}" );
}

// =============================================================================
// T05 — mixed: first row has detail, second does not
// =============================================================================

#[ test ]
fn t05_mixed_detail_first_only()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "detail-a" ) ) )
    .add_row( vec![ DecoratedText::from( "Bob" ) ] )
    .build_view();

  let out = plain_output( &view );

  assert!( out.contains( "  detail-a" ), "first row detail missing" );

  // Count detail lines (lines starting with 2-space indent that aren't header/data)
  let detail_count = out.lines()
    .filter( | l | l.starts_with( "  " ) && !l.contains( "Name" ) && !l.contains( "Alice" ) && !l.contains( "Bob" ) )
    .count();
  assert_eq!( detail_count, 1, "only first row should have detail" );
}

// =============================================================================
// T06 — both rows have detail
// =============================================================================

#[ test ]
fn t06_both_rows_have_detail()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "detail-a" ) ) )
    .add_row_with_detail( vec![ DecoratedText::from( "Bob" ) ], Some( DecoratedText::from( "detail-b" ) ) )
    .build_view();

  let out = plain_output( &view );

  assert!( out.contains( "  detail-a" ), "expected \"  detail-a\" in output:\n{out}" );
  assert!( out.contains( "  detail-b" ), "expected \"  detail-b\" in output:\n{out}" );

  // detail-a must appear before detail-b
  let pos_a = out.find( "detail-a" ).unwrap();
  let pos_b = out.find( "detail-b" ).unwrap();
  assert!( pos_a < pos_b, "detail-a must appear before detail-b" );
}

// =============================================================================
// T07 — custom indent ">>> "
// =============================================================================

#[ test ]
fn t07_custom_indent()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "note" ) ) )
    .build_view();

  let cfg = TableConfig::plain().with_sub_row_indent( ">>> ".to_string() );
  let fmt = TableFormatter::with_config( cfg );
  let out = Format::format( &fmt, &view ).unwrap();

  assert!( out.contains( ">>> note" ), "custom indent not applied, got:\n{out}" );
  assert!( !out.contains( "  note" ), "default indent should not appear" );
}

// =============================================================================
// T08 — empty indent "" (flush-left)
// =============================================================================

#[ test ]
fn t08_empty_indent_flush_left()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "flush" ) ) )
    .build_view();

  let cfg = TableConfig::plain().with_sub_row_indent( String::new() );
  let fmt = TableFormatter::with_config( cfg );
  let out = Format::format( &fmt, &view ).unwrap();

  let detail_line = out.lines().last().unwrap();
  assert_eq!( detail_line, "flush", "detail should be flush-left with empty indent" );
}

// =============================================================================
// T09 — detail does not affect column widths
// =============================================================================

#[ test ]
fn t09_sub_row_does_not_affect_column_widths()
{
  let view_no_detail = RowBuilder::new( vec![ "A".into() ] )
    .add_row( vec![ DecoratedText::from( "x" ) ] )
    .build_view();

  let view_with_detail = RowBuilder::new( vec![ "A".into() ] )
    .add_row_with_detail(
      vec![ DecoratedText::from( "x" ) ],
      Some( DecoratedText::from( "this is a very long detail that should not widen the column" ) ),
    )
    .build_view();

  let out_no = plain_output( &view_no_detail );
  let out_yes = plain_output( &view_with_detail );

  // First line (header) should be identical width
  let header_no = out_no.lines().next().unwrap();
  let header_yes = out_yes.lines().next().unwrap();
  assert_eq!( header_no, header_yes, "column width must not change due to detail text" );
}

// =============================================================================
// T10 — AsciiGrid style with three detail rows
// =============================================================================

#[ test ]
fn t10_ascii_grid_three_details()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Val".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "A" ), DecoratedText::from( "1" ) ], Some( DecoratedText::from( "d1" ) ) )
    .add_row_with_detail( vec![ DecoratedText::from( "B" ), DecoratedText::from( "2" ) ], Some( DecoratedText::from( "d2" ) ) )
    .add_row_with_detail( vec![ DecoratedText::from( "C" ), DecoratedText::from( "3" ) ], Some( DecoratedText::from( "d3" ) ) )
    .build_view();

  let cfg = TableConfig::bordered();
  let fmt = TableFormatter::with_config( cfg );
  let out = Format::format( &fmt, &view ).unwrap();

  assert!( out.contains( "  d1" ), "d1 missing in bordered output:\n{out}" );
  assert!( out.contains( "  d2" ), "d2 missing" );
  assert!( out.contains( "  d3" ), "d3 missing" );
}

// =============================================================================
// T11 — Unicode box style with sub-row
// =============================================================================

#[ test ]
fn t11_unicode_box_with_detail()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "unicode detail" ) ) )
    .build_view();

  let cfg = TableConfig::unicode_box();
  let fmt = TableFormatter::with_config( cfg );
  let out = Format::format( &fmt, &view ).unwrap();

  assert!( out.contains( "unicode detail" ), "detail missing in unicode output:\n{out}" );
  // Detail line should NOT have unicode border pipes
  let detail_line = out.lines()
    .find( | l | l.contains( "unicode detail" ) )
    .unwrap();
  assert!( !detail_line.contains( '│' ), "detail line must not have border pipes" );
}

// =============================================================================
// T12 — bordered() style with sub-row
// =============================================================================

#[ test ]
fn t12_bordered_with_detail()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "bordered detail" ) ) )
    .build_view();

  let cfg = TableConfig::bordered();
  let fmt = TableFormatter::with_config( cfg );
  let out = Format::format( &fmt, &view ).unwrap();

  assert!( out.contains( "bordered detail" ), "detail missing:\n{out}" );
  let detail_line = out.lines()
    .find( | l | l.contains( "bordered detail" ) )
    .unwrap();
  assert!( !detail_line.contains( '|' ), "detail line must not have border pipes" );
}

// =============================================================================
// T13 — Multiline cell + detail
// =============================================================================

#[ test ]
fn t13_multiline_cell_with_detail()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "line1\nline2" ) ], Some( DecoratedText::from( "after-multi" ) ) )
    .build_view();

  let out = plain_output( &view );

  // Detail must appear AFTER all multiline cell lines
  let pos_line2 = out.find( "line2" ).unwrap();
  let pos_detail = out.find( "after-multi" ).unwrap();
  assert!(
    pos_detail > pos_line2,
    "detail must appear after all multiline lines, got:\n{out}"
  );
}

// =============================================================================
// T14 — Format::format(&formatter, &view) path
// =============================================================================

#[ test ]
fn t14_format_trait_path()
{
  let view = TableView::with_details(
    TableMetadata::new( vec![ "Col".to_string() ] ),
    vec![ vec![ DecoratedText::from( "val" ) ] ],
    vec![ Some( DecoratedText::from( "trait-path detail" ) ) ],
  );

  let fmt = TableFormatter::with_config( TableConfig::plain() );
  let out = Format::format( &fmt, &view ).unwrap();
  assert!( out.contains( "  trait-path detail" ), "detail missing via Format trait:\n{out}" );
}

// =============================================================================
// T15 — add_row_mut + add_row_with_detail_mut intermixed
// =============================================================================

#[ test ]
fn t15_mut_api_intermixed()
{
  let mut builder = RowBuilder::new( vec![ "Name".into() ] );
  builder.add_row_mut( vec![ DecoratedText::from( "Alice" ) ] );
  builder.add_row_with_detail_mut( vec![ DecoratedText::from( "Bob" ) ], Some( DecoratedText::from( "mut-detail" ) ) );
  builder.add_row_mut( vec![ DecoratedText::from( "Carol" ) ] );

  let view = builder.build_view();
  let out = plain_output( &view );

  assert!( out.contains( "  mut-detail" ), "mut detail missing:\n{out}" );

  // Only one detail line
  let detail_count = out.lines()
    .filter( | l | l.starts_with( "  " ) && l.contains( "mut-detail" ) )
    .count();
  assert_eq!( detail_count, 1 );
}

// =============================================================================
// T16 — build_view() produces correct row_details vector
// =============================================================================

#[ test ]
fn t16_build_view_row_details_vector()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ DecoratedText::from( "A" ) ] )
    .add_row_with_detail( vec![ DecoratedText::from( "B" ) ], Some( DecoratedText::from( "detail-b" ) ) )
    .add_row_with_detail( vec![ DecoratedText::from( "C" ) ], None )
    .build_view();

  assert_eq!( view.row_details.len(), 3, "row_details must parallel rows" );
  assert_eq!( view.row_details[ 0 ], None );
  assert_eq!( view.row_details[ 1 ], Some( DecoratedText::from( "detail-b" ) ) );
  assert_eq!( view.row_details[ 2 ], None );
}

// =============================================================================
// T17 — TableView::new (old two-arg form) backward compat
// =============================================================================

#[ test ]
fn t17_table_view_new_backward_compat()
{
  let view = TableView::new(
    TableMetadata::new( vec![ "X".to_string() ] ),
    vec![ vec![ DecoratedText::from( "1" ) ] ],
  );

  // row_details should be empty
  assert!( view.row_details.is_empty(), "expected empty row_details for old-form TableView" );

  // Must render without sub-rows
  let out = plain_output( &view );
  let lines : Vec< &str > = out.lines().collect();
  assert_eq!( lines.len(), 3, "no detail lines for old-form TableView" );
}

// =============================================================================
// T18 — single row table with detail
// =============================================================================

#[ test ]
fn t18_single_row_with_detail()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ), DecoratedText::from( "30" ) ], Some( DecoratedText::from( "only row detail" ) ) )
    .build_view();

  let out = plain_output( &view );

  // Exactly one detail line
  let detail_lines : Vec< &str > = out.lines()
    .filter( | l | l.contains( "only row detail" ) )
    .collect();
  assert_eq!( detail_lines.len(), 1, "expected exactly one detail line" );
  assert_eq!( detail_lines[ 0 ], "  only row detail" );
}

// =============================================================================
// T19 — multi-line detail: every line gets indent prefix
// =============================================================================

/// ## Root Cause
///
/// Single `push_str( indent ) + push_str( detail )` emitted the indent only
/// before the entire string; continuation lines started at column 0 with no prefix.
///
/// ## Why Not Caught
///
/// All prior sub-row detail tests used single-line strings. The per-line indent
/// invariant was never exercised on multi-line detail content.
///
/// ## Fix Applied
///
/// The sub-row rendering loop in `src/formatters/table/mod.rs` was refactored to
/// iterate `ct.text.lines()` and emit `indent + line + '\n'` for every line.
///
/// ## Prevention
///
/// Any code that emits user-provided text with a per-line prefix must iterate
/// `.lines()` — a single `push_str` cannot apply per-line prefixes correctly.
///
/// ## Pitfall
///
/// Same pattern as colored row rendering (BUG-009/BUG-010): only `.lines()`
/// iteration guarantees per-line prefix application for embedded-newline content.
// test_kind: bug_reproducer(BUG-013)
#[ test ]
fn t19_multiline_detail_all_lines_indented()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "line1\nline2\nline3" ) ) )
    .build_view();

  let out = plain_output( &view );

  let detail_lines : Vec< &str > = out.lines()
    .filter( | l | l.contains( "line1" ) || l.contains( "line2" ) || l.contains( "line3" ) )
    .collect();

  assert_eq!( detail_lines.len(), 3, "expected 3 detail lines, got:\n{out}" );
  assert_eq!( detail_lines[ 0 ], "  line1" );
  assert_eq!( detail_lines[ 1 ], "  line2" );
  assert_eq!( detail_lines[ 2 ], "  line3" );
}

// =============================================================================
// T20 — alternating row colors: detail lines must NOT be colored
// =============================================================================

#[ test ]
fn t20_detail_not_colored_with_alternating_rows()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "uncolored-detail" ) ) )
    .add_row_with_detail( vec![ DecoratedText::from( "Bob" ) ], Some( DecoratedText::from( "uncolored-detail-2" ) ) )
    .build_view();

  let cfg = TableConfig::plain()
    .with_alternating_rows( true )
    .with_row_colors( "\x1b[41m".to_string(), "\x1b[42m".to_string() );
  let fmt = TableFormatter::with_config( cfg );
  let out = Format::format( &fmt, &view ).unwrap();

  for line in out.lines()
  {
    if line.contains( "uncolored-detail" )
    {
      assert!(
        !line.contains( "\x1b[" ),
        "detail line must not contain ANSI escape codes: {line:?}"
      );
    }
  }
}

// =============================================================================
// T21 — bottom border (grid): detail appears before bottom border
// =============================================================================

#[ test ]
fn t21_grid_detail_before_bottom_border()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ DecoratedText::from( "Alice" ) ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Bob" ) ], Some( DecoratedText::from( "last-detail" ) ) )
    .build_view();

  let cfg = TableConfig::grid();
  let fmt = TableFormatter::with_config( cfg );
  let out = Format::format( &fmt, &view ).unwrap();

  let pos_detail = out.find( "last-detail" ).expect( "detail missing in grid output" );
  let pos_bottom = out.rfind( '+' ).expect( "bottom border missing" );
  assert!( pos_detail < pos_bottom, "detail must appear before bottom border:\n{out}" );
}

// =============================================================================
// T22 — row_details shorter than rows: graceful handling
// =============================================================================

#[ test ]
fn t22_details_shorter_than_rows()
{
  let view = TableView::with_details(
    TableMetadata::new( vec![ "X".to_string() ] ),
    vec![
      vec![ DecoratedText::from( "A" ) ],
      vec![ DecoratedText::from( "B" ) ],
      vec![ DecoratedText::from( "C" ) ],
    ],
    vec![ Some( DecoratedText::from( "only-first" ) ) ],
  );

  let out = plain_output( &view );

  assert!( out.contains( "  only-first" ), "first row detail missing" );
  let detail_count = out.lines()
    .filter( | l | l.starts_with( "  " ) && !l.contains( 'X' ) && !l.contains( 'A' ) && !l.contains( 'B' ) && !l.contains( 'C' ) )
    .count();
  assert_eq!( detail_count, 1, "only row 0 should have detail, got:\n{out}" );
}

// =============================================================================
// T23 — whitespace-only detail is rendered (not suppressed)
// =============================================================================

#[ test ]
fn t23_whitespace_only_detail_rendered()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "   " ) ) )
    .build_view();

  let out = plain_output( &view );
  let lines : Vec< &str > = out.lines().collect();

  // header + separator + data row + detail = 4 lines
  // whitespace-only is NOT empty, so it should render
  assert_eq!( lines.len(), 4, "whitespace-only detail must render, got:\n{out}" );
  assert_eq!( lines[ 3 ], "     ", "indent(2) + 3 spaces" );
}

// =============================================================================
// T24 — row_details longer than rows: extra details silently ignored
// =============================================================================

#[ test ]
fn t24_details_longer_than_rows()
{
  let view = TableView::with_details(
    TableMetadata::new( vec![ "X".to_string() ] ),
    vec![ vec![ DecoratedText::from( "A" ) ] ],
    vec![
      Some( DecoratedText::from( "d0" ) ),
      Some( DecoratedText::from( "d1-orphan" ) ),
      Some( DecoratedText::from( "d2-orphan" ) ),
    ],
  );

  let out = plain_output( &view );

  assert!( out.contains( "  d0" ), "first detail missing" );
  assert!( !out.contains( "d1-orphan" ), "orphan detail d1 must not appear" );
  assert!( !out.contains( "d2-orphan" ), "orphan detail d2 must not appear" );
}

// =============================================================================
// T25 — CSV format includes detail lines
// =============================================================================

#[ test ]
fn t25_csv_with_detail()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Val".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ), DecoratedText::from( "1" ) ], Some( DecoratedText::from( "csv-detail" ) ) )
    .build_view();

  let cfg = TableConfig::csv();
  let fmt = TableFormatter::with_config( cfg );
  let out = Format::format( &fmt, &view ).unwrap();

  assert!( out.contains( "csv-detail" ), "detail missing in csv output:\n{out}" );
}

// =============================================================================
// T26 — Markdown format includes detail lines
// =============================================================================

#[ test ]
fn t26_markdown_with_detail()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "md-detail" ) ) )
    .build_view();

  let cfg = TableConfig::markdown();
  let fmt = TableFormatter::with_config( cfg );
  let out = Format::format( &fmt, &view ).unwrap();

  assert!( out.contains( "md-detail" ), "detail missing in markdown output:\n{out}" );
  // Detail line should NOT have markdown pipes
  let detail_line = out.lines()
    .find( | l | l.contains( "md-detail" ) )
    .unwrap();
  assert!( !detail_line.contains( '|' ), "detail line must not have markdown pipes" );
}

// =============================================================================
// T27 — multiline cell + alternating color + detail triple interaction
// =============================================================================

#[ test ]
fn t27_multiline_color_detail_triple()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail(
      vec![ DecoratedText::from( "line1\nline2" ) ],
      Some( DecoratedText::from( "triple-detail" ) ),
    )
    .build_view();

  let cfg = TableConfig::plain()
    .with_alternating_rows( true )
    .with_row_colors( "\x1b[41m".to_string(), "\x1b[42m".to_string() );
  let fmt = TableFormatter::with_config( cfg );
  let out = Format::format( &fmt, &view ).unwrap();

  // Detail must appear after multiline cell content
  let pos_line2 = out.find( "line2" ).unwrap();
  let pos_detail = out.find( "triple-detail" ).unwrap();
  assert!( pos_detail > pos_line2, "detail must follow multiline content:\n{out}" );

  // Detail line must NOT contain ANSI escape codes
  let detail_line = out.lines()
    .find( | l | l.contains( "triple-detail" ) )
    .unwrap();
  assert!(
    !detail_line.contains( "\x1b[" ),
    "detail line must not be colored in triple interaction: {detail_line:?}"
  );
}

// =============================================================================
// T28 — zero rows: header-only table with empty detail vector
// =============================================================================

#[ test ]
fn t28_zero_rows_empty_details()
{
  let view = TableView::with_details(
    TableMetadata::new( vec![ "X".to_string() ] ),
    vec![],
    vec![],
  );

  let out = plain_output( &view );

  // Should render header only (no panic, no detail lines)
  assert!( out.contains( 'X' ), "header missing" );
  // No detail content
  let line_count = out.lines().count();
  assert!( line_count <= 2, "zero-row table should have at most header + separator, got:\n{out}" );
}

// =============================================================================
// T29 — colored DecoratedText detail renders with ANSI escape codes
// =============================================================================

#[ test ]
fn t29_colored_detail_ansi_codes()
{
  let yellow = "\x1b[33m";
  let ct = DecoratedText::from( "colored-detail" ).with_color( yellow );
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( ct ) )
    .build_view();

  let out = plain_output( &view );

  assert!( out.contains( "colored-detail" ), "detail text missing:\n{out}" );
  let detail_line = out.lines()
    .find( | l | l.contains( "colored-detail" ) )
    .unwrap();
  assert!(
    detail_line.contains( yellow ),
    "detail line must contain ANSI color code: {detail_line:?}",
  );
  assert!(
    detail_line.contains( "\x1b[0m" ),
    "detail line must contain ANSI reset: {detail_line:?}",
  );
}

// =============================================================================
// T30 — colored multiline detail: each sub-line gets its own ANSI wrap
// =============================================================================

/// ## Root Cause
///
/// The sub-row detail rendering called `ct.render()` on the whole multiline
/// colored string, producing `color + "line_a\nline_b" + RESET`. The `'\n'`
/// sits inside the color sequence; the next terminal line sees itself as still
/// inside the color context, causing background-color bleed.
///
/// ## Why Not Caught
///
/// All prior colored detail tests used single-line strings. The per-line
/// RESET invariant was never exercised on multi-line colored detail content.
///
/// ## Fix Applied
///
/// The sub-row rendering loop in `src/formatters/table/mod.rs` was refactored
/// to iterate `ct.text.lines()` and emit `color + line + RESET + '\n'` per
/// output line, ensuring RESET appears before every line boundary.
///
/// ## Prevention
///
/// Any code emitting colored user-provided text must iterate `.lines()` and
/// apply `color + line + RESET` per output line — never wrap the whole
/// multi-line string with a single trailing RESET.
///
/// ## Pitfall
///
/// `ct.render()` places RESET at the very end. Splitting the rendered result
/// by `'\n'` leaves intermediate lines without RESET. Always iterate
/// `.text.lines()` and emit color/RESET per line independently.
// test_kind: bug_reproducer(BUG-010)
#[ test ]
fn t30_colored_multiline_detail_per_line_ansi()
{
  let yellow = "\x1b[33m";
  let reset = "\x1b[0m";
  let ct = DecoratedText::from( "alpha\nbeta\ngamma" ).with_color( yellow );
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( ct ) )
    .build_view();

  let out = plain_output( &view );
  let lines : Vec< &str > = out.lines().collect();

  // Each sub-line must be independently wrapped
  let alpha_line = lines.iter().find( | l | l.contains( "alpha" ) ).expect( "alpha line missing" );
  let beta_line  = lines.iter().find( | l | l.contains( "beta" ) ).expect( "beta line missing" );
  let gamma_line = lines.iter().find( | l | l.contains( "gamma" ) ).expect( "gamma line missing" );

  assert!( alpha_line.contains( yellow ), "alpha line must have ANSI color: {alpha_line:?}" );
  assert!( alpha_line.contains( reset ), "alpha line must have ANSI reset: {alpha_line:?}" );
  assert!( beta_line.contains( yellow ), "beta line must have ANSI color: {beta_line:?}" );
  assert!( beta_line.contains( reset ), "beta line must have ANSI reset: {beta_line:?}" );
  assert!( gamma_line.contains( yellow ), "gamma line must have ANSI color: {gamma_line:?}" );
  assert!( gamma_line.contains( reset ), "gamma line must have ANSI reset: {gamma_line:?}" );

  // Exactly 3 resets — one per sub-line, not one for the whole block
  let reset_count = out.matches( reset ).count();
  assert_eq!(
    reset_count, 3,
    "expected exactly 3 ANSI resets (one per detail sub-line), got {reset_count} in:\n{out}",
  );
}

// =============================================================================
// T31 — custom indent + colored detail: indent precedes ANSI code on each line
// =============================================================================

#[ test ]
fn t31_custom_indent_with_colored_detail()
{
  let yellow = "\x1b[33m";
  let indent = ">>> ";
  let ct = DecoratedText::from( "note" ).with_color( yellow );
  let config = TableConfig::plain().with_sub_row_indent( indent.to_string() );
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( ct ) )
    .build_view();

  let fmt = TableFormatter::with_config( config );
  let out = Format::format( &fmt, &view ).unwrap();

  let detail_line = out.lines()
    .find( | l | l.contains( "note" ) )
    .expect( "detail line not found" );

  let indent_pos = detail_line.find( indent ).expect( "indent not found in detail line" );
  let ansi_pos = detail_line.find( yellow ).expect( "ANSI code not found in detail line" );

  assert!(
    indent_pos < ansi_pos,
    "indent must come before ANSI color code: {detail_line:?}",
  );
}

// =============================================================================
// T32 — detail text with trailing \n: output identical to text without trailing \n
//
// Rust's str::lines() strips the trailing newline — "detail\n" and "detail"
// yield the same iterator, so the rendered output must be identical.
// =============================================================================

#[ test ]
fn t32_detail_trailing_newline_stripped()
{
  let view_with = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "detail\n" ) ) )
    .build_view();

  let view_without = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( DecoratedText::from( "detail" ) ) )
    .build_view();

  assert_eq!(
    plain_output( &view_with ),
    plain_output( &view_without ),
    "trailing \\n in detail text must produce identical output to detail without trailing \\n",
  );
}

// =============================================================================
// T33 — detail text = "\n" only: not empty; renders one blank indented line
//
// "\n".is_empty() == false, so the detail is NOT suppressed by the is_empty()
// guard. str::lines() yields [""] (one empty string), so the formatter emits
// one iteration: indent + "" + \n → a line that is just the 2-space indent.
// Contrast with T04 where "" IS suppressed (is_empty() == true).
// =============================================================================

#[ test ]
fn t33_detail_only_newline_renders_blank_line()
{
  let ct = DecoratedText::from( "\n" );
  assert!( !ct.is_empty(), r#"DecoratedText::from("\n") must not be considered empty"# );

  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ DecoratedText::from( "Alice" ) ], Some( ct ) )
    .build_view();

  let out = plain_output( &view );
  let lines : Vec< &str > = out.lines().collect();

  // header + separator + data row + blank detail = 4 lines
  assert_eq!( lines.len(), 4, "expected 4 lines (header+sep+row+blank detail), got:\n{out}" );
  // The detail line is exactly the 2-space default indent with no content
  assert_eq!(
    lines[ 3 ],
    "  ",
    "detail-only-newline must render as just the indent: {:?}",
    lines[ 3 ],
  );
}
