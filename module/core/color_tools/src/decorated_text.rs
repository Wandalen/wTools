use super::color::Color;

/// Text string paired with an optional ANSI color prefix.
///
/// `From< String >` and `From< &str >` are transparent — no color allocation, no escape injection.
/// Use `.with_color( "\x1b[33m" )` to attach a color; `.render()` appends the ANSI reset `"\x1b[0m"`.
///
/// # Examples
///
/// ```
/// use color_tools::DecoratedText;
///
/// let plain : DecoratedText = "hello".into();
/// assert_eq!( plain.render(), "hello" );
///
/// let colored = DecoratedText::from( "warn" ).with_color( "\x1b[33m" );
/// assert!( colored.render().starts_with( "\x1b[33m" ) );
/// assert!( colored.render().ends_with( "\x1b[0m" ) );
/// ```
#[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
#[ cfg_attr( feature = "serde_support", derive( serde::Serialize, serde::Deserialize ) ) ]
pub struct DecoratedText
{
  /// The raw text content.
  pub text  : String,
  /// Optional ANSI escape prefix (e.g. `"\x1b[33m"` for yellow).
  pub color : Option< String >,
  /// Whether bold (SGR 1) is applied. Combines with `color` into one leading
  /// escape sequence at render time. Not intended to be combined with `dim`:
  /// both are intensity modifiers and most terminals honor only whichever of
  /// the two was applied last, so mixing them is a caller error, not a
  /// supported style.
  #[ cfg_attr( feature = "serde_support", serde( default ) ) ]
  pub bold : bool,
  /// Whether dim/faint (SGR 2) is applied. See `bold` for the mutual-exclusion note.
  #[ cfg_attr( feature = "serde_support", serde( default ) ) ]
  pub dim : bool,
  /// Semantic color intent, preserved for HTML rendering via `render_html()`.
  /// Only available when the `html_support` feature is enabled.
  /// Skipped in serde serialization to preserve JSON schema stability.
  #[ cfg( feature = "html_support" ) ]
  #[ cfg_attr( feature = "serde_support", serde( skip ) ) ]
  pub named_color : Option< Color >,
}

impl DecoratedText
{
  /// Attach an ANSI color prefix. Returns `self` for builder chaining.
  ///
  /// # Example
  ///
  /// ```
  /// use color_tools::DecoratedText;
  /// let ct = DecoratedText::from( "err" ).with_color( "\x1b[31m" );
  /// assert!( ct.is_colored() );
  /// ```
  #[ must_use ]
  pub fn with_color( mut self, ansi : impl Into< String > ) -> Self
  {
    self.color = Some( ansi.into() );
    // Fix(issue-none): clear any previously stored named_color so that
    // render_html() doesn't emit a stale CSS span after the caller overrides
    // the color with a raw ANSI string.
    // Root cause: with_color only updated `color`, leaving `named_color`
    //   pointing at whatever with_color_named had set earlier.
    // Pitfall: always pair raw-ANSI and typed-color state together;
    //   they describe the same semantic slot.
    #[ cfg( feature = "html_support" ) ]
    { self.named_color = None; }
    self
  }

  /// Attach a semantic color by name. Equivalent to `.with_color( color.to_ansi() )`.
  ///
  /// Stores both the ANSI string (for `render()`) and, when `html_support` is enabled,
  /// the original `Color` value so that `render_html()` can produce a typed CSS span.
  ///
  /// # Example
  ///
  /// ```
  /// use color_tools::{ DecoratedText, Color };
  /// let ct = DecoratedText::from( "warn" ).with_color_named( Color::Yellow );
  /// assert_eq!( ct.render(), "\x1b[33mwarn\x1b[0m" );
  /// ```
  #[ must_use ]
  pub fn with_color_named( self, color : Color ) -> Self
  {
    let ansi = color.to_ansi();
    let result = self.with_color( ansi );
    #[ cfg( feature = "html_support" ) ]
    let result = { let mut r = result; r.named_color = Some( color ); r };
    result
  }

  /// Apply bold (SGR 1). Returns `self` for builder chaining.
  ///
  /// # Example
  ///
  /// ```
  /// use color_tools::DecoratedText;
  /// let ct = DecoratedText::from( "title" ).with_bold();
  /// assert_eq!( ct.render(), "\x1b[1mtitle\x1b[0m" );
  /// ```
  #[ must_use ]
  pub fn with_bold( mut self ) -> Self
  {
    self.bold = true;
    self
  }

  /// Apply dim/faint (SGR 2). Returns `self` for builder chaining.
  ///
  /// # Example
  ///
  /// ```
  /// use color_tools::DecoratedText;
  /// let ct = DecoratedText::from( "note" ).with_dim();
  /// assert_eq!( ct.render(), "\x1b[2mnote\x1b[0m" );
  /// ```
  #[ must_use ]
  pub fn with_dim( mut self ) -> Self
  {
    self.dim = true;
    self
  }

