//! Duration/age formatting — fixed-width columnar and variable-width prose forms.

use super::{ QuantityStyle, styled_unit };

#[ cfg( feature = "quantity_parse" ) ]
use core::time::Duration;
#[ cfg( feature = "quantity_parse" ) ]
use error_tools::dependency::thiserror;

const MINUTE : u64 = 60;
const HOUR : u64 = 60 * MINUTE;
const DAY : u64 = 24 * HOUR;
const WEEK : u64 = 7 * DAY;

/// Render one `NNu` segment: a zero-padded 2-digit value followed by its
/// (optionally dimmed) unit letter.
fn part( value : u64, unit : &str, style : QuantityStyle ) -> String
{
  format!( "{value:02}{}", styled_unit( unit, style ) )
}

/// Format a duration in seconds as a fixed **6-visible-character** `NNuNNu`
/// string: two zero-padded 2-digit numbers, each followed by a unit letter, with
/// the largest non-zero unit leading.
///
/// | Range   | Layout   | Example              |
/// |---------|----------|----------------------|
/// | `< 1h`  | `MMmSSs` | `146` → `02m26s`     |
/// | `< 1d`  | `HHhMMm` | `36_480` → `10h08m`  |
/// | `< 1w`  | `DDdHHh` | `93_600` → `01d02h`  |
/// | `>= 1w` | `WWwDDd` | `604_800` → `01w00d` |
///
/// Durations of 99 weeks 6 days or longer clamp to `99w06d`, the largest
/// representable value. The visible width is always exactly 6 columns whatever
/// the [`QuantityStyle`]; with [`QuantityStyle::Colored`] the unit letters are
/// dimmed and the digits are left unstyled.
///
/// # Examples
///
/// ```
/// # #[ cfg( feature = "quantity" ) ]
/// # {
/// use data_fmt::{ duration_6ch, QuantityStyle };
/// assert_eq!( duration_6ch( 5, QuantityStyle::Plain ), "00m05s" );
/// assert_eq!( duration_6ch( 604_800, QuantityStyle::Plain ), "01w00d" );
/// # }
/// ```
pub fn duration_6ch( secs : u64, style : QuantityStyle ) -> String
{
  if secs < HOUR
  {
    format!( "{}{}", part( secs / MINUTE, "m", style ), part( secs % MINUTE, "s", style ) )
  }
  else if secs < DAY
  {
    format!( "{}{}", part( secs / HOUR, "h", style ), part( ( secs % HOUR ) / MINUTE, "m", style ) )
  }
  else if secs < WEEK
  {
    format!( "{}{}", part( secs / DAY, "d", style ), part( ( secs % DAY ) / HOUR, "h", style ) )
  }
  else
  {
    let weeks = secs / WEEK;
    if weeks > 99
    {
      // Clamp: the largest representable value is 99 weeks 6 days.
      format!( "{}{}", part( 99, "w", style ), part( 6, "d", style ) )
    }
    else
    {
      format!( "{}{}", part( weeks, "w", style ), part( ( secs % WEEK ) / DAY, "d", style ) )
    }
  }
}

/// Render one `Nu` segment: a value with no zero-padding, followed by its
/// (optionally dimmed) unit — the variable-width counterpart of [`part`].
fn seg( value : u64, unit : &str, style : QuantityStyle ) -> String
{
  format!( "{value}{}", styled_unit( unit, style ) )
}

/// Format a whole-second span as a compact, **variable-width** human-readable
/// duration: the most-significant non-zero tier, followed by the next-lower tier
/// only when it is itself non-zero. Tiers are days, hours, minutes, seconds.
///
/// Unlike [`duration_6ch`] (fixed 6 columns, zero-padded, for aligned table
/// cells) this form targets inline prose — uptimes, ages, elapsed spans — where
/// a shortest-sensible rendering reads better than a padded one.
///
/// | Range   | Layout  | Example           |
/// |---------|---------|-------------------|
/// | `< 1m`  | `Ss`    | `45` → `45s`      |
/// | `< 1h`  | `Mm Ss` | `90` → `1m 30s`   |
/// | `< 1d`  | `Hh Mm` | `3665` → `1h 1m`  |
/// | `>= 1d` | `Dd Hh` | `90061` → `1d 1h` |
///
/// A lower tier that is exactly zero is dropped (`120` → `2m`, `3600` → `1h`,
/// `86_400` → `1d`), so the result carries at most two tiers and never a trailing
/// `0`-unit. With [`QuantityStyle::Colored`] the unit letters are dimmed and the
/// digits and separating space are left unstyled.
///
/// # Examples
///
/// ```
/// # #[ cfg( feature = "quantity" ) ]
/// # {
/// use data_fmt::{ duration_human, QuantityStyle };
/// assert_eq!( duration_human( 45, QuantityStyle::Plain ), "45s" );
/// assert_eq!( duration_human( 90, QuantityStyle::Plain ), "1m 30s" );
/// assert_eq!( duration_human( 3600, QuantityStyle::Plain ), "1h" );
/// assert_eq!( duration_human( 90_061, QuantityStyle::Plain ), "1d 1h" );
/// # }
/// ```
pub fn duration_human( secs : u64, style : QuantityStyle ) -> String
{
  let days = secs / DAY;
  let hours = ( secs % DAY ) / HOUR;
  let minutes = ( secs % HOUR ) / MINUTE;
  let seconds = secs % MINUTE;

  if days > 0
  {
    if hours > 0 { format!( "{} {}", seg( days, "d", style ), seg( hours, "h", style ) ) }
    else { seg( days, "d", style ) }
  }
  else if hours > 0
  {
    if minutes > 0 { format!( "{} {}", seg( hours, "h", style ), seg( minutes, "m", style ) ) }
    else { seg( hours, "h", style ) }
  }
  else if minutes > 0
  {
    if seconds > 0 { format!( "{} {}", seg( minutes, "m", style ), seg( seconds, "s", style ) ) }
    else { seg( minutes, "m", style ) }
  }
  else
  {
    seg( seconds, "s", style )
  }
}

