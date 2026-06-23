//! `YamlFormatter` spec tests (FM-22..FM-26, file-scoped IDs)

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "format_yaml" ) ]

use data_fmt::{ RowBuilder, YamlFormatter, Format };

/// FM-22: standard output produces YAML sequence of mappings
// test_kind: spec_case(FM-22)
#[ test ]
fn formatter_006_fm_22_standard_output_produces_yaml_sequence_of_mappings()
{
  let view = RowBuilder::new( vec![ "name".into(), "age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let formatter = YamlFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "name" ), "should contain key 'name'" );
  assert!( output.contains( "Alice" ), "should contain value 'Alice'" );
}

/// FM-23: special characters are YAML-safe
// test_kind: spec_case(FM-23)
#[ test ]
fn formatter_006_fm_23_special_characters_are_yaml_safe()
{
  let view = RowBuilder::new( vec![ "text".into() ] )
    .add_row( vec![ "colon: here\nnewline".into() ] )
    .build_view();

  let formatter = YamlFormatter::new();
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "special chars should not cause error" );
  let output = result.unwrap();
  assert!( output.contains( "colon" ), "should contain the text 'colon'" );
}

/// FM-24: Format trait dispatch returns well-formed string
// test_kind: spec_case(FM-24)
#[ test ]
fn formatter_006_fm_24_format_trait_dispatch_returns_well_formed_string()
{
  let view = RowBuilder::new( vec![ "a".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let formatter = YamlFormatter::new();
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "format should return Ok" );
}

/// FM-25: empty data produces empty YAML sequence
// test_kind: spec_case(FM-25)
#[ test ]
fn formatter_006_fm_25_empty_data_produces_empty_yaml_sequence()
{
  let view = RowBuilder::new( vec![ "col".into() ] )
    .build_view();

  let formatter = YamlFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  // Empty YAML sequence: either "[]" or blank/whitespace-only
  let trimmed = output.trim();
  assert!(
    trimmed == "[]" || trimmed.is_empty(),
    "empty data should produce empty YAML sequence, got: '{trimmed}'",
  );
}

/// FM-26: multi-row input produces one mapping per row
// test_kind: spec_case(FM-26)
#[ test ]
fn formatter_006_fm_26_multi_row_input_produces_one_mapping_per_row()
{
  let view = RowBuilder::new( vec![ "k".into() ] )
    .add_row( vec![ "v1".into() ] )
    .add_row( vec![ "v2".into() ] )
    .add_row( vec![ "v3".into() ] )
    .build_view();

  let formatter = YamlFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "v1" ), "should contain v1" );
  assert!( output.contains( "v2" ), "should contain v2" );
  assert!( output.contains( "v3" ), "should contain v3" );
}
