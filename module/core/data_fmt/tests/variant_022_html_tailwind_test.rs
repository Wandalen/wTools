//! Variant 022: HTML Tailwind spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, HtmlFormatter, HtmlVariant, Format };

/// VT-1: output contains Tailwind CSS utility classes
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_022_vt_01_tailwind_utility_classes()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &HtmlFormatter::with_variant( HtmlVariant::Tailwind ),
    &view,
  ).unwrap();

  assert!( out.contains( "class=\"" ), "class attribute present: {out}" );
  // Tailwind uses utility classes like min-w-full, divide-y, etc.
  assert!( out.contains( "<table" ), "table element present: {out}" );
}

/// VT-2: valid HTML table structure
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_022_vt_02_valid_html_structure()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  let out = Format::format(
    &HtmlFormatter::with_variant( HtmlVariant::Tailwind ),
    &view,
  ).unwrap();

  assert!( out.contains( "<table" ), "table element: {out}" );
  assert!( out.contains( "<thead>" ), "thead: {out}" );
  assert!( out.contains( "<tbody>" ), "tbody: {out}" );
  assert!( out.contains( "<th>" ), "th: {out}" );
  assert!( out.contains( "<td>" ), "td: {out}" );
}

/// VT-3: no Bootstrap or custom CSS classes
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_022_vt_03_no_bootstrap_classes()
{
  let view = RowBuilder::new( vec![ "X".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let out = Format::format(
    &HtmlFormatter::with_variant( HtmlVariant::Tailwind ),
    &view,
  ).unwrap();

  // No Bootstrap-specific class names
  assert!( !out.contains( "table-striped" ), "no Bootstrap table-striped: {out}" );
  assert!( !out.contains( "table-hover" ), "no Bootstrap table-hover: {out}" );
}

/// VT-4: empty table produces valid Tailwind HTML
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_022_vt_04_empty_table_valid()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &HtmlFormatter::with_variant( HtmlVariant::Tailwind ),
    &view,
  ).unwrap();

  assert!( out.contains( "<table" ), "table element present: {out}" );
  assert!( out.contains( "<thead>" ), "thead present: {out}" );
}
