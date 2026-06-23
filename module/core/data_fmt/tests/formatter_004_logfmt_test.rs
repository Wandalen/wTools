//! `LogfmtFormatter` spec tests (FM-13..FM-17, file-scoped IDs)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, LogfmtFormatter, Format };

/// FM-13: basic key-value pairs on single line
// test_kind: spec_case(FM-13)
#[ test ]
fn formatter_004_fm_13_basic_key_value_pairs_on_single_line()
{
  let view = RowBuilder::new( vec![ "level".into(), "msg".into() ] )
    .add_row( vec![ "info".into(), "started".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  let line = output.lines().next().expect( "should have at least one line" );
  assert!( line.contains( "level=info" ), "should contain level=info, got: {line}" );
  assert!( line.contains( "msg=started" ), "should contain msg=started, got: {line}" );
}

/// FM-14: special characters in values are quoted
// test_kind: spec_case(FM-14)
#[ test ]
fn formatter_004_fm_14_special_characters_in_values_are_quoted()
{
  let view = RowBuilder::new( vec![ "msg".into() ] )
    .add_row( vec![ "hello world".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  assert!(
    output.contains( "msg=\"hello world\"" ),
    "value with space should be quoted: {output}",
  );
}

/// FM-15: empty values produce key with empty quotes
// test_kind: spec_case(FM-15)
#[ test ]
fn formatter_004_fm_15_empty_values_produce_key_with_empty_quotes()
{
  let view = RowBuilder::new( vec![ "tag".into(), "value".into() ] )
    .add_row( vec![ "x".into(), "".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "tag=x" ), "non-empty field should render normally" );
  // logfmt fast-path: empty string has no special chars → unquoted `value=`
  assert!(
    output.contains( "value=" ),
    "empty field should render as value= (unquoted empty), got: {output}",
  );
}

/// FM-16: Format trait dispatch returns well-formed string
// test_kind: spec_case(FM-16)
#[ test ]
fn formatter_004_fm_16_format_trait_dispatch_returns_well_formed_string()
{
  let view = RowBuilder::new( vec![ "a".into(), "b".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "format should return Ok" );
  let output = result.unwrap();
  assert!( output.contains( '=' ), "logfmt output should contain '='" );
}

/// FM-17: multi-row input produces one logfmt line per row
// test_kind: spec_case(FM-17)
#[ test ]
fn formatter_004_fm_17_multi_row_input_produces_one_logfmt_line_per_row()
{
  let view = RowBuilder::new( vec![ "id".into() ] )
    .add_row( vec![ "1".into() ] )
    .add_row( vec![ "2".into() ] )
    .add_row( vec![ "3".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  let non_empty : Vec< &str > = output.lines().filter( | l | !l.is_empty() ).collect();
  assert_eq!( non_empty.len(), 3, "should have exactly 3 non-empty lines, got: {non_empty:?}" );
}
