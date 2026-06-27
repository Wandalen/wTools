//! Variant 021: HTML Bootstrap spec tests (VT-1..VT-4)

#![ cfg( all( feature = "enabled", feature = "html_bootstrap" ) ) ]

use data_fmt::{ RowBuilder, HtmlFormatter, HtmlVariant, Format };

/// VT-1: output contains Bootstrap CSS classes
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_021_vt_01_bootstrap_css_classes()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &HtmlFormatter::with_variant( HtmlVariant::Bootstrap ),
    &view,
  ).unwrap();

  assert!( out.contains( "table" ), "Bootstrap class present: {out}" );
  assert!( out.contains( "class=" ), "class attribute present: {out}" );
}

/// VT-2: valid HTML table structure
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_021_vt_02_valid_html_structure()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  let out = Format::format(
    &HtmlFormatter::with_variant( HtmlVariant::Bootstrap ),
    &view,
  ).unwrap();

  assert!( out.contains( "<thead>" ), "thead present: {out}" );
  assert!( out.contains( "<tbody>" ), "tbody present: {out}" );
  assert!( out.contains( "<th>" ), "th elements present: {out}" );
  assert!( out.contains( "<td>" ), "td elements present: {out}" );
}

/// VT-3: table element has Bootstrap class attribute
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_021_vt_03_bootstrap_class_attribute()
{
  let view = RowBuilder::new( vec![ "X".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let out = Format::format(
    &HtmlFormatter::with_variant( HtmlVariant::Bootstrap ),
    &view,
  ).unwrap();

  // Bootstrap classes should include "table" keyword
  assert!( out.contains( "class=\"" ), "class attribute with quotes: {out}" );
  // No Tailwind-specific classes
  assert!( !out.contains( "min-w-full" ), "no Tailwind classes: {out}" );
}

/// VT-4: empty table produces valid Bootstrap HTML
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_021_vt_04_empty_table_valid()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &HtmlFormatter::with_variant( HtmlVariant::Bootstrap ),
    &view,
  ).unwrap();

  assert!( out.contains( "<table" ), "table element present: {out}" );
  assert!( out.contains( "<thead>" ), "thead present for empty table: {out}" );
  assert!( out.contains( "class=\"" ), "Bootstrap classes on empty table: {out}" );
}
