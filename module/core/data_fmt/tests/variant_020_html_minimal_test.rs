//! Variant 020: HTML Minimal spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, HtmlFormatter, HtmlVariant, Format };

fn html_formatter() -> HtmlFormatter
{
  HtmlFormatter::with_variant( HtmlVariant::Minimal )
}

/// VT-1: output is valid HTML table structure
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_020_vt_01_valid_html_table_structure()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format( &html_formatter(), &view ).unwrap();

  assert!( out.contains( "<table" ), "<table> element present" );
  assert!( out.contains( "<thead" ), "<thead> element present" );
  assert!( out.contains( "<tbody" ), "<tbody> element present" );
  assert!( out.contains( "<tr" ), "<tr> elements present" );
  assert!( out.contains( "<th" ), "<th> elements present" );
  assert!( out.contains( "<td" ), "<td> elements present" );
}

/// VT-2: header cells use th elements
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_020_vt_02_header_cells_use_th()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let out = Format::format( &html_formatter(), &view ).unwrap();

  // Headers use <th>, not <td>
  assert!( out.contains( "<th>" ) || out.contains( "<th " ), "header cells are <th>" );
  assert!( out.contains( "<td>" ) || out.contains( "<td " ), "data cells are <td>" );
}

/// VT-3: no CSS classes or framework-specific attributes
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_020_vt_03_no_css_classes()
{
  let view = RowBuilder::new( vec![ "X".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let out = Format::format( &html_formatter(), &view ).unwrap();

  assert!( !out.contains( "class=" ), "no CSS classes in minimal output" );
  assert!( !out.contains( "bootstrap" ), "no bootstrap" );
  assert!( !out.contains( "tailwind" ), "no tailwind" );
}

/// VT-4: empty table produces valid HTML
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_020_vt_04_empty_table_valid_html()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format( &html_formatter(), &view ).unwrap();

  assert!( out.contains( "<table" ), "table element present even when empty" );
  assert!( out.contains( "<thead" ), "thead present" );
}
