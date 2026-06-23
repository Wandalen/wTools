//! Variant 011: Expanded Property spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig, Format };

/// VT-1: vertical layout with colon separator
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_011_vt_01_vertical_colon_separator()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &ExpandedFormatter::with_config( ExpandedConfig::property_style() ),
    &view,
  ).unwrap();

  // Property style uses colon-like separator, not pipe
  assert!( out.contains( "Name" ), "Name label present: {out}" );
  assert!( out.contains( "Alice" ), "Alice value present: {out}" );
  assert!( !out.contains( '|' ), "no pipe separator in property style: {out}" );
}

/// VT-2: no record header line
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_011_vt_02_no_record_header()
{
  let view = RowBuilder::new( vec![ "key".into(), "val".into() ] )
    .add_row( vec![ "a".into(), "b".into() ] )
    .build_view();

  let out = Format::format(
    &ExpandedFormatter::with_config( ExpandedConfig::property_style() ),
    &view,
  ).unwrap();

  assert!( !out.contains( "RECORD" ), "no RECORD header in property style: {out}" );
  assert!( !out.contains( "-[" ), "no dash-bracket header: {out}" );
}

/// VT-3: multiple records separated by blank line
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_011_vt_03_records_separated()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .add_row( vec![ "Bob".into() ] )
    .build_view();

  let out = Format::format(
    &ExpandedFormatter::with_config( ExpandedConfig::property_style() ),
    &view,
  ).unwrap();

  assert!( out.contains( "Alice" ), "first record present: {out}" );
  assert!( out.contains( "Bob" ), "second record present: {out}" );
  // Records should be visually separated
  assert!( out.lines().count() > 2, "multiple lines for multiple records: {out}" );
}

/// VT-4: empty table produces no output
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_011_vt_04_empty_table_no_output()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &ExpandedFormatter::with_config( ExpandedConfig::property_style() ),
    &view,
  ).unwrap();

  assert!(
    out.trim().is_empty(),
    "empty table produces no output in property style: '{out}'",
  );
}
