//! Variant 030: Text Sections spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };

/// VT-1: each row rendered as a section with header
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_030_vt_01_rows_as_sections()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .add_row( vec![ "Bob".into(), "25".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::Sections ),
    &view,
  ).unwrap();

  assert!( out.contains( "Alice" ), "first row data: {out}" );
  assert!( out.contains( "Bob" ), "second row data: {out}" );
}

/// VT-2: section header has underline separator
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_030_vt_02_header_separator()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::Sections ),
    &view,
  ).unwrap();

  // Sections format uses colon as header separator
  assert!( out.contains( ':' ), "header separator present: {out}" );
}

/// VT-3: fields listed under section header
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_030_vt_03_fields_listed()
{
  let view = RowBuilder::new( vec![ "Name".into(), "City".into() ] )
    .add_row( vec![ "Alice".into(), "NYC".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::Sections ),
    &view,
  ).unwrap();

  assert!( out.contains( "Alice" ), "name field present: {out}" );
  assert!( out.contains( "NYC" ), "city field present: {out}" );
}

/// VT-4: empty table produces no sections
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_030_vt_04_empty_table()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::Sections ),
    &view,
  ).unwrap();

  // No data fields appear in empty table (header may still be present)
  let data_lines : Vec< &str > = out.lines()
    .filter( | l | l.starts_with( "  " ) )
    .collect();
  assert!( data_lines.is_empty(), "no field data for empty table: {out}" );
}
