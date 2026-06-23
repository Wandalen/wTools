//! `HtmlFormatter` spec tests (FM-32..FM-38, file-scoped IDs)

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "format_html" ) ]

use data_fmt::{ RowBuilder, HtmlFormatter, HtmlVariant, Format };

/// FM-32: minimal variant produces bare HTML table
// test_kind: spec_case(FM-32)
#[ test ]
fn formatter_008_fm_32_minimal_variant_produces_bare_html_table()
{
  let view = RowBuilder::new( vec![ "name".into(), "age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let formatter = HtmlFormatter::with_variant( HtmlVariant::Minimal );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "<table>" ), "minimal variant should produce bare <table> without class" );
  assert!( output.contains( "<thead>" ), "should contain <thead>" );
  assert!( output.contains( "<tbody>" ), "should contain <tbody>" );
  assert!( output.contains( "<th>" ), "should contain <th> elements" );
  assert!( output.contains( "<td>" ), "should contain <td> elements" );
  assert!( !output.contains( "class=" ), "minimal variant should have no CSS class attributes" );
}

/// FM-33: bootstrap variant adds Bootstrap CSS classes
// test_kind: spec_case(FM-33)
#[ test ]
fn formatter_008_fm_33_bootstrap_variant_adds_bootstrap_css_classes()
{
  let view = RowBuilder::new( vec![ "x".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let formatter = HtmlFormatter::with_variant( HtmlVariant::Bootstrap );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "class=\"table" ), "bootstrap variant should contain 'class=\"table'" );
  assert!( output.contains( "<table" ), "output should be valid HTML with <table>" );
}

/// FM-34: tailwind variant adds Tailwind CSS classes
// test_kind: spec_case(FM-34)
#[ test ]
fn formatter_008_fm_34_tailwind_variant_adds_tailwind_css_classes()
{
  let view = RowBuilder::new( vec![ "x".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let formatter = HtmlFormatter::with_variant( HtmlVariant::Tailwind );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "min-w-full" ), "tailwind variant should contain Tailwind classes" );
  assert!( output.contains( "<table" ), "output should contain <table> element" );
}

/// FM-35: custom variant injects user-provided CSS classes
// test_kind: spec_case(FM-35)
#[ test ]
fn formatter_008_fm_35_custom_variant_injects_user_provided_css_classes()
{
  let view = RowBuilder::new( vec![ "x".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let formatter = HtmlFormatter::with_table_class( "my-table striped" );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!(
    output.contains( "class=\"my-table striped\"" ),
    "custom class string should appear verbatim in class attribute",
  );
}

/// FM-36: Format trait dispatch returns well-formed string
// test_kind: spec_case(FM-36)
#[ test ]
fn formatter_008_fm_36_format_trait_dispatch_returns_well_formed_string()
{
  let view = RowBuilder::new( vec![ "a".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let formatter = HtmlFormatter::new();
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "format should return Ok" );
  let output = result.unwrap();
  assert!( output.contains( "<table" ), "should contain valid HTML table element" );
}

/// FM-37: empty data produces header-only HTML table
// test_kind: spec_case(FM-37)
#[ test ]
fn formatter_008_fm_37_empty_data_produces_header_only_html_table()
{
  let view = RowBuilder::new( vec![ "col".into() ] )
    .build_view();

  let formatter = HtmlFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "<thead>" ), "should contain <thead> with column header" );
  assert!( output.contains( "<th>col</th>" ), "should contain header cell" );
  // tbody should exist but contain no <tr> elements
  let tbody_start = output.find( "<tbody>" ).expect( "should have <tbody>" );
  let tbody_end = output.find( "</tbody>" ).expect( "should have </tbody>" );
  let tbody_content = &output[ tbody_start..tbody_end ];
  assert!( !tbody_content.contains( "<tr>" ), "empty data should have no <tr> in <tbody>" );
}

/// FM-38: HTML special characters are entity-escaped
// test_kind: spec_case(FM-38)
#[ test ]
fn formatter_008_fm_38_html_special_characters_are_entity_escaped()
{
  let view = RowBuilder::new( vec![ "text".into() ] )
    .add_row( vec![ "<b>bold</b> & \"quoted\"".into() ] )
    .build_view();

  let formatter = HtmlFormatter::new();
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "&lt;" ), "< should be escaped as &lt;" );
  assert!( output.contains( "&gt;" ), "> should be escaped as &gt;" );
  assert!( output.contains( "&amp;" ), "& should be escaped as &amp;" );
  assert!( output.contains( "&quot;" ), "\" should be escaped as &quot;" );
}
