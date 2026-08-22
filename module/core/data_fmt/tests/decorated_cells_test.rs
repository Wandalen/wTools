//! `DecoratedText` cell integration tests
//!
//! ## What This Tests
//!
//! The `RowBuilder` → `build_view()` → `Format::format` path when individual table
//! cells carry `DecoratedText` with a color prefix. Each cell is rendered with its own
//! ANSI escape sequence independently of any row-level `TableConfig` coloring.
//!
//! ## Key Invariants Verified
//!
//! - A colored `DecoratedText` cell renders as `color + text + "\x1b[0m"` in output.
//! - A plain (uncolored) `DecoratedText` cell renders as plain text with no ANSI codes.
//! - A multi-line colored cell wraps each sub-line independently:
//!   `color + line + "\x1b[0m"\n` — never `color + line1\nline2 + "\x1b[0m"`.
//!   The second form would cause background-color bleed across the `\n` boundary.
//!
//! ## Test Matrix
//!
//! | # | Scenario | Input | Expected |
//! |---|----------|-------|----------|
//! | P01 | Colored single-line cell | `.with_color("\x1b[33m")` on "warn" | Output contains `\x1b[33mwarn\x1b[0m` |
//! | P02 | Plain cell | `DecoratedText::from("plain")` | No escape codes anywhere |
//! | P03 | Multi-line colored cell | `{text:"a\nb", color:"\x1b[32m"}` | Each sub-line ends with `\x1b[0m` before `\n`; 2 colored lines total |
//! | P04 | Mixed: colored + plain cells in same row | one colored, one plain | Colored cell has ANSI; plain cell does not |
//! | P05 | Multi-line, bold-only cell (no color) | `.with_bold()` on "a\nb" | Both sub-lines carry `\x1b[1m` |
//! | P06 | Multi-line, dim-only cell (no color) | `.with_dim()` on "a\nb" | Both sub-lines carry `\x1b[2m` |
//! | P07 | Markdown cell, bold + color | `.with_color(..).with_bold()` on `"a \| b"` | Pipe escaped AND both color+bold survive |

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, DecoratedText, Format };

// ---------------------------------------------------------------------------
// P01: Colored cell renders with ANSI sequence
// ---------------------------------------------------------------------------

