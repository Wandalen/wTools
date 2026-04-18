/// Text string paired with an optional ANSI color prefix.
///
/// `From< String >` and `From< &str >` are transparent — no color allocation, no escape injection.
/// Use `.with_color( "\x1b[33m" )` to attach a color; `.render()` appends the ANSI reset `"\x1b[0m"`.
///
/// # Examples
///
/// ```
/// use color_tools::ColorfulText;
///
/// let plain : ColorfulText = "hello".into();
/// assert_eq!( plain.render(), "hello" );
///
/// let colored = ColorfulText::from( "warn" ).with_color( "\x1b[33m" );
/// assert!( colored.render().starts_with( "\x1b[33m" ) );
/// assert!( colored.render().ends_with( "\x1b[0m" ) );
/// ```
#[ derive( Debug, Clone, PartialEq, Eq, Default ) ]
#[ cfg_attr( feature = "serde_support", derive( serde::Serialize, serde::Deserialize ) ) ]
pub struct ColorfulText
{
  /// The raw text content.
  pub text  : String,
  /// Optional ANSI escape prefix (e.g. `"\x1b[33m"` for yellow).
  pub color : Option< String >,
}

impl ColorfulText
{
  /// Attach an ANSI color prefix. Returns `self` for builder chaining.
  ///
  /// # Example
  ///
  /// ```
  /// use color_tools::ColorfulText;
  /// let ct = ColorfulText::from( "err" ).with_color( "\x1b[31m" );
  /// assert!( ct.is_colored() );
  /// ```
  #[ must_use ]
  pub fn with_color( mut self, ansi : impl Into< String > ) -> Self
  {
    self.color = Some( ansi.into() );
    self
  }

  /// Render to a terminal string.
  ///
  /// When colored: `color_prefix + text + "\x1b[0m"`.
  /// When uncolored: plain `text` clone with no escape codes injected.
  ///
  /// # Example
  ///
  /// ```
  /// use color_tools::ColorfulText;
  /// let plain = ColorfulText::from( "ok" );
  /// assert_eq!( plain.render(), "ok" );
  ///
  /// let colored = ColorfulText::from( "ok" ).with_color( "\x1b[32m" );
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
  /// use color_tools::ColorfulText;
  /// let plain = ColorfulText::from( "text" );
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
  /// use color_tools::ColorfulText;
  /// assert!( ColorfulText::from( "" ).is_empty() );
  /// assert!( ColorfulText::from( "" ).with_color( "\x1b[33m" ).is_empty() );
  /// assert!( !ColorfulText::from( "x" ).is_empty() );
  /// ```
  #[ must_use ]
  pub fn is_empty( &self ) -> bool
  {
    self.text.is_empty()
  }
}

impl From< String > for ColorfulText
{
  fn from( text : String ) -> Self
  {
    Self { text, color : None }
  }
}

impl From< &str > for ColorfulText
{
  fn from( text : &str ) -> Self
  {
    Self { text : text.to_owned(), color : None }
  }
}

impl From< ColorfulText > for String
{
  fn from( ct : ColorfulText ) -> Self
  {
    ct.render()
  }
}

impl core::fmt::Display for ColorfulText
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    f.write_str( &self.render() )
  }
}
