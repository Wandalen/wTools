//! Example demonstrating `HtmlFormatter` usage
//!
//! Shows HTML table generation with different themes and configurations.
//!
//! Run with:
//! ```bash
//! cargo run --example html_format --features format_html
//! ```

#[ cfg( feature = "format_html" ) ]
use tree_fmt::{ RowBuilder, HtmlFormatter, HtmlVariant, Format };

#[ cfg( not( feature = "format_html" ) ) ]
fn main()
{
  println!( "This example requires the 'format_html' feature." );
  println!( "Run with: cargo run --example html_format --features format_html" );
}

#[ cfg( feature = "format_html" ) ]
fn main()
{
  println!( "=== HtmlFormatter Examples ===\n" );

  // Sample data
  let view = RowBuilder::new( vec![ "Product".into(), "Price".into(), "Stock".into() ] )
    .add_row( vec![ "Laptop".into(), "$999".into(), "5".into() ] )
    .add_row( vec![ "Mouse".into(), "$29".into(), "150".into() ] )
    .add_row( vec![ "Keyboard".into(), "$79".into(), "45".into() ] )
    .add_row( vec![ "Monitor".into(), "$299".into(), "12".into() ] )
    .build_view();

  // Example 1: Minimal theme (no CSS classes)
  println!( "1. Minimal Theme (no CSS classes):\n" );
  let formatter = HtmlFormatter::new();
  let html = formatter.format( &view ).unwrap();
  println!( "{html}\n" );

  // Example 2: Bootstrap 5 theme
  println!( "2. Bootstrap 5 Theme:\n" );
  let formatter = HtmlFormatter::with_variant( HtmlVariant::Bootstrap );
  let html = formatter.format( &view ).unwrap();
  println!( "{html}\n" );

  // Example 3: Tailwind CSS theme
  println!( "3. Tailwind CSS Theme:\n" );
  let formatter = HtmlFormatter::with_variant( HtmlVariant::Tailwind );
  let html = formatter.format( &view ).unwrap();
  println!( "{html}\n" );

  // Example 4: Custom CSS class
  println!( "4. Custom CSS Class:\n" );
  let formatter = HtmlFormatter::with_table_class( "my-custom-table striped hover" );
  let html = formatter.format( &view ).unwrap();
  println!( "{html}\n" );

  // Example 5: With table ID
  println!( "5. With Table ID:\n" );
  let formatter = HtmlFormatter::with_variant( HtmlVariant::Bootstrap )
    .table_id( "products-table" );
  let html = formatter.format( &view ).unwrap();
  println!( "{html}\n" );

  // Example 6: Complete HTML document
  println!( "6. Complete HTML Document:\n" );
  let formatter = HtmlFormatter::with_variant( HtmlVariant::Bootstrap )
    .table_id( "products" )
    .include_wrapper( true );
  let html = formatter.format( &view ).unwrap();
  println!( "{html}\n" );

  // Example 7: HTML escaping demonstration
  println!( "7. HTML Escaping (XSS Prevention):\n" );
  let dangerous_view = RowBuilder::new( vec![ "Input".into(), "Output".into() ] )
    .add_row( vec![ "<script>alert('XSS')</script>".into(), "Escaped".into() ] )
    .add_row( vec![ "<img src=x onerror=alert(1)>".into(), "Escaped".into() ] )
    .add_row( vec![ "A & B < C > D".into(), "All escaped".into() ] )
    .add_row( vec![ "Quote: \"test\"".into(), "Escaped".into() ] )
    .build_view();

  let formatter = HtmlFormatter::new();
  let html = formatter.format( &dangerous_view ).unwrap();
  println!( "{html}\n" );
  println!( "Note: All special characters (<, >, &, \", ') are properly escaped.\n" );

  // Example 8: Empty table
  println!( "8. Empty Table:\n" );
  let empty_view = RowBuilder::new( vec![ "Col1".into(), "Col2".into() ] ).build_view();
  let formatter = HtmlFormatter::with_variant( HtmlVariant::Bootstrap );
  let html = formatter.format( &empty_view ).unwrap();
  println!( "{html}\n" );

  println!( "=== Usage Tips ===" );
  println!( "- Use Minimal theme for maximum styling control" );
  println!( "- Bootstrap/Tailwind themes work with respective CSS frameworks" );
  println!( "- Custom theme allows any CSS class combination" );
  println!( "- All content is automatically HTML-escaped for security" );
  println!( "- include_wrapper(true) generates complete HTML5 document" );
}