/// P01 — A `DecoratedText` cell with `with_color("\x1b[33m")` produces
/// `\x1b[33mwarn\x1b[0m` in the formatted table output.
///
/// ## Root Cause (Prevention)
///
/// Before migration, `TableView::rows: Vec<Vec<String>>` discards color info at
/// cell construction time. After migration (`Vec<Vec<DecoratedText>>`), the formatter
/// receives the full `DecoratedText` and calls `ct.render()` to emit color + text + RESET.
///
/// ## Pitfall
///
/// Width measurement must use `ct.text` (raw text), not `ct.render()` (ANSI-decorated
/// string). Calling `unicode_visual_len` on `ct.render()` would count ANSI bytes as
/// display columns, producing over-wide columns.
#[ test ]
fn test_p01_colored_cell_renders_ansi_in_output()
{
  let view = RowBuilder::new( vec![ "Status".into() ] )
    .add_row( vec![ DecoratedText::from( "warn" ).with_color( "\x1b[33m" ) ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!(
    output.contains( "\x1b[33mwarn\x1b[0m" ),
    "P01: colored cell must render as color+text+RESET\nFull output:\n{output:?}"
  );
}

// ---------------------------------------------------------------------------
// P02: Plain cell — no escape codes
// ---------------------------------------------------------------------------

/// P02 — A `DecoratedText` cell with no color (`color: None`) renders as plain
/// text with zero ANSI escape codes in the output.
///
/// Regression guard: migrating the data model to `Vec<Vec<DecoratedText>>` must NOT
/// inject escape sequences for uncolored cells.
#[ test ]
fn test_p02_plain_decorated_cell_no_ansi_codes()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ DecoratedText::from( "plain" ) ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!(
    output.contains( "plain" ),
    "P02: plain cell text must be present in output\nFull output:\n{output:?}"
  );
  assert!(
    !output.contains( '\x1b' ),
    "P02: plain cell must not inject any ANSI escape codes\nFull output:\n{output:?}"
  );
}

// ---------------------------------------------------------------------------
// P03: Multi-line colored cell — per-line ANSI reset, no bleed
// ---------------------------------------------------------------------------

/// P03 — A multi-line colored cell emits per-line ANSI wrapping: each output
/// sub-line ends with `\x1b[0m` before the newline.
///
/// ## Root Cause (Prevention)
///
/// Calling `DecoratedText::render()` on a multi-line colored cell produces
/// `color + "line_a\nline_b" + "\x1b[0m"`. When this is written to a terminal,
/// the `\n` between `line_a` and `line_b` appears INSIDE the color sequence,
/// causing the background color to bleed across the boundary — the line after
/// `line_a` has no RESET before its `\n`.
///
/// ## Fix Applied
///
/// The formatter iterates `ct.text.lines()` and wraps each line individually:
/// `color + line + "\x1b[0m"\n`. This ensures RESET appears before every `\n`.
///
/// ## Pitfall
///
/// Never call `ct.render()` and then `.lines()` on the result — the RESET
/// appears at the very end of the block, not before each internal `\n`.
/// Always iterate `ct.text.lines()` and apply color/RESET per output line.
#[ test ]
fn test_p03_multiline_colored_cell_per_line_reset_no_bleed()
{
  let cell = DecoratedText::from( "line_a\nline_b" ).with_color( "\x1b[32m" );

  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ cell ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = Format::format( &formatter, &view ).unwrap();

  // Every colored line must end with RESET directly before the newline (implicitly via .lines())
  for ( idx, line ) in output.lines().enumerate()
  {
    if line.starts_with( '\x1b' )
    {
      assert!(
        line.ends_with( "\x1b[0m" ),
        "P03: colored line {idx} must end with RESET \\x1b[0m\n  line: {line:?}\nFull output:\n{output:?}"
      );
    }
  }

  // Both sub-lines carry the color prefix
  let green_lines : Vec< &str > = output.lines().filter( | l | l.contains( "\x1b[32m" ) ).collect();
  assert_eq!(
    green_lines.len(), 2,
    "P03: 2-line colored cell must produce 2 colored sub-lines; got {}\nFull output:\n{output:?}",
    green_lines.len()
  );
}

// ---------------------------------------------------------------------------
// P04: Mixed row — colored + plain cells coexist
// ---------------------------------------------------------------------------

/// P04 — A row with one colored cell and one plain cell: the colored cell gets
/// ANSI sequences; the plain cell does not. The formatter must not bleed color
/// across column boundaries.
#[ test ]
fn test_p04_mixed_colored_and_plain_cells_in_row()
{
  let view = RowBuilder::new( vec![ "Status".into(), "Note".into() ] )
    .add_row( vec![
      DecoratedText::from( "warn" ).with_color( "\x1b[33m" ),
      DecoratedText::from( "plain note" ),
    ])
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = Format::format( &formatter, &view ).unwrap();

  // Colored cell must be present
  assert!(
    output.contains( "\x1b[33mwarn\x1b[0m" ),
    "P04: colored cell must render as color+text+RESET\nFull output:\n{output:?}"
  );

  // Plain content must be present
  assert!(
    output.contains( "plain note" ),
    "P04: plain cell text must appear in output\nFull output:\n{output:?}"
  );

  // The data row must contain exactly one RESET (from the colored cell only)
  // Find the data row line (skip header and separator)
  let data_lines : Vec< &str > = output.lines().skip( 2 ).collect();
  let reset_count : usize = data_lines
    .iter()
    .map( | l | l.matches( "\x1b[0m" ).count() )
    .sum();
  assert_eq!(
    reset_count, 1,
    "P04: exactly one RESET expected (from colored cell); got {reset_count}\nFull output:\n{output:?}"
  );
}

// ---------------------------------------------------------------------------
// P05-P07: bold/dim styling survives cell reconstruction (BUG-024)
// ---------------------------------------------------------------------------

/// P05 — A multi-line cell with `with_bold()` set and NO color: bold must still
/// wrap each sub-line.
///
/// ## Root Cause
///
/// `format_row_colored`'s multiline branch gated per-line styling on
/// `ct.color.is_some()` alone. A cell with `bold: true, color: None` fell into
/// the `else` arm and was emitted as fully plain text — bold was silently lost.
///
/// ## Why Not Caught
///
/// All prior multiline tests (P03, T013-M01/M02/M06 in `table_rendering_colors.rs`)
/// set a color on the cell. None exercised a multiline cell styled by bold/dim alone.
///
/// ## Fix Applied
///
/// The branch condition now checks `ct.color.is_some() || ct.bold || ct.dim`, and
/// each sub-line is rebuilt via a shared `restyle()` helper that carries color,
/// bold, AND dim onto the fresh per-line `DecoratedText` before `.render()`
/// (`src/formatters/table/row_rendering.rs`).
///
/// ## Prevention
///
/// Any code that reconstructs a `DecoratedText` from another cell's styling
/// (rather than cloning the cell directly) must copy every style field — color,
/// bold, dim — not just color.
///
/// ## Pitfall
///
/// `ct.color.is_some()` alone is not "is this cell styled" — `bold`/`dim` are
/// independent boolean fields that can be set without a color. Always OR all
/// three when deciding whether a cell needs the styled rendering path.
// test_kind: bug_reproducer(BUG-024)
#[ test ]
fn test_p05_multiline_bold_only_cell_preserves_bold_no_color()
{
  let cell = DecoratedText::from( "line_a\nline_b" ).with_bold();

  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ cell ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = Format::format( &formatter, &view ).unwrap();

  let bold_lines : Vec< &str > = output.lines().filter( | l | l.contains( "\x1b[1m" ) ).collect();
  assert_eq!(
    bold_lines.len(), 2,
    "P05: 2-line bold-only cell must produce 2 bold sub-lines; got {}\nFull output:\n{output:?}",
    bold_lines.len()
  );
}

/// P06 — Same root cause as P05 but for `with_dim()`: dim must survive multiline
/// splitting with no color set. See P05 doc comment for full Root Cause / Fix Applied.
// test_kind: bug_reproducer(BUG-024)
#[ test ]
fn test_p06_multiline_dim_only_cell_preserves_dim_no_color()
{
  let cell = DecoratedText::from( "line_a\nline_b" ).with_dim();

  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ cell ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = Format::format( &formatter, &view ).unwrap();

  let dim_lines : Vec< &str > = output.lines().filter( | l | l.contains( "\x1b[2m" ) ).collect();
  assert_eq!(
    dim_lines.len(), 2,
    "P06: 2-line dim-only cell must produce 2 dim sub-lines; got {}\nFull output:\n{output:?}",
    dim_lines.len()
  );
}

/// P07 — A Markdown-formatted cell with both `with_bold()` and a color: after
/// pipe-escaping, BOTH the color AND bold must survive.
///
/// ## Root Cause
///
/// `format_row_colored`'s Markdown branch reconstructed the escaped cell via
/// `DecoratedText::from(escaped_text)` and re-applied ONLY `.color` — bold and
/// dim were dropped silently regardless of the outer branch's condition.
///
/// ## Fix Applied
///
/// The same shared `restyle()` helper used by the multiline fix (P05/P06) is
/// applied here too, so pipe-escaping no longer strips bold/dim.
// test_kind: bug_reproducer(BUG-024)
#[ test ]
fn test_p07_markdown_cell_bold_and_color_survive_pipe_escaping()
{
  let cell = DecoratedText::from( "a | b" ).with_color( "\x1b[33m" ).with_bold();

  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ cell ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::markdown() );
  let output = Format::format( &formatter, &view ).unwrap();

  // Pipe must still be escaped (BUG-022 behavior preserved)
  assert!(
    output.contains( r"a \| b" ),
    "P07: pipe must remain escaped after restyling, got:\n{output}"
  );

  // header=0, separator=1, data=2 (matches BUG-022 precedent in corner_case_bug_reproducer_test.rs)
  let data_line = output.lines().nth( 2 ).expect( "expected a data line" );
  assert!(
    data_line.contains( "\x1b[33m" ),
    "P07: color must survive pipe-escaping, got: {data_line:?}"
  );
  assert!(
    data_line.contains( "\x1b[1m" ),
    "P07: bold must survive pipe-escaping (was silently dropped before BUG-024 fix), got: {data_line:?}"
  );
}
