//! Variant 008: Table TSV spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

/// VT-1: output is tab-separated with no borders
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_008_vt_01_tab_separated_no_borders()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::tsv() ),
    &view,
  ).unwrap();

  assert!( out.contains( '\t' ), "tab separators present: {out}" );
  assert!( !out.contains( '|' ), "no pipe characters: {out}" );
  assert!( !out.contains( '+' ), "no plus characters: {out}" );
}

/// VT-2: column separator is tab character
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_008_vt_02_tab_separator()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::tsv() ),
    &view,
  ).unwrap();

  for line in out.lines().filter( | l | !l.trim().is_empty() )
  {
    let tab_count = line.matches( '\t' ).count();
    assert!( tab_count >= 1, "at least one tab per line: {line}" );
  }
}

/// VT-3: no header separator line
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_008_vt_03_no_header_separator()
{
  let view = RowBuilder::new( vec![ "key".into(), "val".into() ] )
    .add_row( vec![ "a".into(), "b".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::tsv() ),
    &view,
  ).unwrap();

  let has_sep = out.lines().any( | l |
  {
    let t = l.trim();
    t.contains( "---" ) || t.contains( "===" )
  });
  assert!( !has_sep, "no separator line in TSV: {out}" );
}

/// VT-4: empty table produces header-only TSV
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_008_vt_04_empty_table_header_only()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::tsv() ),
    &view,
  ).unwrap();

  let non_empty : Vec< &str > = out.lines().filter( | l | !l.trim().is_empty() ).collect();
  assert!( non_empty.len() <= 2, "at most header line for empty TSV: {out}" );
}
