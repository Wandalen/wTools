//! Bug reproducer tests for corner-case bugs BUG-018 through BUG-022
//!
//! Each test documents a specific corner case discovered during manual testing,
//! provides a minimal reproduction, and verifies the fix.
//!
//! ## Bug Index
//!
//! - BUG-018: `TextFormatter` `KeyValue` trailing blank line
//! - BUG-019: `HtmlFormatter` empty Custom class emits `class=""`
//! - BUG-020: `SqlFormatter` zero-row table produces invalid SQL
//! - BUG-021: CSV values containing commas not quoted (RFC 4180)
//! - BUG-022: Markdown cells containing `|` not escaped

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, Format };

/// ## Root Cause
///
/// `format_key_value` unconditionally pushed `'\n'` after every record, producing
/// a trailing blank line (`"k: v\n\n"` instead of `"k: v\n"` for a single record).
///
/// ## Why Not Caught
///
/// Prior tests only checked `contains()` on fragments, not the exact trailing structure.
///
/// ## Fix Applied
///
/// Changed from "terminator after each" pattern to "separator between" pattern:
/// blank line inserted before each record except the first.
///
/// ## Prevention
///
/// Always test that single-record output ends with exactly one `\n` and no trailing blank.
///
/// ## Pitfall
///
/// Loop-and-append patterns should use separator-between logic, not terminator-after-each.
// test_kind: bug_reproducer(BUG-018)
#[ cfg( feature = "format_text" ) ]
#[ test ]
fn bug_018_keyvalue_trailing_blank_line()
{
  use data_fmt::TextFormatter;

  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();
  let formatter = TextFormatter::key_value();
  let output = formatter.format( &view ).unwrap();

  // Single record should end with exactly one newline, no trailing blank
  assert!(
    output.ends_with( "30\n" ),
    "single record must end with value + single newline, got: {output:?}",
  );
  assert!(
    !output.ends_with( "\n\n" ),
    "single record must NOT have trailing blank line, got: {output:?}",
  );

  // Multi-record: blank line separator between records, no trailing blank
  let view2 = RowBuilder::new( vec![ "K".into() ] )
    .add_row( vec![ "v1".into() ] )
    .add_row( vec![ "v2".into() ] )
    .build_view();
  let output2 = formatter.format( &view2 ).unwrap();

  assert!(
    output2.contains( "\n\nK: v2" ),
    "multi-record must have blank line separator, got: {output2:?}",
  );
  assert!(
    !output2.ends_with( "\n\n" ),
    "multi-record must NOT have trailing blank line, got: {output2:?}",
  );
}

