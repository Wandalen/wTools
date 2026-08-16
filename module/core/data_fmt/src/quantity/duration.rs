//! Fixed 6-visible-column duration/age formatting.

use super::{ QuantityStyle, styled_unit };

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
