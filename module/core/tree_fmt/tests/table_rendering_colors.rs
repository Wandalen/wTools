//! ANSI coloring tests for header row and alternating data rows (Task 013)
//!
//! ## What This Tests
//!
//! The temp-buffer coloring pipeline wired into `format_internal()`. When
//! `colorize_header = true`, the header row is wrapped in `header_color…RESET`.
//! When `alternating_rows = true`, data rows alternate between `row_color1`
//! and `row_color2`.
//!
//! ## Key Invariants
//!
//! - RESET (`\x1b[0m`) MUST appear BEFORE `\n` in every colored line.
//!   Terminal background colors bleed across the rest of the line if RESET is
//!   placed after `\n` or omitted.
//! - Empty color strings (`""`) suppress color wrapping entirely — no escape
//!   sequences are emitted even when `alternating_rows = true`.
//! - Borders and coloring are independent: grid borders still render correctly
//!   when coloring is enabled.
//!
//! ## Test Matrix
//!
//! ### T013-P: Positive tests
//!
//! | # | Scenario | Config | Expected |
//! |---|----------|--------|----------|
//! | T013-P01 | 2 data rows | `colorize_header(true)` + `header_color("\x1b[1m")` | Header wrapped in bold escape |
//! | T013-P02 | 3 data rows | `alternating_rows(true)` + two row colors | Even/odd rows get color1/color2 |
//! | T013-P03 | 1 data row | `colorize_header(true)` + `alternating_rows(true)` | Both header and row independently colored |
//! | T013-P04 | 2 data rows | defaults (no coloring) | Zero escape codes in output |
//! | T013-P05 | Any | color config | RESET `\x1b[0m` always appears before `\n` |
//!
//! ### T013-N: Negative / edge tests
//!
//! | # | Scenario | Config | Expected |
//! |---|----------|--------|----------|
//! | T013-N01 | 2 data rows | `alternating_rows(false)` (default) | No color codes in data rows |
//! | T013-N02 | 2 data rows | `row_colors("","")` + `alternating_rows(true)` | Empty strings: no escape codes emitted |
//! | T013-N03 | 1 data row | `alternating_rows(true)` + only `row_color1` set | Row 0 uses color1; no panic |
//! | T013-N04 | 2 data rows | `ColorTheme::dark()` applied | Theme-set header + alternating colors appear |
//! | T013-N05 | grid + colors | `grid()` + `colorize_header(true)` | Borders AND color both in output |
//! | T013-N06 | 1 data row | `colorize_header(true)`, no alternating | Only header colored; data row uncolored |

#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

mod inc;

use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };
use inc::sample_data;

// ---------------------------------------------------------------------------
// T013-P: Positive tests
// ---------------------------------------------------------------------------

/// T013-P01 — `colorize_header(true)` + `header_color` wraps the header line
/// in the specified ANSI escape code followed by RESET.
#[ test ]
fn test_t013_p01_colorize_header_wraps_header_in_escape_codes()
{
  let tree = sample_data();
  let config = TableConfig::plain()
    .colorize_header( true )
    .header_color( "\x1b[1m".to_string() );

  let output = TableFormatter::with_config( config ).format( &tree );

  // Header is the first non-border line; for plain() it is the first line
  let header_line = output.lines().next().expect( "output is empty" );
  assert!(
    header_line.starts_with( "\x1b[1m" ),
    "T013-P01: header line should start with color escape, got: {header_line:?}\nFull output:\n{output}"
  );
  assert!(
    header_line.contains( "\x1b[0m" ),
    "T013-P01: header line should contain RESET \\x1b[0m, got: {header_line:?}\nFull output:\n{output}"
  );
  // Should still contain header content
  assert!( header_line.contains( "NAME" ), "T013-P01: header content 'NAME' must be present" );
}

