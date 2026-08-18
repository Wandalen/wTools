//! Compact humanized number and byte-size formatting.

use super::{ QuantityStyle, styled_unit };

/// Format a count compactly with SI-style magnitude suffixes (`k`, `M`, `G`, `T`).
///
/// Values below 1000 render as the integer itself. Otherwise the value scales to
/// the largest fitting magnitude: the `k` tier shows no fractional digit and the
/// `M`/`G`/`T` tiers show up to one (a trailing `.0` is dropped). A rounding
/// roll-over (e.g. `999_999`) promotes to the next magnitude rather than showing
/// a four-digit mantissa. The unit letter is dimmed under
/// [`QuantityStyle::Colored`].
///
/// # Examples
///
/// ```
/// # #[ cfg( feature = "quantity" ) ]
/// # {
/// use data_fmt::{ number_compact, QuantityStyle };
/// assert_eq!( number_compact( 14_464, QuantityStyle::Plain ), "14k" );
/// assert_eq!( number_compact( 26_301_958, QuantityStyle::Plain ), "26.3M" );
/// assert_eq!( number_compact( 42, QuantityStyle::Plain ), "42" );
/// # }
/// ```
pub fn number_compact( n : u64, style : QuantityStyle ) -> String
{
  if n < 1_000
  {
    return n.to_string();
  }

  const UNITS : [ &str; 4 ] = [ "k", "M", "G", "T" ];
  let mut value = n as f64 / 1_000.0;
  let mut idx = 0usize;
  while value >= 1_000.0 && idx + 1 < UNITS.len()
  {
    value /= 1_000.0;
    idx += 1;
  }

  // The `k` tier shows no fractional digit; larger tiers show up to one.
  let decimals = usize::from( idx > 0 );
  let mut rendered = format_mantissa( value, decimals );

  // 0-decimal rounding can push e.g. 999.9 up to "1000"; promote to the next
  // unit so the mantissa never reaches four digits (when a larger unit exists).
  if rendered.starts_with( "1000" ) && idx + 1 < UNITS.len()
  {
    idx += 1;
    rendered = format_mantissa( value / 1_000.0, 1 );
  }

  format!( "{rendered}{}", styled_unit( UNITS[ idx ], style ) )
}

/// Format a byte count with IEC binary magnitudes (`K`, `M`, `G`, `T`, each 1024×
/// the previous).
///
/// Values below 1024 render as `NB`. Larger values scale to the biggest fitting
/// binary magnitude with up to one fractional digit (a trailing `.0` dropped); a
/// rounding roll-over promotes to the next magnitude. The unit letter is dimmed
/// under [`QuantityStyle::Colored`].
///
/// # Examples
///
/// ```
/// # #[ cfg( feature = "quantity" ) ]
/// # {
/// use data_fmt::{ bytes_iec, QuantityStyle };
/// assert_eq!( bytes_iec( 1536, QuantityStyle::Plain ), "1.5K" );
/// assert_eq!( bytes_iec( 512, QuantityStyle::Plain ), "512B" );
/// # }
/// ```
pub fn bytes_iec( n : u64, style : QuantityStyle ) -> String
{
  if n < 1024
  {
    return format!( "{n}{}", styled_unit( "B", style ) );
  }

  const UNITS : [ &str; 4 ] = [ "K", "M", "G", "T" ];
  let mut value = n as f64 / 1024.0;
  let mut idx = 0usize;
  while value >= 1024.0 && idx + 1 < UNITS.len()
  {
    value /= 1024.0;
    idx += 1;
  }

  let mut rendered = format_mantissa( value, 1 );
  if rendered.starts_with( "1024" ) && idx + 1 < UNITS.len()
  {
    idx += 1;
    rendered = format_mantissa( value / 1024.0, 1 );
  }

  format!( "{rendered}{}", styled_unit( UNITS[ idx ], style ) )
}

/// Format a byte count as a **verbose, variable-width** binary size: a value and
/// a spelled-out unit separated by a space, with 1024-based magnitudes labelled
/// `KB`/`MB`/`GB` (JEDEC style) and two fixed fractional digits at `KB` and above.
///
/// Sub-`KB` values render as an exact count — singular `1 byte`, plural
/// `N bytes` — so short sizes read naturally in prose (disk-usage and
/// peak-memory lines) rather than as a terse `NB`. Contrast [`bytes_iec`], which
/// packs the same magnitudes into a tight, single-letter, space-free form for
/// dense table cells. Magnitudes top out at `GB` (a multi-terabyte size widens
/// the `GB` mantissa rather than promoting). With [`QuantityStyle::Colored`] the
/// unit is dimmed and the digits and separating space are left unstyled.
///
/// | Range      | Layout    | Example                     |
/// |------------|-----------|-----------------------------|
/// | `0`        | `0 bytes` | `0` → `0 bytes`             |
/// | `1`        | `1 byte`  | `1` → `1 byte`              |
/// | `< 1 KiB`  | `N bytes` | `512` → `512 bytes`         |
/// | `< 1 MiB`  | `N.NN KB` | `1536` → `1.50 KB`          |
/// | `< 1 GiB`  | `N.NN MB` | `2_621_440` → `2.50 MB`     |
/// | `>= 1 GiB` | `N.NN GB` | `1_073_741_824` → `1.00 GB` |
///
/// # Examples
///
/// ```
/// # #[ cfg( feature = "quantity" ) ]
/// # {
/// use data_fmt::{ bytes_human, QuantityStyle };
/// assert_eq!( bytes_human( 1, QuantityStyle::Plain ), "1 byte" );
/// assert_eq!( bytes_human( 512, QuantityStyle::Plain ), "512 bytes" );
/// assert_eq!( bytes_human( 1536, QuantityStyle::Plain ), "1.50 KB" );
/// assert_eq!( bytes_human( 1_048_576, QuantityStyle::Plain ), "1.00 MB" );
/// # }
/// ```
pub fn bytes_human( bytes : u64, style : QuantityStyle ) -> String
{
  const KB : u64 = 1024;
  const MB : u64 = KB * 1024;
  const GB : u64 = MB * 1024;

  if bytes >= GB
  {
    format!( "{:.2} {}", bytes as f64 / GB as f64, styled_unit( "GB", style ) )
  }
  else if bytes >= MB
  {
    format!( "{:.2} {}", bytes as f64 / MB as f64, styled_unit( "MB", style ) )
  }
  else if bytes >= KB
  {
    format!( "{:.2} {}", bytes as f64 / KB as f64, styled_unit( "KB", style ) )
  }
  else if bytes == 1
  {
    format!( "1 {}", styled_unit( "byte", style ) )
  }
  else
  {
    format!( "{bytes} {}", styled_unit( "bytes", style ) )
  }
}

/// Render `value` with `decimals` fractional digits, dropping a trailing `.0`.
fn format_mantissa( value : f64, decimals : usize ) -> String
{
  let s = format!( "{value:.decimals$}" );
  if decimals > 0
  {
    if let Some( stripped ) = s.strip_suffix( ".0" )
    {
      return stripped.to_string();
    }
  }
  s
}
