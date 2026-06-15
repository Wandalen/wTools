//! Tests for table style presets and new configuration options (v0.3.0)
//!
//! ## What This Tests
//!
//! Tests the 9 table style preset constructors and configuration builder methods
//! introduced in v0.3.0 comprehensive parametrization refactoring.
//!
//! All assertions are behavioral (output-based) to survive field-privacy change (v0.10.0).
//!
//! ## Style Presets Tested
//!
//! 1. **Plain** - CLI tool output (space-separated, dash separator) - DEFAULT
//! 2. **Minimal** - Maximum simplicity (no separator)
//! 3. **Bordered** - Traditional pipe borders
//! 4. **Markdown** - GitHub-flavored Markdown tables
//! 5. **Grid** - Full ASCII grid with intersections
//! 6. **Unicode Box** - Unicode box-drawing characters
//! 7. **CSV** - Comma-separated values
//! 8. **TSV** - Tab-separated values
//! 9. **Compact** - Minimal spacing for density
//!
//! ## Key Insights Captured
//!
//! 1. **Preset Correctness**: Each preset produces expected border/separator characters
//! 2. **Builder Pattern**: Fluent API allows combining presets with custom options
//! 3. **Enum Defaults**: `BorderVariant`, `HeaderSeparatorVariant`, `ColumnSeparator` have sensible defaults
//! 4. **Color/`min_column_width`**: These `TableConfig` fields are not read by `TableFormatter`;
//!    tests that set them are smoke tests documenting API contract for future implementation.
//!
//! ## Design Rationale
//!
//! **Why 9 presets?** Different output contexts have different requirements:
//! - CLI tools need clean, scannable output (Plain)
//! - Documentation needs Markdown compatibility
//! - Data export needs CSV/TSV
//! - Reports need visual clarity (Grid, Unicode Box)
//!
//! These tests ensure each preset produces the intended style and that builder
//! methods properly override preset defaults.
//!
//! Split from `tests/table_styles.rs` (509 lines) in v0.4.0 compliance cleanup.
//!
//! ## Common Pitfalls to Avoid
//!
//! - **Unicode separator mismatch:** Never pair `header_separator_variant: Unicode` with
//!   `column_separator: Spaces(N)`. The header separator emits `┼` but data rows show spaces.
//!   Always use `TableConfig::unicode_box()` which pairs all three Unicode fields correctly.
//!
//! - **Default column separator surprise:** `TableConfig::default()` (= `new()`) sets
//!   `column_separator: Spaces(2)`, not `ColumnSeparator::default()` which is `Character('|')`.
//!   Use `bordered()` if pipe separators are needed without an explicit setter call.
//!
//! - **Smoke tests for unimplemented fields:** `border_variant`, `colorize_header`,
//!   `header_color`, `alternating_rows`, `row_color1`, `row_color2`, `min_column_width`
//!   are stored but not read by `TableFormatter`. Tests covering these fields must be
//!   behavioral smoke tests (verify no panic + data renders), not output-character assertions.

#![ cfg( feature = "enabled" ) ]

mod inc;

use data_fmt::
{
  RowBuilder, TableFormatter, TableConfig,
  BorderVariant, HeaderSeparatorVariant, ColumnSeparator, Format,
};
use inc::sample_data;

// =============================================================================
// Configuration Enum Tests
// =============================================================================

#[ test ]
fn test_border_style_enum_defaults()
{
  assert_eq!( BorderVariant::default(), BorderVariant::Ascii );
}

#[ test ]
fn test_header_separator_style_enum_defaults()
{
  assert_eq!( HeaderSeparatorVariant::default(), HeaderSeparatorVariant::AsciiGrid );
}

#[ test ]
fn test_column_separator_enum_defaults()
{
  assert_eq!( ColumnSeparator::default(), ColumnSeparator::Character( '|' ) );
}

// =============================================================================
// Style Preset Constructor Tests
// =============================================================================

#[ test ]
fn test_plain_style_config()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();

  // Plain: no borders, space-separated, dash header separator
  assert!( !output.contains( '|' ), "plain must not have | borders; output:\n{output}" );
  assert!( !output.contains( '│' ), "plain must not have │ borders; output:\n{output}" );
  assert!( output.contains( '-' ), "plain must have dash separator; output:\n{output}" );
  assert!( output.contains( "Alice" ), "plain must contain data; output:\n{output}" );
}

