//! `TomlFormatter` spec tests (FM-27..FM-31, file-scoped IDs)

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "format_toml" ) ]

use data_fmt::{ RowBuilder, TomlFormatter, Format };

/// FM-27: standard output produces TOML array of inline tables
// test_kind: spec_case(FM-27)
#[ test ]
fn formatter_007_fm_27_standard_output_produces_toml_array_of_inline_tables()
{
  let view = RowBuilder::new( vec![ "name".into(), "age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let formatter = TomlFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "name" ), "should contain key 'name'" );
  assert!( output.contains( "Alice" ), "should contain value 'Alice'" );
}

/// FM-28: special characters are TOML-escaped
// test_kind: spec_case(FM-28)
#[ test ]
fn formatter_007_fm_28_special_characters_are_toml_escaped()
{
  let view = RowBuilder::new( vec![ "text".into() ] )
    .add_row( vec![ "line1\nline2\ttab".into() ] )
    .build_view();

  let formatter = TomlFormatter::new();
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "special chars should not cause error" );
}

/// FM-29: Format trait dispatch returns well-formed string
// test_kind: spec_case(FM-29)
#[ test ]
fn formatter_007_fm_29_format_trait_dispatch_returns_well_formed_string()
{
  let view = RowBuilder::new( vec![ "a".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let formatter = TomlFormatter::new();
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "format should return Ok" );
}

/// FM-30: empty data produces empty TOML array
// test_kind: spec_case(FM-30)
#[ test ]
fn formatter_007_fm_30_empty_data_produces_empty_toml_array()
{
  let view = RowBuilder::new( vec![ "col".into() ] )
    .build_view();

  let formatter = TomlFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  let trimmed = output.trim();
  // Empty TOML: no inline tables
  assert!(
    !trimmed.contains( '{' ),
    "empty data should not contain inline tables, got: '{trimmed}'",
  );
}

/// FM-31: multi-row input produces one inline table per row
// test_kind: spec_case(FM-31)
#[ test ]
fn formatter_007_fm_31_multi_row_input_produces_one_inline_table_per_row()
{
  let view = RowBuilder::new( vec![ "k".into() ] )
    .add_row( vec![ "v1".into() ] )
    .add_row( vec![ "v2".into() ] )
    .add_row( vec![ "v3".into() ] )
    .build_view();

  let formatter = TomlFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "v1" ), "should contain v1" );
  assert!( output.contains( "v2" ), "should contain v2" );
  assert!( output.contains( "v3" ), "should contain v3" );
}
