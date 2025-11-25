//! HTML table formatter for web output
//!
//! ## Purpose
//!
//! Generate semantic HTML5 table markup from tabular data, suitable for:
//! - Web dashboards and admin panels
//! - Static site generation
//! - HTML email reports
//! - Documentation generation
//!
//! ## HTML Structure
//!
//! Generates clean semantic HTML:
//! ```html
//! <table class="...">
//!   <thead>
//!     <tr><th>Header1</th><th>Header2</th></tr>
//!   </thead>
//!   <tbody>
//!     <tr><td>Data1</td><td>Data2</td></tr>
//!   </tbody>
//! </table>
//! ```
//!
//! ## Themes
//!
//! **Minimal** - Pure semantic HTML with no classes:
//! ```html
//! <table>...</table>
//! ```
//!
//! **Bootstrap** - Bootstrap 5 compatible classes:
//! ```html
//! <table class="table table-striped table-hover">...</table>
//! ```
//!
//! **Tailwind** - Tailwind CSS utility classes:
//! ```html
//! <table class="min-w-full divide-y divide-gray-200">...</table>
//! ```
//!
//! **Custom** - User-provided class string
//!
//! ## HTML Escaping
//!
//! All content is properly HTML-escaped:
//! - `<` → `&lt;`
//! - `>` → `&gt;`
//! - `&` → `&amp;`
//! - `"` → `&quot;`
//! - `'` → `&#x27;`
//!
//! ## Examples
//!
//! ```
//! # use tree_fmt::{ RowBuilder, HtmlFormatter, HtmlVariant, Format };
//! let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
//!   .add_row( vec![ "Alice".into(), "30".into() ] )
//!   .build_view();
//!
//! // Minimal theme (no classes)
//! let formatter = HtmlFormatter::new();
//! let html = formatter.format( &view ).unwrap();
//!
//! // Bootstrap theme
//! let formatter = HtmlFormatter::with_variant( HtmlVariant::Bootstrap );
//! let html = formatter.format( &view ).unwrap();
//! ```

use crate::{ TableView, formatters::{ Format, FormatError } };

/// HTML table themes with predefined CSS classes
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum HtmlVariant
{
  /// No CSS classes (pure semantic HTML)
  Minimal,
  /// Bootstrap 5 classes
  Bootstrap,
  /// Tailwind CSS classes
  Tailwind,
  /// Custom CSS classes
  Custom( String ),
}

/// HTML table formatter
///
/// Generates semantic HTML5 table markup with optional CSS theme support.
///
/// # Examples
///
/// ```
/// # use tree_fmt::{ RowBuilder, HtmlFormatter, HtmlVariant, Format };
/// let view = RowBuilder::new( vec![ "Product".into(), "Price".into() ] )
///   .add_row( vec![ "Widget".into(), "$10".into() ] )
///   .add_row( vec![ "Gadget".into(), "$20".into() ] )
///   .build_view();
///
/// let formatter = HtmlFormatter::with_variant( HtmlVariant::Bootstrap );
/// let html = formatter.format( &view ).unwrap();
///
/// assert!( html.contains( "<table" ) );
/// assert!( html.contains( "<thead>" ) );
/// assert!( html.contains( "<tbody>" ) );
/// ```
#[ derive( Debug, Clone ) ]
pub struct HtmlFormatter
{
  /// CSS variant for table styling
  pub variant : HtmlVariant,
  /// Include wrapping HTML structure (html, body tags)
  pub include_wrapper : bool,
  /// Custom table ID attribute
  pub table_id : Option< String >,
}

impl HtmlFormatter
{
  /// Create new HTML formatter with minimal variant (no CSS classes)
  pub fn new() -> Self
  {
    Self
    {
      variant : HtmlVariant::Minimal,
      include_wrapper : false,
      table_id : None,
    }
  }

  /// Create HTML formatter with specified variant
  pub fn with_variant( variant : HtmlVariant ) -> Self
  {
    Self
    {
      variant,
      include_wrapper : false,
      table_id : None,
    }
  }

  /// Create HTML formatter with custom table class
  pub fn with_table_class( class : impl Into< String > ) -> Self
  {
    Self
    {
      variant : HtmlVariant::Custom( class.into() ),
      include_wrapper : false,
      table_id : None,
    }
  }

  /// Set table ID attribute
  #[ must_use ]
  pub fn table_id( mut self, id : impl Into< String > ) -> Self
  {
    self.table_id = Some( id.into() );
    self
  }

  /// Enable/disable HTML wrapper (html, body tags)
  #[ must_use ]
  pub fn include_wrapper( mut self, enabled : bool ) -> Self
  {
    self.include_wrapper = enabled;
    self
  }

  /// Get CSS class string for variant
  fn variant_class( &self ) -> Option< String >
  {
    match &self.variant
    {
      HtmlVariant::Minimal => None,
      HtmlVariant::Bootstrap => Some( "table table-striped table-hover".to_string() ),
      HtmlVariant::Tailwind => Some( "min-w-full divide-y divide-gray-200".to_string() ),
      HtmlVariant::Custom( class ) => Some( class.clone() ),
    }
  }