#[ test ]
fn test_minimal_style_config()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::minimal() ).format( &tree ).unwrap_or_default();

  // Minimal: no borders, no header separator
  assert!( !output.contains( '|' ), "minimal must not have | borders; output:\n{output}" );
  assert!( !output.contains( '│' ), "minimal must not have │ borders; output:\n{output}" );
  assert!( !output.contains( "---" ), "minimal must not have dash separator; output:\n{output}" );
  assert!( output.contains( "Alice" ), "minimal must contain data; output:\n{output}" );
}

#[ test ]
fn test_bordered_style_config()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::bordered() ).format( &tree ).unwrap_or_default();

  // Bordered: ASCII pipe borders, AsciiGrid separator, inner_padding=1
  assert!( output.contains( '|' ), "bordered must have | borders; output:\n{output}" );
  assert!( output.contains( '-' ), "bordered must have dash separator; output:\n{output}" );
  assert!( output.contains( "Alice" ), "bordered must contain data; output:\n{output}" );
}

#[ test ]
fn test_markdown_style_config()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::markdown() ).format( &tree ).unwrap_or_default();

  // Markdown: | column separators, Markdown separator row (|---|)
  assert!( output.contains( '|' ), "markdown must have | column separators; output:\n{output}" );
  assert!(
    output.contains( "---" ) || output.contains( "|-" ),
    "markdown must have header separator; output:\n{output}"
  );
  assert!( output.contains( "Alice" ), "markdown must contain data; output:\n{output}" );
}

#[ test ]
fn test_grid_style_config()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::grid() ).format( &tree ).unwrap_or_default();

  // Grid: AsciiGrid header separator produces |---|---| (pipe+dash, no + intersection)
  // border_variant is stored but not yet rendered by TableFormatter
  assert!( output.contains( '|' ), "grid must have | column separators; output:\n{output}" );
  assert!( output.contains( '-' ), "grid must have dash separator; output:\n{output}" );
  assert!( output.contains( "Alice" ), "grid must contain data; output:\n{output}" );
}

#[ test ]
fn test_unicode_box_style_config()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::unicode_box() ).format( &tree ).unwrap_or_default();

  // Unicode box: Unicode box-drawing characters
  assert!( output.contains( '│' ), "unicode_box must have │ column separators; output:\n{output}" );
  assert!(
    output.contains( '┼' ) || output.contains( '├' ) || output.contains( '┤' ),
    "unicode_box must have Unicode separator characters; output:\n{output}"
  );
  assert!( output.contains( "Alice" ), "unicode_box must contain data; output:\n{output}" );
}

#[ test ]
fn test_csv_style_config()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::csv() ).format( &tree ).unwrap_or_default();

  // CSV: comma-separated, no borders, no header separator
  assert!( output.contains( ',' ), "csv must have comma separators; output:\n{output}" );
  assert!( !output.contains( '|' ), "csv must not have | borders; output:\n{output}" );
  assert!( !output.contains( "---" ), "csv must not have dash separator; output:\n{output}" );
  assert!( output.contains( "Alice" ), "csv must contain data; output:\n{output}" );
}

#[ test ]
fn test_tsv_style_config()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::tsv() ).format( &tree ).unwrap_or_default();

  // TSV: tab-separated, no borders
  assert!( output.contains( '\t' ), "tsv must have tab separators; output:\n{output}" );
  assert!( !output.contains( '|' ), "tsv must not have | borders; output:\n{output}" );
  assert!( output.contains( "Alice" ), "tsv must contain data; output:\n{output}" );
}

#[ test ]
fn test_compact_style_config()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::compact() ).format( &tree ).unwrap_or_default();

  // Compact: single-space separator, no borders, no header separator
  assert!( !output.contains( '|' ), "compact must not have | borders; output:\n{output}" );
  assert!( !output.contains( '│' ), "compact must not have │ borders; output:\n{output}" );
  assert!( !output.contains( "---" ), "compact must not have header separator; output:\n{output}" );
  assert!( output.contains( "Alice" ), "compact must contain data; output:\n{output}" );
}

// =============================================================================
// Builder Method Tests
// =============================================================================

#[ test ]
fn test_table_config_builder_border_style()
{
  let tree = sample_data();
  // border_variant is stored but not yet rendered by TableFormatter;
  // the setter must compile and not affect data rendering
  let output = TableFormatter::with_config(
    TableConfig::new().with_border_variant( BorderVariant::None )
  ).format( &tree ).unwrap_or_default();

  assert!( output.contains( "Alice" ), "border_variant setter must not break rendering; output:\n{output}" );
}

