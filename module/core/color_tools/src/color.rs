/// Semantic color intent for `DecoratedText`.
///
/// Use `to_ansi()` to produce the ANSI SGR opening sequence for terminal rendering.
/// Use `DecoratedText::with_color_named()` to attach a semantic color without raw strings.
///
/// Variants cover 4-bit named colors (SGR 30–37 normal, 90–97 bright),
/// 256-color palette (`Ansi256`), and 24-bit true color (`Rgb`).
///
/// # Example
///
/// ```
/// use color_tools::Color;
/// assert_eq!( Color::Yellow.to_ansi(), "\x1b[33m" );
/// assert_eq!( Color::Rgb( 255, 165, 0 ).to_ansi(), "\x1b[38;2;255;165;0m" );
/// assert_eq!( Color::Ansi256( 208 ).to_ansi(), "\x1b[38;5;208m" );
/// ```
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum Color
{
  /// 4-bit foreground black (SGR 30).
  Black,
  /// 4-bit foreground red (SGR 31).
  Red,
  /// 4-bit foreground green (SGR 32).
  Green,
  /// 4-bit foreground yellow (SGR 33).
  Yellow,
  /// 4-bit foreground blue (SGR 34).
  Blue,
  /// 4-bit foreground magenta (SGR 35).
  Magenta,
  /// 4-bit foreground cyan (SGR 36).
  Cyan,
  /// 4-bit foreground white (SGR 37).
  White,
  /// Bright/intense black — typically dark grey (SGR 90).
  BrightBlack,
  /// Bright/intense red (SGR 91).
  BrightRed,
  /// Bright/intense green (SGR 92).
  BrightGreen,
  /// Bright/intense yellow (SGR 93).
  BrightYellow,
  /// Bright/intense blue (SGR 94).
  BrightBlue,
  /// Bright/intense magenta (SGR 95).
  BrightMagenta,
  /// Bright/intense cyan (SGR 96).
  BrightCyan,
  /// Bright/intense white (SGR 97).
  BrightWhite,
  /// 256-color palette index (0–255). SGR format: `38;5;N`.
  Ansi256( u8 ),
  /// 24-bit true color (red, green, blue each 0–255). SGR format: `38;2;R;G;B`.
  Rgb( u8, u8, u8 ),
}

impl Color
{
  /// Produce the ANSI SGR opening sequence for this color variant.
  ///
  /// Returns the complete prefix: ESC `[` + params + `m`.
  /// Intended for use with `DecoratedText::with_color()` or `with_color_named()`.
  ///
  /// # Example
  ///
  /// ```
  /// use color_tools::Color;
  /// let yellow = Color::Yellow.to_ansi();
  /// assert_eq!( yellow, "\x1b[33m" );
  /// ```
  #[ must_use ]
  pub fn to_ansi( &self ) -> String
  {
    match self
    {
      Color::Black         => "\x1b[30m".to_owned(),
      Color::Red           => "\x1b[31m".to_owned(),
      Color::Green         => "\x1b[32m".to_owned(),
      Color::Yellow        => "\x1b[33m".to_owned(),
      Color::Blue          => "\x1b[34m".to_owned(),
      Color::Magenta       => "\x1b[35m".to_owned(),
      Color::Cyan          => "\x1b[36m".to_owned(),
      Color::White         => "\x1b[37m".to_owned(),
      Color::BrightBlack   => "\x1b[90m".to_owned(),
      Color::BrightRed     => "\x1b[91m".to_owned(),
      Color::BrightGreen   => "\x1b[92m".to_owned(),
      Color::BrightYellow  => "\x1b[93m".to_owned(),
      Color::BrightBlue    => "\x1b[94m".to_owned(),
      Color::BrightMagenta => "\x1b[95m".to_owned(),
      Color::BrightCyan    => "\x1b[96m".to_owned(),
      Color::BrightWhite   => "\x1b[97m".to_owned(),
      Color::Ansi256( n )       => format!( "\x1b[38;5;{n}m" ),
      Color::Rgb( r, g, b ) => format!( "\x1b[38;2;{r};{g};{b}m" ),
    }
  }
}
