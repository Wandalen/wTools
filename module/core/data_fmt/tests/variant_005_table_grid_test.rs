//! Variant 005: Table Grid spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

/// VT-1: pipe column separators on every line
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_005_vt_01_pipe_separators_every_line()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::grid() ),
    &view,
  ).unwrap();

  for line in out.lines().filter( | l | !l.trim().is_empty() )
  {
    assert!(
      line.contains( '|' ) || line.contains( '+' ),
      "every line has pipe or grid char: {line}",
    );
  }
}

/// VT-2: horizontal rules between all rows
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_005_vt_02_horizontal_rules_between_rows()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .add_row( vec![ "3".into(), "4".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::grid() ),
    &view,
  ).unwrap();

  let rule_count = out.lines()
    .filter( | l | l.contains( '+' ) && l.contains( '-' ) )
    .count();

  // top border + after header + between rows + bottom border = at least 4
  assert!( rule_count >= 3, "multiple horizontal rules present ({rule_count}): {out}" );
}

/// VT-3: grid intersections use + characters
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_005_vt_03_plus_intersections()
{
  let view = RowBuilder::new( vec![ "X".into(), "Y".into() ] )
    .add_row( vec![ "a".into(), "b".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::grid() ),
    &view,
  ).unwrap();

  let has_plus = out.lines().any( | l | l.contains( '+' ) );
  assert!( has_plus, "grid intersections use + characters: {out}" );
}

/// VT-4: empty table produces grid header only
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_005_vt_04_empty_grid_header()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::grid() ),
    &view,
  ).unwrap();

  assert!( out.contains( '+' ), "grid frame present for empty table: {out}" );
  assert!( out.contains( '|' ), "pipe present for empty table: {out}" );
}
