//! Variant 002: Table Minimal spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

/// VT-1: output has no borders and no header separator
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_002_vt_01_no_borders_no_header_separator()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::minimal() ),
    &view,
  ).unwrap();

  assert!( !out.contains( '|' ), "no pipe characters: {out}" );
  assert!( !out.contains( '+' ), "no plus characters: {out}" );
  assert!( !out.contains( "---" ), "no dash separator line: {out}" );
  assert!( out.contains( "Alice" ), "data present" );
}

/// VT-2: column separator is double space
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_002_vt_02_column_separator_double_space()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::minimal() ),
    &view,
  ).unwrap();

  assert!( out.contains( "  " ), "double-space gap between columns: {out}" );
}

/// VT-3: no separator line between header and data
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_002_vt_03_no_separator_between_header_and_data()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::minimal() ),
    &view,
  ).unwrap();

  let non_empty : Vec< &str > = out.lines().filter( | l | !l.trim().is_empty() ).collect();
  assert_eq!( non_empty.len(), 2, "exactly header + data line: {out}" );
}

/// VT-4: empty table produces minimal output
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_002_vt_04_empty_table_minimal()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::minimal() ),
    &view,
  ).unwrap();

  assert!( !out.contains( "---" ), "no separator lines for empty table: {out}" );
}
