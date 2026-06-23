//! Variant 006: Table Unicode Box spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

/// VT-1: output uses Unicode box-drawing characters
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_006_vt_01_unicode_box_drawing()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::unicode_box() ),
    &view,
  ).unwrap();

  let has_box_chars = out.chars().any( | c | ('\u{2500}'..='\u{257F}').contains( &c ) );
  assert!( has_box_chars, "Unicode box-drawing characters present: {out}" );
  assert!( !out.contains( '+' ), "no ASCII + in Unicode box output: {out}" );
}

/// VT-2: column separator is U+2502
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_006_vt_02_column_separator_u2502()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::unicode_box() ),
    &view,
  ).unwrap();

  assert!( out.contains( '\u{2502}' ), "U+2502 vertical line present: {out}" );
  // Data lines should not use ASCII pipe
  let data_lines : Vec< &str > = out.lines()
    .filter( | l | l.contains( 'x' ) || l.contains( 'A' ) )
    .collect();
  for line in &data_lines
  {
    assert!( !line.contains( '|' ), "no ASCII pipe on data line: {line}" );
  }
}

/// VT-3: header separator uses Unicode horizontal lines
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_006_vt_03_unicode_header_separator()
{
  let view = RowBuilder::new( vec![ "X".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::unicode_box() ),
    &view,
  ).unwrap();

  assert!( out.contains( '\u{2500}' ), "U+2500 horizontal line present: {out}" );
  // No ASCII dashes in separator lines
  let sep_lines : Vec< &str > = out.lines()
    .filter( | l | l.contains( '\u{2500}' ) )
    .collect();
  for line in &sep_lines
  {
    assert!( !line.contains( '-' ), "no ASCII dash in Unicode separator: {line}" );
  }
}

/// VT-4: empty table produces box-drawing header only
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_006_vt_04_empty_unicode_box()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::unicode_box() ),
    &view,
  ).unwrap();

  let has_box = out.chars().any( | c | ('\u{2500}'..='\u{257F}').contains( &c ) );
  assert!( has_box, "box-drawing frame present for empty table: {out}" );
}
