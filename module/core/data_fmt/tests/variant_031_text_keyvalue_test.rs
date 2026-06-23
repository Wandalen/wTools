//! Variant 031: Text `KeyValue` spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };

/// VT-1: fields rendered as key: value pairs
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_031_vt_01_key_value_pairs()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::KeyValue ),
    &view,
  ).unwrap();

  assert!( out.contains( "Name: Alice" ), "name key-value: {out}" );
  assert!( out.contains( "Age: 30" ), "age key-value: {out}" );
}

/// VT-2: colon separator between key and value
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_031_vt_02_colon_separator()
{
  let view = RowBuilder::new( vec![ "key".into(), "val".into() ] )
    .add_row( vec![ "a".into(), "b".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::KeyValue ),
    &view,
  ).unwrap();

  assert!( out.contains( ':' ), "colon separator present: {out}" );
  assert!( !out.contains( '|' ), "no pipe separator: {out}" );
  assert!( !out.contains( '\t' ), "no tab separator: {out}" );
  assert!( !out.contains( '=' ), "no equals separator: {out}" );
}

/// VT-3: multiple rows produce separate record blocks
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_031_vt_03_separate_blocks()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .add_row( vec![ "Bob".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::KeyValue ),
    &view,
  ).unwrap();

  assert!( out.contains( "Alice" ), "first record: {out}" );
  assert!( out.contains( "Bob" ), "second record: {out}" );
  // Records are separated by blank lines
  assert!( out.contains( "\n\n" ), "blank line separation: {out}" );
}

/// VT-4: empty table produces no key-value output
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_031_vt_04_empty_table()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::KeyValue ),
    &view,
  ).unwrap();

  assert!( out.is_empty() || out.trim().is_empty(), "no key-value output: {out}" );
}
