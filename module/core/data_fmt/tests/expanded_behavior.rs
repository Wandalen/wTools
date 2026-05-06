//! Comprehensive behavior tests for `ExpandedFormatter` and `ExpandedConfig`.
//!
//! Covers all corner cases identified in manual testing:
//! - `show_record_numbers` field behavior (bug reproducer)
//! - Alignment correctness with and without `indent_prefix`
//! - `write_to()` parity with `format()`
//! - Record separator edge cases
//! - Property style blank-line spacing
//! - Unicode and tab prefix handling
//!
//! ## Test Matrix
//!
//! | # | Scenario | Config | Expected |
//! |---|----------|--------|----------|
//! | A1 | show_record_numbers(false) | postgres_style + false | Separator has no row number |
//! | A2 | show_record_numbers(true) | postgres_style + true | Separator has row number |
//! | A3 | show_record_numbers(false), empty sep | record_separator("") + false | No separator, no effect |
//! | A4 | show_record_numbers(false), sep without `{}` | "---" + false | Separator outputs verbatim |
//! | A5 | show_record_numbers(false) + indent_prefix | postgres + indent + no-num | kv indented, sep has no number |
//! | B1 | BeforeSeparator alignment with indent | indent_prefix + keys of varying width | Keys still right-aligned |
//! | B2 | AfterSeparator alignment with indent | property_style + indent | Values still aligned |
//! | B3 | write_to() matches format() | indent_prefix set | Byte-identical output |
//! | B4 | Empty cell value with indent | indent_prefix + empty value | Prefix still on line |
//! | C1 | Blank line between property records | property_style + 2 records | Blank between, not before first |
//! | C2 | No separator multiple records | empty sep + 3 records | Blank lines between only |
//! | C3 | Single record, empty sep | empty sep + 1 record | No blank line at all |
//! | D1 | Separator without `{}` | "---" template | Output verbatim |
//! | D2 | Single-column alignment | 1 header | No padding applied |
//! | D3 | All keys same width | identical width keys | No extra padding |
//! | E1 | Tab character as indent_prefix | "\t" prefix | Tab at start of each kv line |
//! | E2 | Unicode indent_prefix | "→ " prefix | Lines start with that string |

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig, Format };

// ── A: show_record_numbers behavior ──────────────────────────────────────────

