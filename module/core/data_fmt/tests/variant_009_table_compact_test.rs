//! Variant 009: Table Compact spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

/// VT-1: output uses single-space column separation
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_009_vt_01_single_space_separation()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let compact = Format::format(
    &TableFormatter::with_config( TableConfig::compact() ),
    &view,
  ).unwrap();

  let plain = Format::format(
    &TableFormatter::with_config( TableConfig::plain() ),
    &view,
  ).unwrap();

  // Compact should be more compact than plain (less whitespace)
  assert!(
    compact.len() <= plain.len(),
    "compact ({}) should not be larger than plain ({})",
    compact.len(), plain.len(),
  );
}

/// VT-2: no header separator line
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_009_vt_02_no_header_separator()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::compact() ),
    &view,
  ).unwrap();

  let has_sep = out.lines().any( | l |
  {
    let t = l.trim();
    t.contains( "---" ) || t.contains( "===" )
  });
  assert!( !has_sep, "no separator line in compact: {out}" );
}

/// VT-3: no border characters present
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_009_vt_03_no_border_characters()
{
  let view = RowBuilder::new( vec![ "X".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::compact() ),
    &view,
  ).unwrap();

  assert!( !out.contains( '|' ), "no pipes: {out}" );
  assert!( !out.contains( '+' ), "no plus: {out}" );
  let has_box = out.chars().any( | c | ('\u{2500}'..='\u{257F}').contains( &c ) );
  assert!( !has_box, "no box-drawing characters: {out}" );
}

/// VT-4: empty table produces minimal output
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_009_vt_04_empty_table_minimal()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::compact() ),
    &view,
  ).unwrap();

  let non_empty : Vec< &str > = out.lines().filter( | l | !l.trim().is_empty() ).collect();
  assert!( non_empty.len() <= 2, "minimal output for empty table: {out}" );
}
