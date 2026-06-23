//! `ExpandedFormatter` spec tests (FM-5..FM-9, file-scoped IDs)

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "format_expanded" ) ]

use data_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig, Format };

/// FM-5: postgres style renders record header and fields
// test_kind: spec_case(FM-5)
#[ test ]
fn formatter_002_fm_05_postgres_style_renders_record_header_and_fields()
{
  let view = RowBuilder::new( vec![ "name".into(), "age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config( ExpandedConfig::postgres_style() );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "RECORD" ), "postgres style should have RECORD header" );
  assert!( output.contains( "name" ), "should contain field name 'name'" );
  assert!( output.contains( "Alice" ), "should contain value 'Alice'" );
  assert!( output.contains( '|' ) || output.contains( ':' ), "fields should be delimited" );
}

/// FM-6: property style renders dotted key-value lines
// test_kind: spec_case(FM-6)
#[ test ]
fn formatter_002_fm_06_property_style_renders_key_value_lines()
{
  let view = RowBuilder::new( vec![ "host".into(), "port".into() ] )
    .add_row( vec![ "localhost".into(), "8080".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config( ExpandedConfig::property_style() );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "host" ), "should contain key 'host'" );
  assert!( output.contains( "localhost" ), "should contain value 'localhost'" );
  assert!( !output.contains( "RECORD" ), "property style should not have RECORD banner" );
}

/// FM-7: Format trait dispatch returns well-formed string
// test_kind: spec_case(FM-7)
#[ test ]
fn formatter_002_fm_07_format_trait_dispatch_returns_well_formed_string()
{
  let view = RowBuilder::new( vec![ "k".into() ] )
    .add_row( vec![ "v1".into() ] )
    .add_row( vec![ "v2".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config( ExpandedConfig::postgres_style() );
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "format should return Ok" );
  let output = result.unwrap();
  assert!( output.contains( "v1" ), "should contain v1" );
  assert!( output.contains( "v2" ), "should contain v2" );
}

/// FM-8: empty data produces no records
// test_kind: spec_case(FM-8)
#[ test ]
fn formatter_002_fm_08_empty_data_produces_no_records()
{
  let view = RowBuilder::new( vec![ "a".into(), "b".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::new();
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "empty data should not error" );
  let output = result.unwrap();
  assert!( !output.contains( "RECORD" ), "no records should be emitted" );
}

/// FM-9: multi-row input produces numbered record separators
// test_kind: spec_case(FM-9)
#[ test ]
fn formatter_002_fm_09_multi_row_input_produces_numbered_record_separators()
{
  let view = RowBuilder::new( vec![ "x".into() ] )
    .add_row( vec![ "1".into() ] )
    .add_row( vec![ "2".into() ] )
    .add_row( vec![ "3".into() ] )
    .build_view();

  let formatter = ExpandedFormatter::with_config( ExpandedConfig::postgres_style() );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "RECORD 1" ), "should have RECORD 1" );
  assert!( output.contains( "RECORD 2" ), "should have RECORD 2" );
  assert!( output.contains( "RECORD 3" ), "should have RECORD 3" );
}
