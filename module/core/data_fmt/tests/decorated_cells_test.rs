//! DecoratedText cell integration tests
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

#![ cfg( feature = "enabled" ) ]
#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

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
        "P03: colored line {idx} must end with RESET \\x1b[0m\n  line: {:?}\nFull output:\n{output:?}",
        line
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
