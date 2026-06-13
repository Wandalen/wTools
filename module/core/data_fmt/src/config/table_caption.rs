//! `TableCaption` titled rule type and associated constants

/// Character placed between caption fields in a titled rule line (`·` U+00B7)
pub const CAPTION_FIELD_SEP : char = '·';

/// Character used for the horizontal rule fill in a caption line (`─` U+2500)
pub const CAPTION_RULE_CHAR : char = '─';

/// Number of rule characters emitted before the title text in a caption
pub const CAPTION_LEAD_WIDTH : usize = 3;

/// A titled rule to be rendered above a table
///
/// Carries a title and optional caption fields separated by [`CAPTION_FIELD_SEP`].
/// Rendered as: `─── Title · Field1 · Field2 ──────...` filling terminal width.
#[ derive( Debug, Clone ) ]
pub struct TableCaption
{
  title  : String,
  fields : Vec< String >,
}

impl TableCaption
{
  /// Create a new caption with the given title
  #[ must_use ]
  pub fn new( title : impl Into< String > ) -> Self
  {
    Self
    {
      title  : title.into(),
      fields : Vec::new(),
    }
  }

  /// Append a caption field — appears after the title separated by [`CAPTION_FIELD_SEP`]
  #[ must_use ]
  pub fn field( mut self, f : impl Into< String > ) -> Self
  {
    self.fields.push( f.into() );
    self
  }

  /// Build the rendered content string: `"title · field1 · field2 ..."`
  pub( crate ) fn content_str( &self ) -> String
  {
    let mut s = self.title.clone();
    for f in &self.fields
    {
      s.push( ' ' );
      s.push( CAPTION_FIELD_SEP );
      s.push( ' ' );
      s.push_str( f );
    }
    s
  }
}