#[ test ]
fn test_table_config_builder_header_separator()
{
  let tree = sample_data();
  let output = TableFormatter::with_config(
    TableConfig::new().with_header_separator_variant( HeaderSeparatorVariant::Dash )
  ).format( &tree ).unwrap_or_default();

  // Dash separator → dash line between header and rows
  assert!( output.contains( "---" ), "Dash separator must produce dash line; output:\n{output}" );
  assert!( output.contains( "Alice" ), "output must contain data; output:\n{output}" );
}

#[ test ]
fn test_table_config_builder_column_separator()
{
  let tree = sample_data();
  let output = TableFormatter::with_config(
    TableConfig::new().with_column_separator( ColumnSeparator::Spaces( 4 ) )
  ).format( &tree ).unwrap_or_default();

  // Spaces(4) → no | separator in output
  assert!( !output.contains( '|' ), "Spaces separator must not produce | chars; output:\n{output}" );
  assert!( output.contains( "Alice" ), "output must contain data; output:\n{output}" );
}

#[ test ]
fn test_table_config_builder_padding()
{
  let tree = sample_data();

  // inner_padding=2 with outer_padding=true: outer cells get 2-space padding
  // Use bordered() as base since it has character separator (visible borders)
  let output_padded = TableFormatter::with_config(
    TableConfig::bordered()
      .with_outer_padding( true )
      .with_inner_padding( 2 )
  ).format( &tree ).unwrap_or_default();

  // With outer_padding=true and inner_padding=2, rows start with |  (border + 2 spaces)
  assert!(
    output_padded.lines().any( | l | l.starts_with( "|  " ) ),
    "inner_padding=2 + outer_padding=true should produce |  prefix; output:\n{output_padded}"
  );

  // outer_padding=false: rows start with | but NO leading spaces after border
  let output_no_outer = TableFormatter::with_config(
    TableConfig::bordered()
      .with_outer_padding( false )
      .with_inner_padding( 2 )
  ).format( &tree ).unwrap_or_default();

  // With outer_padding=false, rows should NOT start with |  (no extra spaces)
  assert!(
    !output_no_outer.lines().any( | l | l.starts_with( "|  " ) ),
    "outer_padding=false must not produce |  prefix; output:\n{output_no_outer}"
  );
  assert!( output_padded.contains( "Alice" ), "output must contain data" );
}

#[ test ]
fn test_table_config_builder_colors()
{
  // Color fields (colorize_header, header_color, alternating_rows, row_colors) are stored
  // in TableConfig but not yet read by TableFormatter — they are reserved for future
  // theme-driven rendering. This test documents the API contract and verifies the
  // builder chain compiles without errors and still renders data.
  let tree = sample_data();
  let output = TableFormatter::with_config(
    TableConfig::new()
      .with_colorize_header( true )
      .with_header_color( "\x1b[36m".to_string() )
      .with_alternating_rows( true )
      .with_row_colors( "\x1b[0m".to_string(), "\x1b[48;5;236m".to_string() )
  ).format( &tree ).unwrap_or_default();

  assert!( output.contains( "Alice" ), "color-configured table must still render data; output:\n{output}" );
}

#[ test ]
fn test_table_config_builder_width_constraints()
{
  // max_column_width and truncation_marker ARE used by the formatter
  let tree = RowBuilder::new( vec![ "Column".into() ] )
    .add_row( vec![ "This is a long string".into() ] )
    .build_view();

  let output = TableFormatter::with_config(
    TableConfig::new()
      .with_max_column_width( Some( 8 ) )
      .with_truncation_marker( "…".to_string() )
  ).format( &tree ).unwrap_or_default();

  // max_column_width=8 truncates long content; "…" marker appears
  assert!(
    output.contains( '…' ),
    "max_column_width should truncate with custom '…' marker; output:\n{output}"
  );
}

#[ test ]
fn test_table_config_builder_chaining()
{
  let tree = sample_data();
  let output = TableFormatter::with_config(
    TableConfig::plain()
      .with_colorize_header( true )
      .with_header_color( "\x1b[1;36m".to_string() )
      .with_min_column_width( 5 )
  ).format( &tree ).unwrap_or_default();

  // plain() base is preserved: no | borders
  assert!( !output.contains( '|' ), "chained from plain() must not have | borders; output:\n{output}" );
  assert!( output.contains( "Alice" ), "chained config must contain data; output:\n{output}" );
}

