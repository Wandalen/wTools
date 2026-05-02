//! Tests for `indent_prefix` field on `ExpandedConfig`.
//! Task 016: Add `indent_prefix` to `ExpandedConfig`.

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig, Format };

/// T01: Property style default — no indent, lines start at column 0.
#[ test ]
fn test_expanded_indent_prefix_t01_property_default_no_indent()
{
  let tree = RowBuilder::new( vec![ "Key".into(), "Val".into() ] )
    .add_row( vec![ "alice".into(), "30".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config( ExpandedConfig::property_style() );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Lines must NOT start with whitespace
  for line in output.lines()
  {
    assert!
    (
      !line.starts_with( ' ' ),
      "default property_style should not indent, but got: {line:?}",
    );
  }
  assert!( output.contains( "Key" ) );
  assert!( output.contains( "alice" ) );
}

/// T02: Property style with 2-space indent — every key-value line starts with "  ".
#[ test ]
fn test_expanded_indent_prefix_t02_property_two_space_indent()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config
  (
    ExpandedConfig::property_style()
      .indent_prefix( "  ".into() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  for line in output.lines()
  {
    assert!
    (
      line.starts_with( "  " ),
      "each key-value line must start with 2-space indent, got: {line:?}",
    );
  }
  // property_style colorizes keys, so "Name" is wrapped in ANSI codes
  assert!( output.contains( "Name" ) );
  assert!( output.contains( "Alice" ) );
}

/// T03: Postgres style with indent — key-value lines indented, record separator NOT indented.
#[ test ]
fn test_expanded_indent_prefix_t03_postgres_indent_separator_unaffected()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Val".into() ] )
    .add_row( vec![ "a".into(), "1".into() ] )
    .add_row( vec![ "b".into(), "2".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config
  (
    ExpandedConfig::postgres_style()
      .indent_prefix( "  ".into() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  for line in output.lines()
  {
    if line.starts_with( "-[" )
    {
      // Record separator lines must NOT be indented
      assert!
      (
        !line.starts_with( "  " ),
        "record separator must not be indented, got: {line:?}",
      );
    }
    else
    {
      // Key-value lines must be indented
      assert!
      (
        line.starts_with( "  " ),
        "key-value line must start with 2-space indent, got: {line:?}",
      );
    }
  }
}

/// T04: Two records with custom prefix "> " — all key-value lines from both records prefixed.
#[ test ]
fn test_expanded_indent_prefix_t04_two_records_custom_prefix()
{
  let tree = RowBuilder::new( vec![ "K".into(), "V".into() ] )
    .add_row( vec![ "x".into(), "1".into() ] )
    .add_row( vec![ "y".into(), "2".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config
  (
    ExpandedConfig::property_style()
      .indent_prefix( "> ".into() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  let kv_lines : Vec< &str > = output.lines()
    .filter( | l | !l.is_empty() )
    .collect();

  assert!( kv_lines.len() >= 4, "expected at least 4 key-value lines, got {}", kv_lines.len() );

  for line in &kv_lines
  {
    assert!
    (
      line.starts_with( "> " ),
      "all kv lines must start with '> ', got: {line:?}",
    );
  }
}

/// T05: Empty data (no rows) with indent — must return empty string, no panic.
#[ test ]
fn test_expanded_indent_prefix_t05_empty_data_no_panic()
{
  let tree = RowBuilder::new( vec![ "A".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config
  (
    ExpandedConfig::property_style()
      .indent_prefix( "  ".into() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  assert!( output.trim().is_empty(), "empty data should produce empty output, got: {output:?}" );
}

/// T06: Explicit empty string `indent_prefix` — identical to default, no prefix.
#[ test ]
fn test_expanded_indent_prefix_t06_explicit_empty_string()
{
  let tree = RowBuilder::new( vec![ "Key".into() ] )
    .add_row( vec![ "val".into() ] )
    .build_view();

  let with_explicit_empty = ExpandedFormatter::with_config
  (
    ExpandedConfig::property_style()
      .indent_prefix( String::new() )
  );
  let without = ExpandedFormatter::with_config( ExpandedConfig::property_style() );

  let out_a = with_explicit_empty.format( &tree ).unwrap_or_default();
  let out_b = without.format( &tree ).unwrap_or_default();

  assert_eq!( out_a, out_b, "explicit empty prefix must be identical to default" );
}

/// T07: Colorize keys + indent — indent appears before ANSI color code on each line.
#[ test ]
fn test_expanded_indent_prefix_t07_colorize_keys_indent_before_color()
{
  let tree = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config
  (
    ExpandedConfig::property_style()
      .colorize_keys( true )
      .key_color( "\x1b[90m".into() )
      .indent_prefix( "  ".into() )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  for line in output.lines()
  {
    // Each line must start with indent, then ANSI color code
    assert!
    (
      line.starts_with( "  \x1b[" ),
      "indent must precede ANSI color code, got: {line:?}",
    );
  }
}
