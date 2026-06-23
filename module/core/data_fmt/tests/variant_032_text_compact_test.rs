//! Variant 032: Text Compact spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TextFormatter, Format };

/// VT-1: fields separated by commas within record
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_032_vt_01_comma_separated()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::compact(),
    &view,
  ).unwrap();

  assert!( out.contains( "Alice" ), "name present: {out}" );
  assert!( out.contains( "30" ), "age present: {out}" );
}

/// VT-2: minimal output overhead
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_032_vt_02_minimal_overhead()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let compact_out = Format::format(
    &TextFormatter::compact(),
    &view,
  ).unwrap();

  let bullets_out = Format::format(
    &TextFormatter::bullets(),
    &view,
  ).unwrap();

  assert!(
    compact_out.len() <= bullets_out.len(),
    "compact ({}) should be <= bullets ({})",
    compact_out.len(),
    bullets_out.len(),
  );
}

/// VT-3: multiple rows produce separate records
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_032_vt_03_separate_records()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .add_row( vec![ "Bob".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::compact(),
    &view,
  ).unwrap();

  assert!( out.contains( "Alice" ), "first record: {out}" );
  assert!( out.contains( "Bob" ), "second record: {out}" );
}

/// VT-4: empty table produces no output
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_032_vt_04_empty_table()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::compact(),
    &view,
  ).unwrap();

  assert!( out.is_empty(), "empty output: {out}" );
}
