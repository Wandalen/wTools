//! Variant 010: Expanded Postgres spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig, Format };

/// VT-1: vertical record layout with one field per line
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_010_vt_01_vertical_record_layout()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &ExpandedFormatter::with_config( ExpandedConfig::postgres_style() ),
    &view,
  ).unwrap();

  // Each field on its own line — "Name" and "Age" appear as labels
  assert!( out.contains( "Name" ), "Name label present" );
  assert!( out.contains( "Age" ), "Age label present" );
  // Multiple lines
  assert!( out.lines().count() > 1, "multi-line vertical layout" );
}

/// VT-2: field label and value separated by pipe
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_010_vt_02_field_label_pipe_separator()
{
  let view = RowBuilder::new( vec![ "key".into(), "val".into() ] )
    .add_row( vec![ "a".into(), "b".into() ] )
    .build_view();

  let out = Format::format(
    &ExpandedFormatter::with_config( ExpandedConfig::postgres_style() ),
    &view,
  ).unwrap();

  assert!( out.contains( '|' ), "pipe separator between label and value" );
}

/// VT-3: record separator between multiple records
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_010_vt_03_record_separator()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .add_row( vec![ "Bob".into() ] )
    .build_view();

  let out = Format::format(
    &ExpandedFormatter::with_config( ExpandedConfig::postgres_style() ),
    &view,
  ).unwrap();

  // Record separator: "-[ RECORD N ]" format
  assert!( out.contains( "-[ RECORD 1 ]" ), "RECORD 1 separator present: {out}" );
  assert!( out.contains( "-[ RECORD 2 ]" ), "RECORD 2 separator present: {out}" );
  assert!( out.contains( "Alice" ), "first record present" );
  assert!( out.contains( "Bob" ), "second record present" );
}

/// VT-4: empty table produces no records
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_010_vt_04_empty_table_no_records()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &ExpandedFormatter::with_config( ExpandedConfig::postgres_style() ),
    &view,
  ).unwrap();

  assert!(
    out.trim().is_empty() || !out.contains( '|' ),
    "no record blocks for empty table",
  );
}
