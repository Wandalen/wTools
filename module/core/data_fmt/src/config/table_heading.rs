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

/// A titled rule rendered as a header (above) or footer (below) formatted output
///
/// Carries a title and optional heading fields separated by [`HEADING_FIELD_SEP`].
/// Rendered as: `─── Title · Field1 · Field2 ──────...` filling the width of the
/// output it accompanies.
// Renamed: TableCaption → Heading (TSK-009)
// Generalized: header-only → header + footer (TSK-018)
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

  /// Render this heading/footer as a single titled-rule line (including trailing `\n`),
  /// filling `table_width` display columns: `─── content ──────...`.
  ///
  /// Formatter-agnostic: any formatter computes its own rendered width and passes it here;
  /// this method has no awareness of which formatter or which position (above/below) is
  /// rendering it — see `render_rule_if_present` for the `Option`-handling convenience
  /// wrapper used at formatter call sites.
  ///
  /// Uses `unicode_visual_len` for content width (not `.chars().count()`) so that
  /// CJK / wide characters are measured correctly at 2 display columns each.
  pub( crate ) fn render_line( &self, table_width : usize ) -> String
  {
    let content = self.content_str();
    // Fix(BUG-015): use unicode_visual_len (display columns) instead of .chars().count().
    // Root cause: CJK characters are 1 char but 2 display columns; .chars().count()
    //   undercounted, making the trail too long and heading line wider than table body.
    // Pitfall: always use unicode_visual_len for any width arithmetic that must match
    //   what the terminal actually renders.
    let content_dw = crate::ansi_str::unicode_visual_len( &content );
    let used = HEADING_LEAD_WIDTH + 1 + content_dw + 1;
    let trail = table_width.saturating_sub( used );
    let lead  : String = std::iter::repeat_n( HEADING_RULE_CHAR, HEADING_LEAD_WIDTH ).collect();
    let trail_str : String = std::iter::repeat_n( HEADING_RULE_CHAR, trail ).collect();
    let mut line = String::with_capacity( lead.len() + content.len() + trail_str.len() + 3 );
    line.push_str( &lead );
    line.push( ' ' );
    line.push_str( &content );
    line.push( ' ' );
    line.push_str( &trail_str );
    line.push( '\n' );
    line
  }
}

/// Render a titled rule (heading or footer) into `output`, or do nothing if `rule` is `None`.
///
/// Formatter-agnostic and position-agnostic: every formatter (Table, Tree, ...) passes its
/// own `heading_ref()`/`footer_ref()`-equivalent accessor and its own rendered width; this
/// function has no awareness of which formatter or which position (above/below) calls it —
/// every historical bug fix in [`Heading::render_line`] therefore applies uniformly to every
/// caller.
pub( crate ) fn render_rule_if_present( output : &mut String, rule : Option< &Heading >, table_width : usize )
{
  if let Some( heading ) = rule
  {
    output.push_str( &heading.render_line( table_width ) );
  }
}

/// Render a titled rule (heading or footer) prefixed with a comment marker, or do nothing
/// if `rule` is `None`. Reuses [`Heading::render_line`] unmodified — the comment prefix is
/// prepended as a wrapping step so YAML/TOML (`#`) and SQL (`--`) heading/footer lines stay
/// valid comments in their respective syntaxes, without changing `Heading`'s own rendering.
///
/// `target_width` is the total visible width the emitted line (prefix + rule) should fill;
/// the prefix's own display width is subtracted from it before calling `render_line`, so the
/// rule portion narrows to leave room for the prefix rather than being added on top of it.
pub( crate ) fn render_commented_rule_if_present( output : &mut String, rule : Option< &Heading >, target_width : usize, comment_prefix : &str )
{
  if let Some( heading ) = rule
  {
    let prefix_dw = crate::ansi_str::unicode_visual_len( comment_prefix );
    let inner_width = target_width.saturating_sub( prefix_dw );
    output.push_str( comment_prefix );
    output.push_str( &heading.render_line( inner_width ) );
  }
}
