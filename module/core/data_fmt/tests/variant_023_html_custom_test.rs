//! Variant 023: HTML Custom spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, HtmlFormatter, HtmlVariant, Format };

/// VT-1: custom CSS class applied to table element
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_023_vt_01_custom_class_applied()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build_view();

  let out = Format::format(
    &HtmlFormatter::with_variant( HtmlVariant::Custom( "my-table dark-theme".into() ) ),
    &view,
  ).unwrap();

  assert!( out.contains( "my-table" ), "custom class applied: {out}" );
  assert!( out.contains( "dark-theme" ), "custom class applied: {out}" );
}

/// VT-2: valid HTML table structure
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_023_vt_02_valid_html_structure()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  let out = Format::format(
    &HtmlFormatter::with_variant( HtmlVariant::Custom( "custom".into() ) ),
    &view,
  ).unwrap();

  assert!( out.contains( "<thead>" ), "thead: {out}" );
  assert!( out.contains( "<tbody>" ), "tbody: {out}" );
  assert!( out.contains( "<th>" ), "th: {out}" );
  assert!( out.contains( "<td>" ), "td: {out}" );
}

/// VT-3: user-provided class string appears verbatim
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_023_vt_03_verbatim_class_string()
{
  let custom_class = "data-grid responsive shadow-lg";
  let view = RowBuilder::new( vec![ "X".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let out = Format::format(
    &HtmlFormatter::with_variant( HtmlVariant::Custom( custom_class.into() ) ),
    &view,
  ).unwrap();

  assert!( out.contains( custom_class ), "exact class string in output: {out}" );
}

/// VT-4: empty table produces valid HTML with custom class
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_023_vt_04_empty_table_custom_class()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &HtmlFormatter::with_variant( HtmlVariant::Custom( "empty-table".into() ) ),
    &view,
  ).unwrap();

  assert!( out.contains( "empty-table" ), "custom class on empty table: {out}" );
  assert!( out.contains( "<table" ), "table element present: {out}" );
}
