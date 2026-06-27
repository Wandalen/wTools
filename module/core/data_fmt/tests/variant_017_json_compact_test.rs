//! Variant 017: JSON Compact spec tests (VT-1..VT-4)

#![ cfg( feature = "format_json" ) ]

use data_fmt::{ RowBuilder, JsonFormatter, Format };

/// VT-1: output is valid parseable JSON
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_017_vt_01_valid_parseable_json()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format( &JsonFormatter::compact(), &view ).unwrap();
  let _parsed : serde_json::Value = serde_json::from_str( &out )
    .expect( "compact output must be valid JSON" );
}

/// VT-2: output is single-line with minimal whitespace
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_017_vt_02_single_line_minimal()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let compact = Format::format( &JsonFormatter::compact(), &view ).unwrap();
  let pretty = Format::format( &JsonFormatter::new(), &view ).unwrap();

  // Compact should be smaller than pretty
  assert!(
    compact.len() < pretty.len(),
    "compact ({}) must be smaller than pretty ({})",
    compact.len(), pretty.len(),
  );
}

/// VT-3: compact and pretty produce equivalent data
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_017_vt_03_compact_pretty_equivalent()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let compact = Format::format( &JsonFormatter::compact(), &view ).unwrap();
  let pretty = Format::format( &JsonFormatter::new(), &view ).unwrap();

  let parsed_compact : serde_json::Value = serde_json::from_str( &compact ).unwrap();
  let parsed_pretty : serde_json::Value = serde_json::from_str( &pretty ).unwrap();

  assert_eq!( parsed_compact, parsed_pretty, "same data in compact and pretty" );
}

/// VT-4: empty table produces valid compact JSON
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_017_vt_04_empty_table_valid()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format( &JsonFormatter::compact(), &view ).unwrap();
  let _parsed : serde_json::Value = serde_json::from_str( &out )
    .expect( "empty table compact JSON must parse" );
}