/// ## Root Cause
///
/// `HtmlVariant::Custom("")` matched the `Custom(class) => Some(class.clone())` arm,
/// producing `class=""` in the `<table>` tag — semantically incorrect HTML.
///
/// ## Why Not Caught
///
/// Prior tests only used non-empty custom classes and the Minimal variant.
///
/// ## Fix Applied
///
/// Added a guard pattern `Custom(class) if class.is_empty() => None` before the
/// general `Custom(class)` arm.
///
/// ## Prevention
///
/// Test empty-string variants for all enum wrappers that produce attributes.
///
/// ## Pitfall
///
/// Always check for empty custom strings before emitting HTML attributes.
// test_kind: bug_reproducer(BUG-019)
#[ cfg( any( feature = "html_minimal", feature = "html_bootstrap", feature = "html_tailwind", feature = "html_custom" ) ) ]
#[ test ]
fn bug_019_html_empty_custom_class()
{
  use data_fmt::{ HtmlFormatter, HtmlVariant };

  let view = RowBuilder::new( vec![ "A".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  // Empty custom class should behave like Minimal (no class attr)
  let formatter = HtmlFormatter::with_variant( HtmlVariant::Custom( String::new() ) );
  let output = formatter.format( &view ).unwrap();

  assert!(
    !output.contains( "class=\"\"" ),
    "empty Custom class must NOT produce class=\"\", got: {output:?}",
  );
  assert!(
    output.starts_with( "<table>\n" ),
    "empty Custom class must produce bare <table> tag, got: {output:?}",
  );

  // Non-empty custom class should still work
  let formatter2 = HtmlFormatter::with_variant( HtmlVariant::Custom( "my-table".to_string() ) );
  let output2 = formatter2.format( &view ).unwrap();
  assert!(
    output2.contains( "class=\"my-table\"" ),
    "non-empty Custom class must produce class attr, got: {output2:?}",
  );
}

/// ## Root Cause
///
/// `SqlFormatter::format` always emitted `INSERT INTO ... VALUES;` even with zero rows,
/// producing invalid SQL (`VALUES;` without any row tuples).
///
/// ## Why Not Caught
///
/// Prior tests always included at least one data row.
///
/// ## Fix Applied
///
/// Early return `Ok(String::new())` when `data.rows.is_empty()`.
///
/// ## Prevention
///
/// Always test formatters with zero-row (headers-only) input.
///
/// ## Pitfall
///
/// Guard on rows, not columns — a headers-only table has nothing to insert.
// test_kind: bug_reproducer(BUG-020)
#[ cfg( feature = "format_sql" ) ]
#[ test ]
fn bug_020_sql_zero_rows_invalid_sql()
{
  use data_fmt::SqlFormatter;

  let view = RowBuilder::new( vec![ "name".into(), "age".into() ] )
    .build_view();
  let formatter = SqlFormatter::new( "users" );
  let output = formatter.format( &view ).unwrap();

  assert!(
    output.is_empty(),
    "zero-row SQL output must be empty string, got: {output:?}",
  );

  // With rows should still work normally
  let view2 = RowBuilder::new( vec![ "name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build_view();
  let output2 = formatter.format( &view2 ).unwrap();
  assert!(
    output2.contains( "VALUES" ),
    "non-empty SQL must contain VALUES, got: {output2:?}",
  );
}

/// ## Root Cause
///
/// CSV cell text containing commas was emitted raw without RFC 4180 quoting, making
/// in-value commas indistinguishable from column separators.
///
/// ## Why Not Caught
///
/// Prior CSV tests only used simple alphanumeric cell values.
///
/// ## Fix Applied
///
/// Added `csv_quote()` helper that wraps cells in double-quotes and doubles internal
/// `"` when text contains `,` or `"`. Applied after newline escaping.
///
/// ## Prevention
///
/// Always test CSV with values containing the separator character and double-quotes.
///
/// ## Pitfall
///
/// Apply quoting AFTER newline escaping. The escaped `\n` literal does not need
/// RFC 4180 quoting, but commas and double-quotes do.
// test_kind: bug_reproducer(BUG-021)
#[ test ]
fn bug_021_csv_comma_in_value_not_quoted()
{
  use data_fmt::{ TableFormatter, TableConfig };

  let view = RowBuilder::new( vec![ "Name".into(), "Address".into() ] )
    .add_row( vec![ "Alice".into(), "123 Main St, Apt 4".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::csv() );
  let output = formatter.format( &view ).unwrap();
  let lines : Vec< &str > = output.lines().collect();

  // Header line should not be quoted (no commas in headers)
  assert_eq!(
    lines[ 0 ], "Name,Address",
    "header line must be unquoted"
  );

  // Data line: address with comma must be RFC 4180 quoted
  assert_eq!(
    lines[ 1 ], "Alice,\"123 Main St, Apt 4\"",
    "cell with comma must be double-quoted per RFC 4180, got: {:?}",
    lines[ 1 ]
  );
}

/// Test CSV quoting of cells containing double-quotes (RFC 4180: double them)
// test_kind: bug_reproducer(BUG-021)
#[ test ]
fn bug_021_csv_double_quote_in_value()
{
  use data_fmt::{ TableFormatter, TableConfig };

  let view = RowBuilder::new( vec![ "Name".into(), "Quote".into() ] )
    .add_row( vec![ "Alice".into(), "She said \"hello\"".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::csv() );
  let output = formatter.format( &view ).unwrap();
  let lines : Vec< &str > = output.lines().collect();

  // Internal double-quotes must be doubled AND cell must be quoted
  assert_eq!(
    lines[ 1 ], "Alice,\"She said \"\"hello\"\"\"",
    "cell with double-quotes must be RFC 4180 escaped, got: {:?}",
    lines[ 1 ]
  );
}

/// Test that TSV values with commas are NOT quoted (only CSV uses RFC 4180)
// test_kind: bug_reproducer(BUG-021)
#[ test ]
fn bug_021_tsv_comma_not_quoted()
{
  use data_fmt::{ TableFormatter, TableConfig };

  let view = RowBuilder::new( vec![ "K".into() ] )
    .add_row( vec![ "a,b".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::tsv() );
  let output = formatter.format( &view ).unwrap();
  let lines : Vec< &str > = output.lines().collect();

  // TSV should NOT quote commas — they are not the separator
  assert_eq!(
    lines[ 1 ], "a,b",
    "TSV must NOT quote commas (comma is not the TSV separator), got: {:?}",
    lines[ 1 ]
  );
}

/// ## Root Cause
///
/// Markdown cell text containing `|` was emitted raw, indistinguishable from the
/// column separator pipe, producing extra columns and corrupting table structure.
///
/// ## Why Not Caught
///
/// Prior Markdown tests only used simple alphanumeric cell values.
///
/// ## Fix Applied
///
/// Added pipe escaping (`|` → `\|`) in the cell preparation phase of both
/// `format_row` and `format_row_colored` when Markdown mode is active.
///
/// ## Prevention
///
/// Always test Markdown tables with values containing the pipe character.
///
/// ## Pitfall
///
/// Only escape in Markdown mode — other table styles emit `|` as a border character
/// separately via the rendering pipeline, not embedded in cell text.
// test_kind: bug_reproducer(BUG-022)
#[ test ]
fn bug_022_markdown_pipe_in_cell_not_escaped()
{
  use data_fmt::{ TableFormatter, TableConfig };

  let view = RowBuilder::new( vec![ "Command".into(), "Description".into() ] )
    .add_row( vec![ "a | b".into(), "pipe in value".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::markdown() );
  let output = formatter.format( &view ).unwrap();

  // The pipe in "a | b" must be escaped as "a \| b"
  assert!(
    output.contains( r"a \| b" ),
    "pipe in Markdown cell must be escaped as \\|, got:\n{output}",
  );

  // Verify correct column count: header + separator + 1 data row = 3 lines
  let lines : Vec< &str > = output.lines().collect();
  assert_eq!(
    lines.len(), 3,
    "markdown table must have exactly 3 lines (header + separator + data), got {}",
    lines.len()
  );
}

/// Test Markdown pipe escaping in header cells
// test_kind: bug_reproducer(BUG-022)
#[ test ]
fn bug_022_markdown_pipe_in_header()
{
  use data_fmt::{ TableFormatter, TableConfig };

  let view = RowBuilder::new( vec![ "A|B".into(), "C".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::markdown() );
  let output = formatter.format( &view ).unwrap();

  // Header pipe must also be escaped
  assert!(
    output.contains( r"A\|B" ),
    "pipe in Markdown header must be escaped as \\|, got:\n{output}",
  );
}
