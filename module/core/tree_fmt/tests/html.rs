//! Integration tests for `HtmlFormatter`
//!
//! Tests HTML table generation with various themes and configurations.

#[ cfg( feature = "format_html" ) ]
mod html_tests
{
  use tree_fmt::{ RowBuilder, HtmlFormatter, HtmlVariant, Format };

  #[ test ]
  fn test_html_minimal_theme_basic()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = HtmlFormatter::new();
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "<table>" ) );
    assert!( html.contains( "<thead>" ) );
    assert!( html.contains( "<th>Name</th><th>Age</th>" ) );
    assert!( html.contains( "<tbody>" ) );
    assert!( html.contains( "<td>Alice</td><td>30</td>" ) );
    assert!( html.contains( "<td>Bob</td><td>25</td>" ) );
    assert!( !html.contains( "class=" ) );
  }

  #[ test ]
  fn test_html_bootstrap_theme()
  {
    let view = RowBuilder::new( vec![ "Product".into(), "Price".into() ] )
      .add_row( vec![ "Widget".into(), "$10".into() ] )
      .build_view();

    let formatter = HtmlFormatter::with_variant( HtmlVariant::Bootstrap );
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "<table class=\"table table-striped table-hover\">" ) );
    assert!( html.contains( "<th>Product</th><th>Price</th>" ) );
    assert!( html.contains( "<td>Widget</td><td>$10</td>" ) );
  }

  #[ test ]
  fn test_html_tailwind_theme()
  {
    let view = RowBuilder::new( vec![ "Col1".into() ] )
      .add_row( vec![ "Data1".into() ] )
      .build_view();

    let formatter = HtmlFormatter::with_variant( HtmlVariant::Tailwind );
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "<table class=\"min-w-full divide-y divide-gray-200\">" ) );
  }

  #[ test ]
  fn test_html_custom_class()
  {
    let view = RowBuilder::new( vec![ "Col".into() ] )
      .add_row( vec![ "Data".into() ] )
      .build_view();

    let formatter = HtmlFormatter::with_table_class( "custom-table responsive" );
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "<table class=\"custom-table responsive\">" ) );
  }

  #[ test ]
  fn test_html_table_id()
  {
    let view = RowBuilder::new( vec![ "Col".into() ] )
      .add_row( vec![ "Data".into() ] )
      .build_view();

    let formatter = HtmlFormatter::new().table_id( "my-table" );
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( " id=\"my-table\"" ) );
  }

  #[ test ]
  fn test_html_escaping_xss_prevention()
  {
    let view = RowBuilder::new( vec![ "Content".into() ] )
      .add_row( vec![ "<script>alert('XSS')</script>".into() ] )
      .add_row( vec![ "<img src=x onerror=alert(1)>".into() ] )
      .add_row( vec![ "A & B".into() ] )
      .add_row( vec![ "Quote: \"Test\"".into() ] )
      .build_view();

    let formatter = HtmlFormatter::new();
    let html = formatter.format( &view ).unwrap();

    // Ensure dangerous content is properly escaped
    assert!( !html.contains( "<script>" ) );
    assert!( !html.contains( "<img src=" ) ); // Raw tag should not appear
    assert!( html.contains( "&lt;script&gt;" ) );
    assert!( html.contains( "&lt;img" ) );
    assert!( html.contains( "&amp;" ) );
    assert!( html.contains( "&quot;" ) );
  }

  #[ test ]
  fn test_html_empty_table()
  {
    let view = RowBuilder::new( vec![ "Col1".into(), "Col2".into() ] ).build_view();

    let formatter = HtmlFormatter::new();
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "<table>" ) );
    assert!( html.contains( "<thead>" ) );
    assert!( html.contains( "<th>Col1</th><th>Col2</th>" ) );
    assert!( html.contains( "<tbody>" ) );
    assert!( html.contains( "</tbody>" ) );
  }

  #[ test ]
  fn test_html_wrapper_enabled()
  {
    let view = RowBuilder::new( vec![ "Col".into() ] )
      .add_row( vec![ "Data".into() ] )
      .build_view();

    let formatter = HtmlFormatter::new().include_wrapper( true );
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "<!DOCTYPE html>" ) );
    assert!( html.contains( "<html>" ) );
    assert!( html.contains( "<head>" ) );
    assert!( html.contains( "<meta charset=\"UTF-8\">" ) );
    assert!( html.contains( "<body>" ) );
    assert!( html.contains( "</body>" ) );
    assert!( html.contains( "</html>" ) );
  }

  #[ test ]
  fn test_html_multiple_rows()
  {
    let mut builder = RowBuilder::new( vec![ "ID".into(), "Name".into(), "Status".into() ] );

    for i in 1..=10
    {
      builder = builder.add_row( vec![
        format!( "{}", i ),
        format!( "User{}", i ),
        "Active".into()
      ] );
    }

    let view = builder.build_view();
    let formatter = HtmlFormatter::with_variant( HtmlVariant::Bootstrap );
    let html = formatter.format( &view ).unwrap();

    // Check all 10 rows are present
    for i in 1..=10
    {
      assert!( html.contains( &format!( "<td>{i}</td>" ) ) );
      assert!( html.contains( &format!( "<td>User{i}</td>" ) ) );
    }
  }

  #[ test ]
  fn test_html_unicode_content()
  {
    let view = RowBuilder::new( vec![ "Language".into(), "Greeting".into() ] )
      .add_row( vec![ "Japanese".into(), "こんにちは".into() ] )
      .add_row( vec![ "Arabic".into(), "مرحبا".into() ] )
      .add_row( vec![ "Emoji".into(), "🎉🚀✨".into() ] )
      .build_view();

    let formatter = HtmlFormatter::new();
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "こんにちは" ) );
    assert!( html.contains( "مرحبا" ) );
    assert!( html.contains( "🎉🚀✨" ) );
  }

  #[ test ]
  fn test_html_all_special_chars()
  {
    let view = RowBuilder::new( vec![ "Char".into(), "Escaped".into() ] )
      .add_row( vec![ "<".into(), "&lt;".into() ] )
      .add_row( vec![ ">".into(), "&gt;".into() ] )
      .add_row( vec![ "&".into(), "&amp;".into() ] )
      .add_row( vec![ "\"".into(), "&quot;".into() ] )
      .add_row( vec![ "'".into(), "&#x27;".into() ] )
      .build_view();

    let formatter = HtmlFormatter::new();
    let html = formatter.format( &view ).unwrap();

    // All special chars should be escaped
    assert!( html.contains( "<td>&lt;</td>" ) );
    assert!( html.contains( "<td>&gt;</td>" ) );
    assert!( html.contains( "<td>&amp;</td>" ) );
    assert!( html.contains( "<td>&quot;</td>" ) );
    assert!( html.contains( "<td>&#x27;</td>" ) );
  }

  #[ test ]
  fn test_html_chained_configuration()
  {
    let view = RowBuilder::new( vec![ "Col".into() ] )
      .add_row( vec![ "Data".into() ] )
      .build_view();

    let formatter = HtmlFormatter::with_variant( HtmlVariant::Bootstrap )
      .table_id( "results" )
      .include_wrapper( true );

    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "<!DOCTYPE html>" ) );
    assert!( html.contains( "class=\"table table-striped table-hover\"" ) );
    assert!( html.contains( "id=\"results\"" ) );
  }
}
