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
    match &self.named_color
    {
      Some( c ) => format!( "<span style=\"color: {}\">{escaped}</span>", c.to_css() ),
      None      => escaped,
    }
  }

  /// Render to a terminal string.
  ///
  /// When colored: `color_prefix + text + "\x1b[0m"`.
  /// When uncolored: plain `text` clone with no escape codes injected.
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
  /// ```
  #[ must_use ]
  pub fn render( &self ) -> String
  {
    match self.color
    {
      Some( ref c ) => format!( "{}{}\x1b[0m", c, self.text ),
      None          => self.text.clone(),
    }
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
