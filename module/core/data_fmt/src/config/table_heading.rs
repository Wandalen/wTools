//! `Heading` titled rule type and associated constants

/// Character placed between heading fields in a titled rule line (`·` U+00B7)
pub const HEADING_FIELD_SEP : char = '·';

/// Character used for the horizontal rule fill in a heading line (`─` U+2500)
pub const HEADING_RULE_CHAR : char = '─';

/// Number of rule characters emitted before the title text in a heading
pub const HEADING_LEAD_WIDTH : usize = 3;

/// Replace all line-break sequences (`\r\n`, `\r`, `\n`) with a single space.
///
/// Order matters: `\r\n` is consumed as one pair before bare `\r` or `\n`.
fn sanitize_line_breaks( s : &str ) -> String
{
  s.replace( "\r\n", " " ).replace( [ '\r', '\n' ], " " )
}

/// A titled rule to be rendered above a table
///
/// Carries a title and optional heading fields separated by [`HEADING_FIELD_SEP`].
/// Rendered as: `─── Title · Field1 · Field2 ──────...` filling rendered table width.
// Renamed: TableCaption → Heading (TSK-009)
#[ derive( Debug, Clone ) ]
pub struct Heading
{
  title  : String,
  fields : Vec< String >,
}

impl Heading
{
  /// Create a new heading with the given title
  #[ must_use ]
  pub fn new( title : impl Into< String > ) -> Self
  {
    Self
    {
      title  : title.into(),
      fields : Vec::new(),
    }
  }

  /// Append a heading field — appears after the title separated by [`HEADING_FIELD_SEP`]
  #[ must_use ]
  pub fn with_field( mut self, f : impl Into< String > ) -> Self
  {
    self.fields.push( f.into() );
    self
  }

  /// Build the rendered content string: `"title · field1 · field2 ..."`
  ///
  /// Fix(BUG-016): newlines in title or fields are replaced with spaces to guarantee
  /// the heading occupies exactly one output line (invariant IN-3).
  /// Root cause: title/fields were emitted verbatim; embedded `\n` broke the
  ///   heading across multiple terminal lines, violating the single-line invariant.
  /// Pitfall: any user-supplied string passed to heading content must be sanitized
  ///   before width arithmetic — `\n` is invisible to `unicode_visual_len` but
  ///   produces visible line breaks in terminal output.
  pub( crate ) fn content_str( &self ) -> String
  {
    let mut s = sanitize_line_breaks( &self.title );
    for f in &self.fields
    {
      s.push( ' ' );
      s.push( HEADING_FIELD_SEP );
      s.push( ' ' );
      s.push_str( &sanitize_line_breaks( f ) );
    }
    s
  }
}