/// T013-P02 — `alternating_rows(true)` alternates color1 (even index) and
/// color2 (odd index) across data rows.
#[ test ]
fn test_t013_p02_alternating_rows_colors_even_odd()
{
  let tree = RowBuilder::new( vec![ "A".into() ] )
    .add_row( vec![ "row0".into() ] )
    .add_row( vec![ "row1".into() ] )
    .add_row( vec![ "row2".into() ] )
    .build();

  let config = TableConfig::plain()
    .alternating_rows( true )
    .row_colors( "\x1b[31m".to_string(), "\x1b[32m".to_string() );

  let output = TableFormatter::with_config( config ).format( &tree );

  // With plain(): line 0=header, line 1=sep, line 2=row0, line 3=row1, line 4=row2
  let lines : Vec< &str > = output.lines().collect();
  assert!( lines.len() >= 5, "Expected at least 5 lines (header+sep+3 rows); got {}", lines.len() );

  let row0 = lines[ 2 ];
  let row1 = lines[ 3 ];
  let row2 = lines[ 4 ];

  assert!(
    row0.starts_with( "\x1b[31m" ),
    "T013-P02: row0 (even index 0) should use color1 \\x1b[31m, got: {row0:?}"
  );
  assert!(
    row1.starts_with( "\x1b[32m" ),
    "T013-P02: row1 (odd index 1) should use color2 \\x1b[32m, got: {row1:?}"
  );
  assert!(
    row2.starts_with( "\x1b[31m" ),
    "T013-P02: row2 (even index 2) should use color1 \\x1b[31m, got: {row2:?}"
  );
}

/// T013-P03 — Both `colorize_header` and `alternating_rows` work independently
/// in the same config.
#[ test ]
fn test_t013_p03_header_and_row_coloring_independent()
{
  let tree = RowBuilder::new( vec![ "A".into() ] )
    .add_row( vec![ "x".into() ] )
    .build();

  let config = TableConfig::plain()
    .colorize_header( true )
    .header_color( "\x1b[1m".to_string() )
    .alternating_rows( true )
    .row_colors( "\x1b[31m".to_string(), "\x1b[32m".to_string() );

  let output = TableFormatter::with_config( config ).format( &tree );

  let lines : Vec< &str > = output.lines().collect();
  assert!( lines.len() >= 3, "Expected header + sep + data row" );

  let header_line = lines[ 0 ];
  let data_line = lines[ 2 ];  // line 0=header, line 1=sep, line 2=row0

  assert!(
    header_line.starts_with( "\x1b[1m" ),
    "T013-P03: header should use header_color \\x1b[1m, got: {header_line:?}"
  );
  assert!(
    data_line.starts_with( "\x1b[31m" ),
    "T013-P03: data row should use row_color1 \\x1b[31m, got: {data_line:?}"
  );
  // Header should NOT contain the row color
  assert!(
    !header_line.contains( "\x1b[31m" ),
    "T013-P03: header must not contain row color, got: {header_line:?}"
  );
}

/// T013-P04 — Default `TableConfig` (no coloring) produces zero escape codes.
///
/// This is both a positive test and a regression guard: uncolored output must
/// be byte-for-byte free of escape sequences.
#[ test ]
fn test_t013_p04_default_config_no_escape_codes()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree );

  assert!(
    !output.contains( '\x1b' ),
    "T013-P04: plain() output must contain zero escape codes\nFull output:\n{output}"
  );
}

/// T013-P05 — RESET `\x1b[0m` always appears BEFORE `\n` in every colored line.
///
/// ## Pitfall
/// Placing RESET after `\n` causes terminal background-color bleed: the color
/// remains active for the blank space after the last character until the terminal
/// resets at the RESET on the next line. Always use `{color}{content}{RESET}\n`,
/// never `{color}{content}\n{RESET}`.
#[ test ]
fn test_t013_p05_reset_appears_before_newline()
{
  let tree = sample_data();
  let config = TableConfig::plain()
    .colorize_header( true )
    .header_color( "\x1b[1m".to_string() );

  let output = TableFormatter::with_config( config ).format( &tree );

  // RESET must directly precede the newline in the colored header line
  assert!(
    output.contains( "\x1b[0m\n" ),
    "T013-P05: RESET must appear directly before \\n (pattern \\x1b[0m\\n), not found\nFull output:\n{output:?}"
  );
  // RESET must NOT appear after a newline (which would mean it was placed on the next line)
  assert!(
    !output.contains( "\n\x1b[0m" ),
    "T013-P05: found \\n\\x1b[0m — RESET placed after newline causes background color bleed\nFull output:\n{output:?}"
  );
}

// ---------------------------------------------------------------------------
// T013-N: Negative / edge tests
// ---------------------------------------------------------------------------

/// T013-N01 — `alternating_rows(false)` (default): no color codes in data rows.
#[ test ]
fn test_t013_n01_no_alternating_rows_no_escape_codes()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree );

  // With plain() defaults, no color fields are set
  let has_escape = output.lines().skip( 2 ).any( |l| l.contains( '\x1b' ) );
  assert!(
    !has_escape,
    "T013-N01: data rows must not contain escape codes when alternating_rows=false\nFull output:\n{output}"
  );
}