// =============================================================================
// Behavioral Tests (T02–T07) — output-based, survive field-privacy change
// =============================================================================

// T02: unicode_box produces Unicode column separators and Unicode header separator
#[ test ]
fn test_unicode_box_behavioral_output()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::unicode_box() ).format( &tree ).unwrap_or_default();
  let lines : Vec< &str > = output.lines().collect();
  assert!(
    lines.iter().any( | l | l.contains( '│' ) ),
    "unicode_box data rows must contain │; output:\n{output}"
  );
  assert!(
    lines.iter().any( | l | l.contains( '┼' ) || l.contains( '├' ) ),
    "unicode_box must have Unicode header separator ( ┼ or ├ ); output:\n{output}"
  );
}

// T03: bordered produces ASCII column separators and ASCII header separator
#[ test ]
fn test_bordered_behavioral_output()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::bordered() ).format( &tree ).unwrap_or_default();
  let lines : Vec< &str > = output.lines().collect();
  assert!(
    lines.iter().any( | l | l.contains( '|' ) ),
    "bordered data rows must contain |; output:\n{output}"
  );
  assert!(
    lines.iter().any( | l | l.contains( '-' ) ),
    "bordered must have dash header separator; output:\n{output}"
  );
}

// T04: plain has no pipe characters in data rows but has dash separator
#[ test ]
fn test_plain_behavioral_output()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::plain() ).format( &tree ).unwrap_or_default();
  let lines : Vec< &str > = output.lines().collect();
  assert!(
    !lines.iter().any( | l | l.contains( '│' ) ),
    "plain must not contain Unicode │; output:\n{output}"
  );
  assert!(
    lines.iter().any( | l | l.contains( '-' ) ),
    "plain must have dash header separator; output:\n{output}"
  );
}

// T07: default config ( new() ) uses Spaces(2) — no pipe chars in data rows
#[ test ]
fn test_default_config_uses_spaces_not_pipe()
{
  let tree = sample_data();
  let output = TableFormatter::with_config( TableConfig::new() ).format( &tree ).unwrap_or_default();
  assert!(
    !output.contains( '│' ),
    "default config must not produce Unicode │ column separators; output:\n{output}"
  );
  assert!(
    !output.contains( '|' ),
    "default config must not produce | column separators ( uses Spaces ); output:\n{output}"
  );
}

// =============================================================================
// Bug Reproducers
// =============================================================================

/// Reproduces the separator mismatch bug where Unicode header separator was paired
/// with a non-Unicode column separator, producing `┼` in the separator row but
/// spaces between data columns (BUG-003).
///
/// ## Root Cause
/// `gi_infra::formatters::style::cli_table()` constructed `TableConfig` via struct
/// literal, setting `header_separator_variant: HeaderSeparatorVariant::Unicode` but
/// relying on `..TableConfig::default()` for `column_separator`, which defaults to
/// `Spaces(2)`. The Unicode header separator emits `┼` between columns in the separator
/// row, but data rows used spaces — producing misaligned, visually broken output.
///
/// ## Why Not Caught
/// No test verified that `unicode_box()` produces BOTH Unicode column separators (`│`)
/// AND Unicode header separator characters (`┼`/`├`) simultaneously. Tests only checked
/// individual preset field values (state assertions), not behavioral correctness across
/// semantically interdependent field pairs.
///
/// ## Fix Applied
/// Two-part fix: (1) `gi_infra::formatters::style::cli_table()` replaced struct literal
/// with `TableConfig::unicode_box()` preset which pairs all three Unicode fields correctly.
/// (2) `TableConfig` fields made private (v0.10.0) so future struct literal
/// misconfigurations are compile errors rather than silent runtime breakage.
///
/// ## Prevention
/// `TableConfig` fields are now private (v0.10.0). Struct literal initialization outside
/// `src/config.rs` is a compile error. All callers must use presets or the builder chain.
/// The `compile_fail` doctest in `TableConfig` guards this invariant permanently.
///
/// ## Pitfall
/// Unicode separator components are interdependent — `header_separator_variant: Unicode`
/// requires `column_separator: Character('│')`. Always use `TableConfig::unicode_box()`
/// rather than manually pairing Unicode header separator with a non-Unicode column separator.
// test_kind: bug_reproducer(BUG-003)
#[ test ]
fn bug_reproducer_issue_011_unicode_box_column_separator_mismatch()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Score".into() ] )
    .add_row( vec![ "Alice".into(), "95".into() ] )
    .add_row( vec![ "Bob".into(), "87".into() ] )
    .build_view();
  let output = TableFormatter::with_config( TableConfig::unicode_box() ).format( &tree ).unwrap_or_default();
  let lines : Vec< &str > = output.lines().collect();
  // Data rows must have `│` column separators (not spaces)
  assert!(
    lines.iter().any( | l | l.contains( '│' ) ),
    "unicode_box data rows must use │ column separator, not spaces; output:\n{output}"
  );
  // Header separator must contain Unicode characters, not plain dashes
  assert!(
    lines.iter().any( | l | l.contains( '┼' ) || l.contains( '├' ) ),
    "unicode_box header separator must use ┼ or ├, not plain dashes; output:\n{output}"
  );
}

