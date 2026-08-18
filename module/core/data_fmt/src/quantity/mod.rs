//! Terminal-aware quantity formatting.
//!
//! Renders durations/ages, humanized counts, and byte sizes as compact strings
//! for CLI output — fixed-width forms for aligned columns ([`duration_6ch`],
//! [`bytes_iec`]) and variable-width forms for inline prose ([`duration_human`],
//! [`duration_ms`], [`bytes_human`]). Each formatter takes a [`QuantityStyle`]
//! deciding whether unit letters are dimmed with ANSI color
//! ([`QuantityStyle::Colored`]) or emitted as plain ASCII
//! ([`QuantityStyle::Plain`]); the visible glyphs are identical either way.
//!
//! The formatting path never inspects the environment — the caller decides
//! `Plain` vs `Colored`. [`QuantityStyle::resolve`] is the one provided
//! convenience that folds a caller-supplied `is_tty` with the `NO_COLOR`
//! environment variable, so consumers share a single policy instead of each
//! re-deriving it.
//!
//! # Examples
//!
//! ```
//! # #[ cfg( feature = "quantity" ) ]
//! # {
//! use data_fmt::{ duration_6ch, duration_human, duration_ms, number_compact, bytes_iec, bytes_human, QuantityStyle };
//!
//! assert_eq!( duration_6ch( 146, QuantityStyle::Plain ), "02m26s" );
//! assert_eq!( duration_human( 3665, QuantityStyle::Plain ), "1h 1m" );
//! assert_eq!( duration_ms( 1500, QuantityStyle::Plain ), "1.50s" );
//! assert_eq!( number_compact( 26_301_958, QuantityStyle::Plain ), "26.3M" );
//! assert_eq!( bytes_iec( 1536, QuantityStyle::Plain ), "1.5K" );
//! assert_eq!( bytes_human( 1536, QuantityStyle::Plain ), "1.50 KB" );
//! # }
//! ```

mod duration;
mod number;

pub use duration::{ duration_6ch, duration_human, duration_ms };
pub use number::{ number_compact, bytes_iec, bytes_human };

use color_tools::DecoratedText;

/// ANSI SGR for a dimmed (gray) glyph — matches the crate's gray-key convention
/// (`ExpandedConfig::key_color`'s default).
const GRAY : &str = "\x1b[90m";

/// Whether quantity formatters dim unit letters with ANSI color or emit plain ASCII.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum QuantityStyle
{
  /// Plain ASCII — no ANSI escapes.
  Plain,
  /// Unit letters dimmed (gray) via ANSI SGR; digits left unstyled.
  Colored,
}

impl QuantityStyle
{
  /// Fold a caller-supplied `is_tty` with the `NO_COLOR` environment variable
  /// into a style: [`QuantityStyle::Colored`] only when `is_tty` is `true` **and**
  /// `NO_COLOR` is unset; otherwise [`QuantityStyle::Plain`] (`NO_COLOR` set to
  /// any value forces plain, per <https://no-color.org>).
  ///
  /// This is the single place the environment is consulted; the formatters stay
  /// pure so they remain trivially testable.
  pub fn resolve( is_tty : bool ) -> Self
  {
    if is_tty && std::env::var_os( "NO_COLOR" ).is_none()
    {
      Self::Colored
    }
    else
    {
      Self::Plain
    }
  }
}

/// Dim `unit` when `style` is `Colored`, else return it unchanged.
///
/// Built through [`DecoratedText`] rather than raw ANSI assembly, per the
/// crate's decorated-text rendering convention.
fn styled_unit( unit : &str, style : QuantityStyle ) -> String
{
  match style
  {
    QuantityStyle::Plain => unit.to_string(),
    QuantityStyle::Colored => DecoratedText::from( unit ).with_color( GRAY.to_string() ).render(),
  }
}
