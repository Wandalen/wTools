//! Variant 018: YAML Standard spec tests (VT-1..VT-4)

#![ cfg( feature = "format_yaml" ) ]

use data_fmt::{ RowBuilder, YamlFormatter, Format };

/// VT-1: output is valid parseable YAML
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_018_vt_01_valid_parseable_yaml()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format( &YamlFormatter::new(), &view ).unwrap();
  let parsed : serde_yaml_ng::Value = serde_yaml_ng::from_str( &out )
    .expect( "output must be valid YAML" );

  assert!( parsed.is_sequence() || parsed.is_mapping(), "YAML structure present" );
}

/// VT-2: indentation-based nesting structure
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_018_vt_02_indentation_based_nesting()
{
  let view = RowBuilder::new( vec![ "key".into(), "val".into() ] )
    .add_row( vec![ "a".into(), "b".into() ] )
    .build_view();

  let out = Format::format( &YamlFormatter::new(), &view ).unwrap();

  assert!( out.lines().count() > 1, "multi-line output" );
  assert!( !out.contains( '{' ), "no brace-based nesting" );
  assert!( out.contains( '-' ), "YAML list items with dash" );
}

/// VT-3: header names used as keys
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_018_vt_03_header_names_as_keys()
{
  let view = RowBuilder::new( vec![ "Name".into(), "City".into() ] )
    .add_row( vec![ "Alice".into(), "NYC".into() ] )
    .build_view();

  let out = Format::format( &YamlFormatter::new(), &view ).unwrap();

  assert!( out.contains( "Name" ), "Name key present" );
  assert!( out.contains( "City" ), "City key present" );
  assert!( out.contains( "Alice" ), "Alice value present" );
  assert!( out.contains( "NYC" ), "NYC value present" );
}

/// VT-4: empty table produces valid YAML
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_018_vt_04_empty_table_valid_yaml()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format( &YamlFormatter::new(), &view ).unwrap();
  let _parsed : serde_yaml_ng::Value = serde_yaml_ng::from_str( &out )
    .expect( "empty table YAML must parse" );
}
