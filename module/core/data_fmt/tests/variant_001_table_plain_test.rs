//! Variant 001: Table Plain spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

/// VT-1: output is space-separated with no borders
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_001_vt_01_space_separated_no_borders()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .add_row( vec![ "Bob".into(), "25".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::plain() ),
    &view,
  ).unwrap();

  assert!( !out.contains( '|' ), "no pipe characters in plain output" );
  assert!( !out.contains( '+' ), "no plus characters in plain output" );
  assert!( !out.contains( '┌' ), "no box-drawing characters" );
  assert!( out.contains( "Alice" ), "output contains data" );
}

/// VT-2: column separator is double space
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_001_vt_02_column_separator_double_space()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::plain() ),
    &view,
  ).unwrap();

  assert!( !out.contains( '|' ), "no pipe separator" );
  assert!( !out.contains( '\t' ), "no tab separator" );
  // Columns have at least 2 spaces between them
  assert!( out.contains( "  " ), "double-space gap between columns" );
}

/// VT-3: header separator is dashes
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_001_vt_03_header_separator_dashes()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::plain() ),
    &view,
  ).unwrap();

  let has_dash_line = out.lines().any( | line | line.contains( "---" ) );
  assert!( has_dash_line, "dash separator line between header and data" );
}

/// VT-4: empty table produces minimal output
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_001_vt_04_empty_table_minimal()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::plain() ),
    &view,
  ).unwrap();

  assert!( !out.contains( "---" ) || out.lines().count() <= 3, "no excessive trailing separators" );
}
