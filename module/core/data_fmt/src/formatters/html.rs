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
//! # use data_fmt::{ RowBuilder, HtmlFormatter, HtmlVariant, Format };
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

use crate::{ TableView, Heading, formatters::{ Format, FormatError } };

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
/// # use data_fmt::{ RowBuilder, HtmlFormatter, HtmlVariant, Format };
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
  /// Optional titled rule rendered above the formatted output, as an `<!-- -->` comment (`None` = no heading)
  pub heading : Option< Heading >,
  /// Optional titled rule rendered below the formatted output, as an `<!-- -->` comment (`None` = no footer)
  pub footer : Option< Heading >,
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
      heading : None,
      footer : None,
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
      heading : None,
      footer : None,
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
      heading : None,
      footer : None,
    }
  }

  /// Set table ID attribute
  #[ must_use ]
  pub fn with_table_id( mut self, id : impl Into< String > ) -> Self
  {
    self.table_id = Some( id.into() );
    self
  }

  /// Enable/disable HTML wrapper (html, body tags)
  #[ must_use ]
  pub fn with_include_wrapper( mut self, enabled : bool ) -> Self
  {
    self.include_wrapper = enabled;
    self
  }

  /// Attach a titled heading rule rendered above the formatted output, as an `<!-- -->` comment
  #[ must_use ]
  pub fn with_heading( mut self, h : Heading ) -> Self
  {
    self.heading = Some( h );
    self
  }

  /// Attach a titled rule rendered below the formatted output, as an `<!-- -->` comment
  #[ must_use ]
  pub fn with_footer( mut self, f : Heading ) -> Self
  {
    self.footer = Some( f );
    self
  }

  /// Prepend heading and/or append footer around the already-rendered HTML output (including
  /// the optional `<!DOCTYPE>`/`<html>`/`<body>` wrapper), each wrapped in an `<!-- -->` comment
  /// so the titled rule stays valid HTML — unlike `#`/`--` line comments, `<!-- -->` needs an
  /// explicit closing delimiter or everything up to the next `-->` would be swallowed.
  ///
  /// HTML output has no fixed column width, so the rule fills to the widest rendered line's
  /// display width instead of a precomputed `table_width` — same approach as
  /// `TreeFormatter`/`ExpandedFormatter`/`TextFormatter`/`YamlFormatter`/`TomlFormatter`/`SqlFormatter`.
  ///
  /// Like `SqlFormatter`'s body (and unlike Yaml/Toml/Text, which always end with `\n`), HTML
  /// output ends with a bare closing tag (`</table>` or `</html>`) and no trailing newline. A
  /// footer appended directly onto that would land on the same line as the closing tag instead
  /// of its own line, so a separating `\n` is inserted first whenever the body is non-empty and
  /// doesn't already end in one.
  fn wrap_with_heading_footer( &self, body : String ) -> String
  {
    if self.heading.is_none() && self.footer.is_none()
    {
      return body;
    }
    let width = body.lines().map( crate::ansi_str::unicode_visual_len ).max().unwrap_or( 0 );
    let mut output = String::with_capacity( body.len() + 64 );
    crate::config::render_commented_rule_if_present( &mut output, self.heading.as_ref(), width, "<!-- ", " -->" );
    output.push_str( &body );
    if self.footer.is_some() && !body.is_empty() && !body.ends_with( '\n' )
    {
      output.push( '\n' );
    }
    crate::config::render_commented_rule_if_present( &mut output, self.footer.as_ref(), width, "<!-- ", " -->" );
    output
  }

  /// Get CSS class string for variant
  fn variant_class( &self ) -> Option< String >
  {
    match &self.variant
    {
      HtmlVariant::Minimal => None,
      HtmlVariant::Bootstrap => Some( "table table-striped table-hover".to_string() ),
      HtmlVariant::Tailwind => Some( "min-w-full divide-y divide-gray-200".to_string() ),
      // Fix(BUG-019): treat empty custom class the same as Minimal (no class attr).
      // Root cause: `Some("")` caused `class=""` to be emitted in the opening tag,
      // which is semantically incorrect — an empty class attribute is never useful.
      // Pitfall: always check for empty custom class string before emitting the attr.
      HtmlVariant::Custom( class ) if class.is_empty() => None,
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
        output.push_str( &Self::escape_html( &cell.text ) );
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

    Ok( self.wrap_with_heading_footer( output ) )
  }
}