/// T013-N02 — `row_colors("", "")` with `alternating_rows(true)`: empty color
/// strings suppress escape code emission entirely.
///
/// Empty string color means "no-op" — the row is emitted without any wrapping.
#[ test ]
fn test_t013_n02_empty_color_strings_suppress_escapes()
{
  let tree = sample_data();
  let config = TableConfig::plain()
    .alternating_rows( true )
    .row_colors( String::new(), String::new() );

  let output = TableFormatter::with_config( config ).format( &tree );

  assert!(
    !output.contains( '\x1b' ),
    "T013-N02: empty color strings must produce zero escape codes\nFull output:\n{output}"
  );
}

/// T013-N03 — Single data row with only `row_color1` set: works without `row_color2`.
///
/// A table with exactly 1 data row never reaches the odd-index branch; only
/// `row_color1` is needed and must not panic.
#[ test ]
fn test_t013_n03_single_row_only_color1()
{
  let tree = RowBuilder::new( vec![ "A".into() ] )
    .add_row( vec![ "x".into() ] )
    .build();

  let config = TableConfig::plain()
    .alternating_rows( true )
    .row_colors( "\x1b[31m".to_string(), String::new() );

  let output = TableFormatter::with_config( config ).format( &tree );

  // Should not panic; row 0 (even) gets color1
  assert!(
    output.contains( "\x1b[31m" ),
    "T013-N03: single data row should use row_color1 \\x1b[31m\nFull output:\n{output}"
  );
}

/// T013-N04 — `ColorTheme::dark()` applied via `apply_to_table()` produces both
/// colored headers and alternating row colors.
#[ cfg( feature = "themes" ) ]
#[ test ]
fn test_t013_n04_theme_applied_produces_colored_output()
{
  use tree_fmt::ColorTheme;

  let tree = sample_data();
  let config = ColorTheme::dark().apply_to_table( TableConfig::plain() );

  let output = TableFormatter::with_config( config ).format( &tree );

  // dark() header_color = "\x1b[1;36m" (bright cyan bold)
  assert!(
    output.contains( "\x1b[1;36m" ),
    "T013-N04: dark theme header color must appear in output\nFull output:\n{output}"
  );
  // dark() row_color2 = "\x1b[48;5;235m" (alternating background)
  assert!(
    output.contains( "\x1b[48;5;235m" ),
    "T013-N04: dark theme row alternating color must appear\nFull output:\n{output}"
  );
}

/// T013-N05 — `TableConfig::grid()` + `colorize_header(true)`: borders AND color
/// both appear in the output (the two features must coexist).
#[ test ]
fn test_t013_n05_grid_borders_and_colors_coexist()
{
  let tree = sample_data();
  let config = TableConfig::grid()
    .colorize_header( true )
    .header_color( "\x1b[1m".to_string() );

  let output = TableFormatter::with_config( config ).format( &tree );

  // Grid top border must still be present
  let first_line = output.lines().next().expect( "output is empty" );
  assert!(
    first_line.starts_with( '+' ),
    "T013-N05: grid top border must still render with coloring enabled, got: {first_line:?}"
  );

  // Header color must also be present somewhere after the top border
  let has_color = output.lines().any( |l| l.contains( "\x1b[1m" ) );
  assert!(
    has_color,
    "T013-N05: header color \\x1b[1m must appear in output\nFull output:\n{output}"
  );
}

/// T013-N06 — `colorize_header(true)` with 1 data row: only header is colored;
/// data row remains uncolored (no `alternating_rows`). No panic.
#[ test ]
fn test_t013_n06_header_colored_data_uncolored_single_row()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build();

  let config = TableConfig::plain()
    .colorize_header( true )
    .header_color( "\x1b[1m".to_string() );

  let output = TableFormatter::with_config( config ).format( &tree );

  // Must not panic; header must be colored
  assert!(
    output.contains( "\x1b[1m" ),
    "T013-N06: header must be colored even with 1 data row\nFull output:\n{output}"
  );

  // Count lines that contain the header color — should be exactly 1 (header only)
  let colored_count = output.lines().filter( |l| l.contains( "\x1b[1m" ) ).count();
  assert_eq!(
    colored_count, 1,
    "T013-N06: only the header line should contain the header color, got {colored_count}\nFull output:\n{output}"
  );
}
