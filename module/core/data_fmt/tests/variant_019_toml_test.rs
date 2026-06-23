//! Variant 019: TOML Standard spec tests (VT-1..VT-4)

#![ cfg( feature = "format_toml" ) ]

use data_fmt::{ RowBuilder, TomlFormatter, Format };

/// VT-1: output is valid parseable TOML
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_019_vt_01_valid_parseable_toml()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format( &TomlFormatter::new(), &view ).unwrap();

  // Deserialize via typed wrapper matching the [[row]] array-of-tables structure
  #[ derive( serde::Deserialize ) ]
  struct Wrapper
  {
    #[ allow( dead_code ) ]
    row : Vec< std::collections::HashMap< String, String > >,
  }
  let _parsed : Wrapper = toml::from_str( &out )
    .expect( "output must be valid TOML" );
}

/// VT-2: rows use array-of-tables notation
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_019_vt_02_array_of_tables_notation()
{
  let view = RowBuilder::new( vec![ "key".into(), "val".into() ] )
    .add_row( vec![ "a".into(), "1".into() ] )
    .add_row( vec![ "b".into(), "2".into() ] )
    .build_view();

  let out = Format::format( &TomlFormatter::new(), &view ).unwrap();

  // Array-of-tables uses [[...]] notation
  assert!( out.contains( "[[" ), "array-of-tables bracket notation" );
}

/// VT-3: header names become TOML keys
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_019_vt_03_header_names_as_keys()
{
  let view = RowBuilder::new( vec![ "Name".into(), "City".into() ] )
    .add_row( vec![ "Alice".into(), "NYC".into() ] )
    .build_view();

  let out = Format::format( &TomlFormatter::new(), &view ).unwrap();

  assert!( out.contains( "Name" ), "Name key present" );
  assert!( out.contains( "City" ), "City key present" );
  assert!( out.contains( "Alice" ), "Alice value present" );
}

/// VT-4: empty table produces valid TOML
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_019_vt_04_empty_table_valid_toml()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format( &TomlFormatter::new(), &view ).unwrap();

  // Either empty or valid TOML structure
  #[ derive( serde::Deserialize ) ]
  struct Wrapper
  {
    #[ allow( dead_code ) ]
    row : Vec< std::collections::HashMap< String, String > >,
  }
  if !out.trim().is_empty()
  {
    let _parsed : Wrapper = toml::from_str( &out )
      .expect( "empty table TOML must parse" );
  }
}
