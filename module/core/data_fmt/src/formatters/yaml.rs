//! YAML formatter for `TableView` data
//!
//! ## Output Example
//! ```
//! # #[cfg(feature = "format_yaml")]
//! # {
//! # use data_fmt::{ RowBuilder, YamlFormatter, Format };
//! # let view = RowBuilder::new(vec!["Name".into(), "Age".into()])
//! #   .add_row(vec!["Alice".into(), "30".into()])
//! #   .build_view();
//! let formatter = YamlFormatter::new();
//! // - Name: Alice
//! //   Age: '30'
//! # }
//! ```

use crate::{ TableView, Heading, formatters::{ Format, FormatError } };

/// YAML output formatter
///
/// Converts `TableView` data to array of objects where each row becomes
/// an object with column names as keys.
///
/// Output format: list of dictionaries in YAML
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "format_yaml")]
/// # {
/// use data_fmt::{ RowBuilder, YamlFormatter, Format };
///
/// let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
///   .add_row( vec![ "Alice".into(), "30".into() ] )
///   .build_view();
///
/// let formatter = YamlFormatter::new();
/// let yaml = formatter.format( &view ).unwrap();
/// assert!( yaml.contains( "Name:" ) );
/// assert!( yaml.contains( "Alice" ) );
/// # }
/// ```
#[ derive( Debug, Clone ) ]
pub struct YamlFormatter
{
  /// Optional titled rule rendered above the formatted output, as a `#` comment (`None` = no heading)
  pub heading : Option< Heading >,
  /// Optional titled rule rendered below the formatted output, as a `#` comment (`None` = no footer)
  pub footer : Option< Heading >,
}

impl YamlFormatter
{
  /// Create new YAML formatter
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

  /// Prepend heading and/or append footer around already-rendered YAML output, each wrapped
  /// in a `#` comment marker so the titled rule stays valid YAML.
  ///
  /// YAML output has no fixed column width, so the rule fills to the widest rendered line's
  /// display width instead of a precomputed `table_width` — same approach as
  /// `TreeFormatter`/`ExpandedFormatter`/`TextFormatter`.
  fn wrap_with_heading_footer( &self, body : String ) -> String
  {
    if self.heading.is_none() && self.footer.is_none()
    {
      return body;
    }
    let width = body.lines().map( crate::ansi_str::unicode_visual_len ).max().unwrap_or( 0 );
    let mut output = String::with_capacity( body.len() + 64 );
    crate::config::render_commented_rule_if_present( &mut output, self.heading.as_ref(), width, "# " );
    output.push_str( &body );
    crate::config::render_commented_rule_if_present( &mut output, self.footer.as_ref(), width, "# " );
    output
  }
}

impl Default for YamlFormatter
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl Format for YamlFormatter
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >
  {
    let rows = super::table_view_to_row_maps( data );

    serde_yaml_ng::to_string( &rows )
      .map( | body | self.wrap_with_heading_footer( body ) )
      .map_err( | e | FormatError::Serialization( e.to_string() ) )
  }
}
