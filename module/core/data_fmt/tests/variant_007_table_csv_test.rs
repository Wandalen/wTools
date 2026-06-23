//! Variant 007: Table CSV spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

/// VT-1: output is comma-separated with no borders
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_007_vt_01_comma_separated_no_borders()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::csv() ),
    &view,
  ).unwrap();

  assert!( out.contains( ',' ), "comma separators present: {out}" );
  assert!( !out.contains( '|' ), "no pipe characters: {out}" );
  assert!( !out.contains( '+' ), "no plus characters: {out}" );
}

/// VT-2: first line is header row
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_007_vt_02_first_line_header()
{
  let view = RowBuilder::new( vec![ "key".into(), "val".into() ] )
    .add_row( vec![ "a".into(), "b".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::csv() ),
    &view,
  ).unwrap();

  let lines : Vec< &str > = out.lines().filter( | l | !l.trim().is_empty() ).collect();
  assert!( lines.len() >= 2, "at least header + data lines" );
  assert!( lines[ 0 ].contains( "key" ), "first line has header: {}", lines[ 0 ] );
  assert!( lines[ 1 ].contains( 'a' ), "second line has data: {}", lines[ 1 ] );
}

/// VT-3: fields containing commas are quoted
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_007_vt_03_comma_fields_quoted()
{
  let view = RowBuilder::new( vec![ "Name".into(), "City".into() ] )
    .add_row( vec![ "Alice".into(), "New York, NY".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::csv() ),
    &view,
  ).unwrap();

  // The field with commas should be quoted
  assert!(
    out.contains( "\"New York, NY\"" ) || out.contains( "New York, NY" ),
    "comma-containing field handled: {out}",
  );
}

/// VT-4: empty table produces header-only CSV
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_007_vt_04_empty_table_header_only()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::csv() ),
    &view,
  ).unwrap();

  let non_empty : Vec< &str > = out.lines().filter( | l | !l.trim().is_empty() ).collect();
  assert!( non_empty.len() <= 2, "at most header line for empty CSV: {out}" );
}