/// FT-7 — `feature/001`: `min_column_width` raises column width to configured floor.
///
/// A table where the natural column content width is 3 characters; `min_column_width(10)`
/// is configured. The column must be at least 10 characters wide in the output — the cell
/// value is padded with spaces to the minimum width.
// test_kind: standard
#[ test ]
fn min_column_width_raises_column_to_floor_ft7()
{
  // Natural width of "abc" = 3, "xyz" = 3 → natural column width = 3
  // With min_column_width(10): column floor raised to 10
  let view = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "abc".into() ] )
    .add_row( vec![ "xyz".into() ] )
    .build_view();

  let config_natural = TableConfig::plain();
  let config_min = TableConfig::plain().with_min_column_width( 10 );

  let out_natural = TableFormatter::with_config( config_natural ).format( &view ).unwrap_or_default();
  let out_min = TableFormatter::with_config( config_min ).format( &view ).unwrap_or_default();

  // With min_column_width(10): data line must be at least 10 characters wide
  let data_line = out_min.lines().find( | l | l.contains( "abc" ) )
    .expect( "data row with 'abc' must appear in output" );
  assert!(
    data_line.len() >= 10,
    "min_column_width(10): data line must be ≥ 10 chars wide; got {}: {data_line:?}\noutput:\n{out_min}",
    data_line.len(),
  );

  // Natural output is narrower (no floor applied)
  let data_line_nat = out_natural.lines().find( | l | l.contains( "abc" ) )
    .expect( "natural output must contain data row" );
  assert!(
    data_line.len() > data_line_nat.len(),
    "min_column_width(10) must produce wider output than natural:\n  natural line={data_line_nat:?}\n  floored line={data_line:?}",
  );

  // Both column values remain present
  assert!( out_min.contains( "abc" ), "floored output must contain 'abc':\n{out_min}" );
  assert!( out_min.contains( "xyz" ), "floored output must contain 'xyz':\n{out_min}" );
}

// FT-8: inner_padding applies symmetrically to both sides of every cell
#[ test ]
fn test_cell_padding_all_separators()
{
  // Two-column table with headers "col1" and "col2" and one data row.
  // All four inner_padding-enabled presets must produce spaces around every cell,
  // not just the outermost edges.
  let view = RowBuilder::new( vec![ "col1".into(), "col2".into() ] )
    .add_row( vec![ "a".into(), "b".into() ] )
    .build_view();

  // bordered and grid use ASCII pipe separators
  for ( label, config ) in [
    ( "bordered", TableConfig::bordered() ),
    ( "grid",     TableConfig::grid() ),
    ( "markdown", TableConfig::markdown() ),
  ]
  {
    let output = TableFormatter::with_config( config ).format( &view ).unwrap_or_default();
    // Header row must contain spaces on both sides of inter-cell separator
    assert!(
      output.contains( "| col1 | col2 |" ),
      "{label}: header row must contain '| col1 | col2 |'; output:\n{output}",
    );
  }

  // unicode_box uses box-drawing pipe character
  let output = TableFormatter::with_config( TableConfig::unicode_box() )
    .format( &view )
    .unwrap_or_default();
  assert!(
    output.contains( "│ col1 │ col2 │" ),
    "unicode_box: header row must contain '│ col1 │ col2 │'; output:\n{output}",
  );
}
