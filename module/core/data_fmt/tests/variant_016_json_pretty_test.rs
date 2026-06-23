//! Variant 016: JSON Pretty spec tests (VT-1..VT-4)

#![ cfg( feature = "format_json" ) ]

use data_fmt::{ RowBuilder, JsonFormatter, Format };

/// VT-1: output is valid parseable JSON
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_016_vt_01_valid_parseable_json()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format( &JsonFormatter::new(), &view ).unwrap();
  let parsed : serde_json::Value = serde_json::from_str( &out )
    .expect( "output must be valid JSON" );

  assert!( parsed.is_array() || parsed.is_object(), "JSON structure present" );
}

/// VT-2: output is indented with newlines
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_016_vt_02_indented_with_newlines()
{
  let view = RowBuilder::new( vec![ "key".into(), "val".into() ] )
    .add_row( vec![ "a".into(), "b".into() ] )
    .build_view();

  let out = Format::format( &JsonFormatter::new(), &view ).unwrap();

  assert!( out.lines().count() > 1, "multi-line output" );
  assert!( out.contains( "  " ), "indentation present" );
}

/// VT-3: special characters are backslash-escaped
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_016_vt_03_special_chars_escaped()
{
  let view = RowBuilder::new( vec![ "msg".into() ] )
    .add_row( vec![ "hello \"world\"".into() ] )
    .build_view();

  let out = Format::format( &JsonFormatter::new(), &view ).unwrap();

  // Must be valid JSON (which requires proper escaping)
  let _parsed : serde_json::Value = serde_json::from_str( &out )
    .expect( "escaped output must be valid JSON" );
  assert!( out.contains( "\\\"" ), "quotes are backslash-escaped" );
}

/// VT-4: empty table produces valid JSON structure
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_016_vt_04_empty_table_valid_json()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format( &JsonFormatter::new(), &view ).unwrap();
  let _parsed : serde_json::Value = serde_json::from_str( &out )
    .expect( "empty table JSON must parse" );
}