/// Format a millisecond span with **sub-second precision** at the low end,
/// falling back to [`duration_human`] tiers once it reaches a minute.
///
/// | Range   | Layout  | Example            |
/// |---------|---------|--------------------|
/// | `< 1s`  | `Nms`   | `500` → `500ms`    |
/// | `< 1m`  | `N.NNs` | `1500` → `1.50s`   |
/// | `>= 1m` | (human) | `65_000` → `1m 5s` |
///
/// The seconds tier truncates to hundredths rather than rounding, so a value
/// like `59_990` renders `59.99s` and can never round up across the minute
/// boundary into `1m 0s` (Fix(BUG-1071)). With [`QuantityStyle::Colored`] the
/// unit letters are dimmed and the digits left unstyled.
///
/// # Examples
///
/// ```
/// # #[ cfg( feature = "quantity" ) ]
/// # {
/// use data_fmt::{ duration_ms, QuantityStyle };
/// assert_eq!( duration_ms( 500, QuantityStyle::Plain ), "500ms" );
/// assert_eq!( duration_ms( 1500, QuantityStyle::Plain ), "1.50s" );
/// assert_eq!( duration_ms( 59_990, QuantityStyle::Plain ), "59.99s" );
/// assert_eq!( duration_ms( 65_000, QuantityStyle::Plain ), "1m 5s" );
/// # }
/// ```
pub fn duration_ms( ms : u64, style : QuantityStyle ) -> String
{
  if ms < 1_000
  {
    format!( "{ms}{}", styled_unit( "ms", style ) )
  }
  else if ms < 60_000
  {
    // Truncate to hundredths (never round) so 59_990 stays "59.99s" and cannot
    // cross into the minute tier as "1m 0s" — Fix(BUG-1071).
    let hundredths = ms / 10;
    let whole = hundredths / 100;
    let frac = hundredths % 100;
    format!( "{whole}.{frac:02}{}", styled_unit( "s", style ) )
  }
  else
  {
    duration_human( ms / 1_000, style )
  }
}

// ── parsing: the inverse of the duration formatters (opt-in `quantity_parse`) ──

/// Errors returned by [`parse_duration`].
#[ cfg( feature = "quantity_parse" ) ]
#[ derive( thiserror::Error, Debug, Clone, PartialEq, Eq ) ]
pub enum DurationError
{
  /// The input string was empty.
  #[ error( "Duration string cannot be empty" ) ]
  Empty,
  /// The input was not a recognizable duration (e.g. `"soon"`, `"1x"`).
  #[ error( "Invalid duration '{0}': expected a form like '1h', '30m', '1d6h', '1w'" ) ]
  InvalidFormat( String ),
  /// The parsed duration exceeded the supported range (`> u64::MAX / 2` seconds).
  #[ error( "Duration '{0}' is too large (overflow)" ) ]
  Overflow( String ),
}

/// Parse a human-readable duration string into a [`core::time::Duration`] —
/// the inverse of [`duration_human`].
///
/// Accepts the compact forms a CLI user types (`"1h"`, `"30m"`, `"7d"`, `"2w"`),
/// including combined units (`"1d6h30m"`) and long-form units (`"90 seconds"`,
/// `"1hour 30min"`). Unit letters follow the `humantime` grammar: `s`, `m`
/// (minutes), `h`, `d`, `w`.
///
/// # Errors
///
/// - [`DurationError::Empty`] — the input string was empty.
/// - [`DurationError::InvalidFormat`] — the input was not a recognizable duration.
/// - [`DurationError::Overflow`] — the value exceeded `u64::MAX / 2` seconds.
///
/// # Examples
///
/// ```
/// # #[ cfg( feature = "quantity_parse" ) ]
/// # {
/// use data_fmt::parse_duration;
/// use core::time::Duration;
///
/// assert_eq!( parse_duration( "1h" ).unwrap(), Duration::from_secs( 3600 ) );
/// assert_eq!( parse_duration( "30m" ).unwrap(), Duration::from_secs( 1800 ) );
/// assert_eq!( parse_duration( "1d6h" ).unwrap(), Duration::from_secs( 86_400 + 21_600 ) );
/// # }
/// ```
#[ cfg( feature = "quantity_parse" ) ]
pub fn parse_duration( s : &str ) -> Result< Duration, DurationError >
{
  if s.is_empty()
  {
    return Err( DurationError::Empty );
  }

  let duration = humantime::parse_duration( s )
    .map_err( | _ | DurationError::InvalidFormat( s.to_string() ) )?;

  // humantime can yield very large spans; cap well below u64 overflow territory.
  if duration.as_secs() > u64::MAX / 2
  {
    return Err( DurationError::Overflow( s.to_string() ) );
  }

  Ok( duration )
}