/// A1: `show_record_numbers(false)` must suppress the row number from the separator.
///
/// ## Bug Reproducer
///
/// **Root Cause**: `show_record_numbers` field was stored in `ExpandedConfig` but
/// never read in `ExpandedFormatter::format()`. The method always called
/// `record_separator.replace("{}", record_name)` regardless of the flag, so setting
/// `show_record_numbers(false)` had zero effect on output.
///
/// **Why Not Caught**: The only existing test (`test_expanded_config_builder_methods`)
/// verified the builder set the struct field but did NOT verify the formatter output.
/// A config-only test cannot catch a formatter that ignores a field.
///
/// **Fix Applied**: `format()` now passes `""` as the replacement when
/// `show_record_numbers` is `false`, so `"-[ RECORD {} ]"` becomes `"-[ RECORD  ]"`.
///
/// **Prevention**: For every config field, write both (a) a builder/struct test and
/// (b) a formatter-output test confirming the field actually changes rendered output.
///
/// **Pitfall**: Config builder tests that only check struct fields are insufficient —
/// they cannot detect that a formatter silently ignores a field.
#[ test ]
fn test_expanded_show_record_numbers_false_suppresses_number()
{
  let tree = RowBuilder::new( vec![ "Key".into() ] )
    .add_row( vec![ "v1".into() ] )
    .add_row( vec![ "v2".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .show_record_numbers( false )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // No row numbers in separator lines
  assert!
  (
    !output.contains( "-[ RECORD 1 ]" ),
    "show_record_numbers(false) must suppress number '1' from separator; got:\n{output}",
  );
  assert!
  (
    !output.contains( "-[ RECORD 2 ]" ),
    "show_record_numbers(false) must suppress number '2' from separator; got:\n{output}",
  );
  // Separator still appears (not suppressed entirely)
  assert!
  (
    output.contains( "-[ RECORD" ),
    "show_record_numbers(false) must keep the separator text (just drop the number); got:\n{output}",
  );
}

/// A2: `show_record_numbers(true)` (default) keeps row numbers in separator.
#[ test ]
fn test_expanded_show_record_numbers_true_keeps_number()
{
  let tree = RowBuilder::new( vec![ "Key".into() ] )
    .add_row( vec![ "v1".into() ] )
    .add_row( vec![ "v2".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .show_record_numbers( true )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!
  (
    output.contains( "-[ RECORD 1 ]" ),
    "show_record_numbers(true) must keep number in separator; got:\n{output}",
  );
  assert!
  (
    output.contains( "-[ RECORD 2 ]" ),
    "show_record_numbers(true) must keep number in separator; got:\n{output}",
  );
}

/// A3: `show_record_numbers(false)` with empty separator — separator is already
/// disabled, flag has no effect; no panic, no separator lines.
#[ test ]
fn test_expanded_show_record_numbers_false_empty_separator_no_effect()
{
  let tree = RowBuilder::new( vec![ "Key".into() ] )
    .add_row( vec![ "v1".into() ] )
    .add_row( vec![ "v2".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .record_separator( String::new() )
      .show_record_numbers( false )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!
  (
    !output.contains( "RECORD" ),
    "empty separator must produce no separator lines; got:\n{output}",
  );
}

/// A4: `show_record_numbers(false)` with separator containing no `{}` placeholder —
/// separator is output verbatim, flag has no visible effect (nothing to strip).
#[ test ]
fn test_expanded_show_record_numbers_false_separator_without_placeholder()
{
  let tree = RowBuilder::new( vec![ "Key".into() ] )
    .add_row( vec![ "v1".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .record_separator( "---".to_string() )
      .show_record_numbers( false )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!
  (
    output.contains( "---" ),
    "separator without {{}} should be output verbatim; got:\n{output}",
  );
}

/// A5: `show_record_numbers(false)` + `indent_prefix` — kv lines indented, separator
/// has no number and is NOT indented.
#[ test ]
fn test_expanded_show_record_numbers_false_with_indent_prefix()
{
  let tree = RowBuilder::new( vec![ "K".into() ] )
    .add_row( vec![ "v".into() ] )
    .add_row( vec![ "w".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .show_record_numbers( false )
      .indent_prefix( "  ".into() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  for line in output.lines()
  {
    if line.starts_with( "-[" )
    {
      // Separator: not indented, no number
      assert!(
        !line.starts_with( "  " ),
        "separator must not be indented; got: {line:?}"
      );
      assert!(
        !line.contains( '1' ) && !line.contains( '2' ),
        "separator must have no number when show_record_numbers=false; got: {line:?}"
      );
    }
    else
    {
      // Key-value: indented
      assert!(
        line.starts_with( "  " ),
        "kv line must be indented; got: {line:?}"
      );
    }
  }
}

// ── B: alignment correctness ──────────────────────────────────────────────────

/// B1: `BeforeSeparator` alignment is preserved when `indent_prefix` is set.
/// Keys should still be right-padded to align the separator column.
#[ test ]
fn test_expanded_before_separator_alignment_with_indent()
{
  let tree = RowBuilder::new( vec![ "A".into(), "LongKey".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .indent_prefix( ">> ".into() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Key "A" (1 char) padded to width of "LongKey" (7 chars) = 6 extra spaces
  assert!
  (
    output.contains( ">> A       |" ),
    "short key must be padded to align with longest key (indent included); got:\n{output}",
  );
  assert!
  (
    output.contains( ">> LongKey |" ),
    "longest key should have no extra padding; got:\n{output}",
  );
}

/// B2: `AfterSeparator` alignment preserved when `indent_prefix` is set.
#[ test ]
fn test_expanded_after_separator_alignment_with_indent()
{
  let tree = RowBuilder::new( vec![ "A".into(), "LongKey".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .indent_prefix( "  ".into() )
      .key_value_separator( ": ".to_string() )
      .colorize_keys( false )
      .padding_side( data_fmt::PaddingSide::AfterSeparator )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // "A" (1 char) gets padding = max_key_width(7) - 1 = 6, plus 1 trailing sep space = 7 spaces
  assert!
  (
    output.contains( "  A:       x" ),
    "short key with AfterSeparator must be padded; got:\n{output}",
  );
  assert!
  (
    output.contains( "  LongKey: y" ),
    "longest key with AfterSeparator must have 1 space (from sep only); got:\n{output}",
  );
}

/// B3: `write_to()` produces byte-identical output to `format()` when `indent_prefix` set.
#[ test ]
fn test_expanded_write_to_matches_format_with_indent()
{
  use std::io::Cursor;

  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::property_style()
      .indent_prefix( "  ".into() )
  );

  let via_format = Format::format( &formatter, &view ).unwrap();

  let mut buf = Cursor::new( Vec::new() );
  formatter.write_to( &view, &mut buf ).unwrap();
  let via_write = String::from_utf8( buf.into_inner() ).unwrap();

  assert_eq!
  (
    via_format,
    via_write,
    "write_to() must produce identical output to format() when indent_prefix is set",
  );
}

/// B4: Empty cell value with `indent_prefix` — the prefix and key still appear on the line.
#[ test ]
fn test_expanded_indent_prefix_with_empty_cell_value()
{
  let tree = RowBuilder::new( vec![ "Key".into() ] )
    .add_row( vec![ "".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .indent_prefix( "  ".into() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Line must start with indent and contain the key
  assert!
  (
    output.lines().any( | l | l.starts_with( "  Key" ) ),
    "empty value line must still have indent prefix; got:\n{output}",
  );
}

// ── C: spacing and separator edge cases ──────────────────────────────────────

/// C1: Property style inserts exactly one blank line BETWEEN records but NOT before
/// the first record and NOT after the last record.
#[ test ]
fn test_expanded_property_style_blank_line_between_records_only()
{
  let tree = RowBuilder::new( vec![ "K".into() ] )
    .add_row( vec![ "a".into() ] )
    .add_row( vec![ "b".into() ] )
    .add_row( vec![ "c".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config( ExpandedConfig::property_style() );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Must NOT start with a blank line
  assert!
  (
    !output.starts_with( '\n' ),
    "output must not start with blank line; got:\n{output:?}",
  );
  // Must NOT end with double newline (extra blank line at end)
  assert!
  (
    !output.ends_with( "\n\n" ),
    "output must not end with extra blank line; got:\n{output:?}",
  );
  // Must have exactly 2 blank lines (between 3 records = 2 gaps)
  let blank_line_count = output.lines().filter( | l | l.trim().is_empty() ).count();
  assert_eq!
  (
    blank_line_count,
    2,
    "3 property-style records must produce exactly 2 blank separator lines; got {blank_line_count}; output:\n{output}",
  );
}

/// C2: No separator + multiple records → blank lines between records (not before first).
#[ test ]
fn test_expanded_no_separator_blank_lines_between_records()
{
  let tree = RowBuilder::new( vec![ "K".into() ] )
    .add_row( vec![ "a".into() ] )
    .add_row( vec![ "b".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .record_separator( String::new() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!(
    !output.starts_with( '\n' ),
    "output must not start with blank line; got:\n{output:?}",
  );
  let blank_line_count = output.lines().filter( | l | l.trim().is_empty() ).count();
  assert_eq!(
    blank_line_count, 1,
    "2 records with no separator must have exactly 1 blank line between them; got {blank_line_count}",
  );
}

/// C3: Single record with empty separator — no blank lines, no separator headers.
#[ test ]
fn test_expanded_single_record_empty_separator_no_blanks()
{
  let tree = RowBuilder::new( vec![ "K".into() ] )
    .add_row( vec![ "v".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .record_separator( String::new() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  let blank_line_count = output.lines().filter( | l | l.trim().is_empty() ).count();
  assert_eq!(
    blank_line_count, 0,
    "single record with empty separator must have no blank lines; got {blank_line_count}",
  );
}

// ── D: alignment edge cases ───────────────────────────────────────────────────

/// D1: Separator without `{}` placeholder is output verbatim (no number injected).
#[ test ]
fn test_expanded_separator_without_placeholder_verbatim()
{
  let tree = RowBuilder::new( vec![ "K".into() ] )
    .add_row( vec![ "v1".into() ] )
    .add_row( vec![ "v2".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .record_separator( "---".to_string() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Both record separators are "---" (no number)
  let sep_count = output.lines().filter( | l | *l == "---" ).count();
  assert_eq!(
    sep_count, 2,
    "separator without {{}} must appear verbatim for each record; got {sep_count} times; output:\n{output}",
  );
}

/// D2: Single-column table — no padding applied (only column so `max_key_width` == `key_width`).
#[ test ]
fn test_expanded_single_column_no_padding()
{
  let tree = RowBuilder::new( vec![ "Title".into() ] )
    .add_row( vec![ "Hello".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // "Title" should appear without trailing spaces before separator
  assert!
  (
    output.contains( "Title |" ),
    "single column must output key without extra padding; got:\n{output}",
  );
}

/// D3: All keys same width — no extra padding on any key.
#[ test ]
fn test_expanded_all_same_width_keys_no_extra_padding()
{
  let tree = RowBuilder::new( vec![ "Aaa".into(), "Bbb".into(), "Ccc".into() ] )
    .add_row( vec![ "1".into(), "2".into(), "3".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // All keys are 3 chars — no trailing spaces before separator
  assert!( output.contains( "Aaa |" ), "got:\n{output}" );
  assert!( output.contains( "Bbb |" ), "got:\n{output}" );
  assert!( output.contains( "Ccc |" ), "got:\n{output}" );
}

// ── E: prefix edge cases ──────────────────────────────────────────────────────

/// E1: Tab character as `indent_prefix` — each kv line starts with a tab.
#[ test ]
fn test_expanded_tab_indent_prefix()
{
  let tree = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .indent_prefix( "\t".into() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  for line in output.lines()
  {
    if !line.starts_with( "-[" )
    {
      assert!
      (
        line.starts_with( '\t' ),
        "tab prefix: kv line must start with tab; got: {line:?}",
      );
    }
  }
}

/// E2: Unicode `indent_prefix` ("→ ") — each kv line starts with that exact string.
#[ test ]
fn test_expanded_unicode_indent_prefix()
{
  let tree = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config(
    ExpandedConfig::new()
      .indent_prefix( "→ ".into() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  for line in output.lines()
  {
    if !line.starts_with( "-[" )
    {
      assert!
      (
        line.starts_with( "→ " ),
        "unicode prefix: kv line must start with '→ '; got: {line:?}",
      );
    }
  }
}
