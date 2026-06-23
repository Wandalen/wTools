//! Variant 003: Table Bordered spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

/// VT-1: output has pipe column separators
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_003_vt_01_pipe_column_separators()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::bordered() ),
    &view,
  ).unwrap();

  let data_lines : Vec< &str > = out.lines()
    .filter( | l | l.contains( "Alice" ) || l.contains( "Name" ) )
    .collect();

  for line in &data_lines
  {
    assert!( line.contains( '|' ), "pipe separator on data line: {line}" );
  }
}

/// VT-2: ASCII grid header separator present
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_003_vt_02_ascii_grid_header_separator()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::bordered() ),
    &view,
  ).unwrap();

  let has_separator = out.lines().any( | l | l.contains( '+' ) && l.contains( '-' ) );
  assert!( has_separator, "ASCII grid separator with + and - present: {out}" );
}

/// VT-3: outer border lines surround the table
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_003_vt_03_outer_border_lines()
{
  let view = RowBuilder::new( vec![ "X".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::bordered() ),
    &view,
  ).unwrap();

  let lines : Vec< &str > = out.lines().filter( | l | !l.trim().is_empty() ).collect();
  assert!( lines.len() >= 3, "at least top border + header + bottom border" );

  let first = lines.first().unwrap();
  let last = lines.last().unwrap();
  assert!( first.contains( '+' ) || first.contains( '-' ), "first line is border: {first}" );
  assert!( last.contains( '+' ) || last.contains( '-' ), "last line is border: {last}" );
}

/// VT-4: empty table produces bordered header only
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_003_vt_04_empty_bordered_header()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::bordered() ),
    &view,
  ).unwrap();

  assert!( out.contains( '|' ), "bordered frame present even for empty table: {out}" );
}
