//! `JsonFormatter` spec tests (FM-17..FM-21, file-scoped IDs)

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "format_json" ) ]

use data_fmt::{ RowBuilder, JsonFormatter, Format };

/// FM-17: pretty print produces indented JSON array
// test_kind: spec_case(FM-17)
#[ test ]
fn formatter_005_fm_17_pretty_print_produces_indented_json_array()
{
  let view = RowBuilder::new( vec![ "name".into(), "age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let formatter = JsonFormatter::new(); // default is pretty
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( '\n' ), "pretty JSON should be multi-line" );
  assert!( output.contains( "Alice" ), "should contain 'Alice'" );
  assert!( output.contains( "name" ), "should contain key 'name'" );
  // Validate JSON by checking bracket structure
  assert!( output.trim().starts_with( '[' ), "should start with '['" );
  assert!( output.trim().ends_with( ']' ), "should end with ']'" );
}

/// FM-18: compact mode produces single-line JSON
// test_kind: spec_case(FM-18)
#[ test ]
fn formatter_005_fm_18_compact_mode_produces_single_line_json()
{
  let view = RowBuilder::new( vec![ "k".into() ] )
    .add_row( vec![ "v1".into() ] )
    .add_row( vec![ "v2".into() ] )
    .build_view();

  let formatter = JsonFormatter::compact();
  let output = Format::format( &formatter, &view ).unwrap();

  // Compact should be on one line (no intermediate newlines within JSON structure)
  let trimmed = output.trim();
  assert!( !trimmed[ 1..trimmed.len() - 1 ].contains( '\n' ), "compact JSON should not have intermediate newlines" );
  assert!( trimmed.starts_with( '[' ) && trimmed.ends_with( ']' ), "should be a JSON array" );
}

/// FM-19: special characters are JSON-escaped
// test_kind: spec_case(FM-19)
#[ test ]
fn formatter_005_fm_19_special_characters_are_json_escaped()
{
  let value = "line1\nline2 \"quoted\" path\\dir";
  let view = RowBuilder::new( vec![ "text".into() ] )
    .add_row( vec![ value.into() ] )
    .build_view();

  let formatter = JsonFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  // Escaped forms in JSON output
  assert!( output.contains( "\\n" ), "newline should be escaped as \\n" );
  assert!( output.contains( "\\\"" ), "double quote should be escaped as \\\"" );
  assert!( output.contains( "\\\\" ), "backslash should be escaped as \\\\" );
}

/// FM-20: Format trait dispatch returns well-formed string
// test_kind: spec_case(FM-20)
#[ test ]
fn formatter_005_fm_20_format_trait_dispatch_returns_well_formed_string()
{
  let view = RowBuilder::new( vec![ "a".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let formatter = JsonFormatter::new();
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "format should return Ok" );
  let output = result.unwrap();
  assert!( output.contains( "\"a\"" ), "should have key 'a'" );
  assert!( output.contains( "\"1\"" ), "should have value '1'" );
}

/// FM-21: empty data produces empty JSON array
// test_kind: spec_case(FM-21)
#[ test ]
fn formatter_005_fm_21_empty_data_produces_empty_json_array()
{
  let view = RowBuilder::new( vec![ "col".into() ] )
    .build_view();

  let formatter = JsonFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  assert_eq!( output.trim(), "[]", "empty data should produce empty JSON array" );
}