  /// Produce browser-usable HTML output.
  ///
  /// Plain text (no `with_color_named`): returns HTML-escaped text with no wrapper.
  /// Named-color text (via `with_color_named`): returns `<span style="color: {css}">escaped_text</span>`.
  /// Raw-string-color text (via `with_color`): returns plain escaped text — CSS cannot be
  /// derived from raw ANSI bytes; use `render()` for terminal output in that case.
  ///
  /// # Example
  ///
  /// ```
  /// # #[cfg(feature = "html_support")]
  /// # {
  /// use color_tools::{ DecoratedText, Color };
  /// let ct = DecoratedText::from( "warn" ).with_color_named( Color::Yellow );
  /// assert_eq!( ct.render_html(), "<span style=\"color: yellow\">warn</span>" );
  ///
  /// let plain = DecoratedText::from( "ok" );
  /// assert_eq!( plain.render_html(), "ok" );
  ///
  /// let bold = DecoratedText::from( "title" ).with_bold();
  /// assert_eq!( bold.render_html(), "<span style=\"font-weight: bold\">title</span>" );
  /// # }
  /// ```
  #[ cfg( feature = "html_support" ) ]
  #[ must_use ]
  pub fn render_html( &self ) -> String
  {
    let escaped = self.text
      .replace( '&', "&amp;" )
      .replace( '<', "&lt;" )
      .replace( '>', "&gt;" );
    let mut styles = Vec::new();
    if let Some( c ) = &self.named_color { styles.push( format!( "color: {}", c.to_css() ) ); }
    if self.bold { styles.push( "font-weight: bold".to_owned() ); }
    // CSS has no native SGR-dim/faint equivalent; reduced opacity is the
    // closest visual analogue and mirrors how `to_css()` already documents
    // lossy terminal-to-web mappings for the Bright variants.
    if self.dim { styles.push( "opacity: 0.7".to_owned() ); }
    if styles.is_empty()
    {
      escaped
    }
    else
    {
      format!( "<span style=\"{}\">{escaped}</span>", styles.join( "; " ) )
    }
  }

  /// Render to a terminal string.
  ///
  /// When colored and/or styled (bold/dim): `bold_prefix + dim_prefix + color_prefix + text + "\x1b[0m"`.
  /// Bold and dim are emitted as their own SGR sequences ahead of `color`, cumulative
  /// with it rather than merged into one combined SGR parameter list — every real
  /// terminal treats consecutive SGR sequences as cumulative state, so this is
  /// visually identical to a single combined sequence.
  /// When neither color, bold, nor dim is set: plain `text` clone with no escape codes injected.
  ///
  /// # Example
  ///
  /// ```
  /// use color_tools::DecoratedText;
  /// let plain = DecoratedText::from( "ok" );
  /// assert_eq!( plain.render(), "ok" );
  ///
  /// let colored = DecoratedText::from( "ok" ).with_color( "\x1b[32m" );
  /// assert_eq!( colored.render(), "\x1b[32mok\x1b[0m" );
  ///
  /// let bold_colored = DecoratedText::from( "warn" ).with_bold().with_color( "\x1b[33m" );
  /// assert_eq!( bold_colored.render(), "\x1b[1m\x1b[33mwarn\x1b[0m" );
  /// ```
  #[ must_use ]
  pub fn render( &self ) -> String
  {
    // `color.is_some()` alone (even `Some("")`) must still trigger a reset —
    // see t44 in tests/decorated_text_test.rs for the documented design boundary.
    let active = self.color.is_some() || self.bold || self.dim;
    if !active
    {
      return self.text.clone();
    }
    let mut prefix = String::new();
    if self.bold { prefix.push_str( "\x1b[1m" ); }
    if self.dim  { prefix.push_str( "\x1b[2m" ); }
    if let Some( ref c ) = self.color { prefix.push_str( c ); }
    format!( "{prefix}{}\x1b[0m", self.text )
  }

  /// Returns `true` when an ANSI color prefix is attached.
  ///
  /// # Example
  ///
  /// ```
  /// use color_tools::DecoratedText;
  /// let plain = DecoratedText::from( "text" );
  /// assert!( !plain.is_colored() );
  /// let colored = plain.with_color( "\x1b[31m" );
  /// assert!( colored.is_colored() );
  /// ```
  #[ must_use ]
  pub fn is_colored( &self ) -> bool
  {
    self.color.is_some()
  }

  /// Returns `true` when the text content is empty.
  ///
  /// Tests `self.text.is_empty()` — NOT `self.render().is_empty()`.
  /// A colored empty text (e.g. `from("").with_color(...)`) is still considered empty
  /// because no visible content will be displayed.
  ///
  /// # Example
  ///
  /// ```
  /// use color_tools::DecoratedText;
  /// assert!( DecoratedText::from( "" ).is_empty() );
  /// assert!( DecoratedText::from( "" ).with_color( "\x1b[33m" ).is_empty() );
  /// assert!( !DecoratedText::from( "x" ).is_empty() );
  /// ```
  #[ must_use ]
  pub fn is_empty( &self ) -> bool
  {
    self.text.is_empty()
  }
}

impl From< String > for DecoratedText
{
  fn from( text : String ) -> Self
  {
    Self
    {
      text,
      color : None,
      bold : false,
      dim : false,
      #[ cfg( feature = "html_support" ) ]
      named_color : None,
    }
  }
}

impl From< &str > for DecoratedText
{
  fn from( text : &str ) -> Self
  {
    Self
    {
      text : text.to_owned(),
      color : None,
      bold : false,
      dim : false,
      #[ cfg( feature = "html_support" ) ]
      named_color : None,
    }
  }
}

impl From< DecoratedText > for String
{
  fn from( ct : DecoratedText ) -> Self
  {
    ct.render()
  }
}

impl core::fmt::Display for DecoratedText
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    f.write_str( &self.render() )
  }
}
