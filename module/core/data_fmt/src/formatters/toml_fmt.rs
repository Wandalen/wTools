//! TOML formatter for `TableView` data
//!
//! ## Output Example
//! ```
//! # #[cfg(feature = "format_toml")]
//! # {
//! # use data_fmt::{ RowBuilder, TomlFormatter, Format };
//! # let view = RowBuilder::new(vec!["Name".into(), "Age".into()])
//! #   .add_row(vec!["Alice".into(), "30".into()])
//! #   .build_view();
//! let formatter = TomlFormatter::new();
//! // [[row]]
//! // Name = "Alice"
//! // Age = "30"
//! # }
//! ```

use crate::{ TableView, Heading, formatters::{ Format, FormatError } };

/// TOML output formatter
///
/// Converts `TableView` data to array of tables where each row becomes
/// a table with column names as keys.
///
/// Output format: TOML array of tables `[[row]]`
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "format_toml")]
/// # {
/// use data_fmt::{ RowBuilder, TomlFormatter, Format };
///
/// let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
///   .add_row( vec![ "Alice".into(), "30".into() ] )
///   .build_view();
///
/// let formatter = TomlFormatter::new();
/// let toml_str = formatter.format( &view ).unwrap();
/// assert!( toml_str.contains( "Name" ) );
/// # }
/// ```
#[ derive( Debug, Clone ) ]
pub struct TomlFormatter
{
  /// Optional titled rule rendered above the formatted output, as a `#` comment (`None` = no heading)
  pub heading : Option< Heading >,
  /// Optional titled rule rendered below the formatted output, as a `#` comment (`None` = no footer)
  pub footer : Option< Heading >,
}

impl TomlFormatter
{
  /// Create new TOML formatter
  pub fn new() -> Self
  {
    Self { heading : None, footer : None }
  }

  /// Attach a titled heading rule rendered above the formatted output, as a `#` comment
  #[ must_use ]
  pub fn with_heading( mut self, h : Heading ) -> Self
  {
    self.heading = Some( h );
    self
  }

  /// Attach a titled rule rendered below the formatted output, as a `#` comment
  #[ must_use ]
  pub fn with_footer( mut self, f : Heading ) -> Self
  {
    self.footer = Some( f );
    self
  }

  /// Prepend heading and/or append footer around already-rendered TOML output, each wrapped
  /// in a `#` comment marker so the titled rule stays valid TOML.
  ///
  /// TOML output has no fixed column width, so the rule fills to the widest rendered line's
  /// display width instead of a precomputed `table_width` — same approach as
  /// `TreeFormatter`/`ExpandedFormatter`/`TextFormatter`/`YamlFormatter`.
  fn wrap_with_heading_footer( &self, body : String ) -> String
  {
    if self.heading.is_none() && self.footer.is_none()
    {
      return body;
    }
    let width = body.lines().map( crate::ansi_str::unicode_visual_len ).max().unwrap_or( 0 );
    let mut output = String::with_capacity( body.len() + 64 );
    crate::config::render_commented_rule_if_present( &mut output, self.heading.as_ref(), width, "# ", "" );
    output.push_str( &body );
    crate::config::render_commented_rule_if_present( &mut output, self.footer.as_ref(), width, "# ", "" );
    output
  }
}

impl Default for TomlFormatter
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl Format for TomlFormatter
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >
  {
    let rows = super::table_view_to_row_maps( data );

    // TOML requires wrapping in a structure for array of tables
    #[ derive( serde::Serialize ) ]
    struct TomlWrapper
    {
      row : Vec< std::collections::HashMap< String, String > >,
    }

    let wrapper = TomlWrapper { row : rows };
    toml::to_string( &wrapper )
      .map( | body | self.wrap_with_heading_footer( body ) )
      .map_err( | e | FormatError::Serialization( e.to_string() ) )
  }
}
