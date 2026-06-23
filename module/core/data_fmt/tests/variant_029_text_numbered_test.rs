//! Variant 029: Text Numbered spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };

/// VT-1: each row has sequential number prefix
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_029_vt_01_sequential_number_prefix()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .add_row( vec![ "Bob".into() ] )
    .add_row( vec![ "Charlie".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::Numbered ),
    &view,
  ).unwrap();

  assert!( out.contains( "1." ), "first number prefix: {out}" );
  assert!( out.contains( "2." ), "second number prefix: {out}" );
  assert!( out.contains( "3." ), "third number prefix: {out}" );
}

/// VT-2: numbering starts at 1
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_029_vt_02_starts_at_one()
{
  let view = RowBuilder::new( vec![ "X".into() ] )
    .add_row( vec![ "value".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::Numbered ),
    &view,
  ).unwrap();

  assert!( out.contains( "1." ), "starts at 1: {out}" );
  assert!( !out.contains( "0." ), "no zero-based numbering: {out}" );
}

/// VT-3: multi-column rows include all fields
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_029_vt_03_multi_column_all_fields()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::Numbered ),
    &view,
  ).unwrap();

  assert!( out.contains( "Alice" ), "name field present: {out}" );
  assert!( out.contains( "30" ), "age field present: {out}" );
}

/// VT-4: empty table produces no numbered items
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_029_vt_04_empty_table()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::Numbered ),
    &view,
  ).unwrap();

  assert!( out.is_empty() || out.trim().is_empty(), "no numbered items: {out}" );
}
