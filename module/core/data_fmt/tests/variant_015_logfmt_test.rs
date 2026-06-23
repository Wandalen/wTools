//! Variant 015: Logfmt Standard spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, LogfmtFormatter, Format };

/// VT-1: output is key=value pairs per line
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_015_vt_01_key_value_pairs_per_line()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format( &LogfmtFormatter::new(), &view ).unwrap();

  assert!( out.contains( "Name=Alice" ), "Name=Alice pair present" );
  assert!( out.contains( "Age=30" ), "Age=30 pair present" );
}

/// VT-2: pairs are space-separated
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_015_vt_02_pairs_space_separated()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into() ] )
    .add_row( vec![ "1".into(), "2".into(), "3".into() ] )
    .build_view();

  let out = Format::format( &LogfmtFormatter::new(), &view ).unwrap();
  let line = out.lines().next().unwrap_or( "" );

  assert!( line.contains( ' ' ), "space-separated pairs" );
  assert!( !line.contains( ',' ), "no comma separators" );
}

/// VT-3: values with spaces are quoted
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_015_vt_03_values_with_spaces_quoted()
{
  let view = RowBuilder::new( vec![ "Name".into(), "City".into() ] )
    .add_row( vec![ "Alice".into(), "New York".into() ] )
    .build_view();

  let out = Format::format( &LogfmtFormatter::new(), &view ).unwrap();

  // Value with space should be quoted
  assert!(
    out.contains( "City=\"New York\"" ),
    "space-containing value is quoted: {out}",
  );
  // Simple value should NOT be quoted
  assert!( out.contains( "Name=Alice" ), "simple value unquoted" );
}

/// VT-4: empty table produces empty output
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_015_vt_04_empty_table_empty_output()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format( &LogfmtFormatter::new(), &view ).unwrap();

  assert!( out.trim().is_empty(), "empty table produces empty output" );
}