  /// Escape HTML special characters
  fn escape_html( text : &str ) -> String
  {
    let mut escaped = String::with_capacity( text.len() + 10 );

    for ch in text.chars()
    {
      match ch
      {
        '<' => escaped.push_str( "&lt;" ),
        '>' => escaped.push_str( "&gt;" ),
        '&' => escaped.push_str( "&amp;" ),
        '"' => escaped.push_str( "&quot;" ),
        '\'' => escaped.push_str( "&#x27;" ),
        _ => escaped.push( ch ),
      }
    }

    escaped
  }
}

impl Default for HtmlFormatter
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl Format for HtmlFormatter
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >
  {
    let mut output = String::new();

    // Optional HTML wrapper
    if self.include_wrapper
    {
      output.push_str( "<!DOCTYPE html>\n" );
      output.push_str( "<html>\n<head>\n" );
      output.push_str( "  <meta charset=\"UTF-8\">\n" );
      output.push_str( "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n" );
      output.push_str( "  <title>Table</title>\n" );
      output.push_str( "</head>\n<body>\n" );
    }

    // Table opening tag with class and ID
    output.push_str( "<table" );

    if let Some( class ) = self.variant_class()
    {
      output.push_str( " class=\"" );
      output.push_str( &class );
      output.push( '"' );
    }

    if let Some( id ) = &self.table_id
    {
      output.push_str( " id=\"" );
      output.push_str( &Self::escape_html( id ) );
      output.push( '"' );
    }

    output.push_str( ">\n" );

    // Table header
    if !data.metadata.column_names.is_empty()
    {
      output.push_str( "  <thead>\n    <tr>" );

      for col_name in &data.metadata.column_names
      {
        output.push_str( "<th>" );
        output.push_str( &Self::escape_html( col_name ) );
        output.push_str( "</th>" );
      }

      output.push_str( "</tr>\n  </thead>\n" );
    }

    // Table body
    output.push_str( "  <tbody>\n" );

    for row in &data.rows
    {
      output.push_str( "    <tr>" );

      for cell in row
      {
        output.push_str( "<td>" );
        output.push_str( &Self::escape_html( cell ) );
        output.push_str( "</td>" );
      }

      output.push_str( "</tr>\n" );
    }

    output.push_str( "  </tbody>\n" );
    output.push_str( "</table>" );

    // Close HTML wrapper
    if self.include_wrapper
    {
      output.push_str( "\n</body>\n</html>" );
    }

    Ok( output )
  }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use crate::RowBuilder;

  #[ test ]
  fn test_html_minimal_theme()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .build_view();

    let formatter = HtmlFormatter::new();
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "<table>" ) );
    assert!( html.contains( "<thead>" ) );
    assert!( html.contains( "<th>Name</th>" ) );
    assert!( html.contains( "<tbody>" ) );
    assert!( html.contains( "<td>Alice</td>" ) );
  }

  #[ test ]
  fn test_html_bootstrap_variant()
  {
    let view = RowBuilder::new( vec![ "Col".into() ] )
      .add_row( vec![ "Data".into() ] )
      .build_view();

    let formatter = HtmlFormatter::with_variant( HtmlVariant::Bootstrap );
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "class=\"table table-striped table-hover\"" ) );
  }

  #[ test ]
  fn test_html_escaping()
  {
    let view = RowBuilder::new( vec![ "HTML".into() ] )
      .add_row( vec![ "<script>alert('xss')</script>".into() ] )
      .add_row( vec![ "A & B".into() ] )
      .add_row( vec![ "Quote: \"Hello\"".into() ] )
      .build_view();

    let formatter = HtmlFormatter::new();
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "&lt;script&gt;" ) );
    assert!( html.contains( "&amp;" ) );
    assert!( html.contains( "&quot;" ) );
    assert!( !html.contains( "<script>" ) );
  }

  #[ test ]
  fn test_html_custom_class()
  {
    let view = RowBuilder::new( vec![ "Col".into() ] )
      .add_row( vec![ "Data".into() ] )
      .build_view();

    let formatter = HtmlFormatter::with_table_class( "my-custom-table" );
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "class=\"my-custom-table\"" ) );
  }

  #[ test ]
  fn test_html_table_id()
  {
    let view = RowBuilder::new( vec![ "Col".into() ] )
      .add_row( vec![ "Data".into() ] )
      .build_view();

    let formatter = HtmlFormatter::new().table_id( "results-table" );
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "id=\"results-table\"" ) );
  }

  #[ test ]
  fn test_html_empty_table()
  {
    let view = RowBuilder::new( vec![ "Col".into() ] ).build_view();

    let formatter = HtmlFormatter::new();
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "<table>" ) );
    assert!( html.contains( "<thead>" ) );
    assert!( html.contains( "<tbody>" ) );
  }

  #[ test ]
  fn test_html_tailwind_variant()
  {
    let view = RowBuilder::new( vec![ "Col".into() ] )
      .add_row( vec![ "Data".into() ] )
      .build_view();

    let formatter = HtmlFormatter::with_variant( HtmlVariant::Tailwind );
    let html = formatter.format( &view ).unwrap();

    assert!( html.contains( "class=\"min-w-full divide-y divide-gray-200\"" ) );
  }
}
