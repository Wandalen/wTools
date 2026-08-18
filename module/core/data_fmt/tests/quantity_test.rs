//! Golden tests for the feature-gated `quantity` formatting module.
//!
//! Covers the six formatter families — fixed-width `duration_6ch`,
//! `number_compact`, `bytes_iec` and variable-width `duration_human`,
//! `duration_ms`, `bytes_human` — plus the `QuantityStyle::resolve` policy and,
//! under the opt-in `quantity_parse` feature, the `parse_duration` inverse. The
//! load-bearing invariant for `duration_6ch` is that its **visible** width (ANSI
//! escapes stripped, measured via the crate's public `visual_len`) is always
//! exactly 6 columns, in both `Plain` and `Colored` styles; the variable-width
//! formatters instead assert per-band golden strings and colored/plain glyph
//! parity.

#![ cfg( feature = "quantity" ) ]

use data_fmt::{ duration_6ch, duration_human, duration_ms, number_compact, bytes_iec, bytes_human, QuantityStyle, visual_len };

// ── duration_6ch: band-selection golden values (Plain) ────────────────────────

#[ test ]
fn duration_minutes_seconds_band()
{
  assert_eq!( duration_6ch( 0, QuantityStyle::Plain ), "00m00s" );
  assert_eq!( duration_6ch( 5, QuantityStyle::Plain ), "00m05s" );
  assert_eq!( duration_6ch( 59, QuantityStyle::Plain ), "00m59s" );
  assert_eq!( duration_6ch( 146, QuantityStyle::Plain ), "02m26s" );
  assert_eq!( duration_6ch( 3599, QuantityStyle::Plain ), "59m59s" );
}

#[ test ]
fn duration_hours_minutes_band()
{
  assert_eq!( duration_6ch( 3600, QuantityStyle::Plain ), "01h00m" );
  assert_eq!( duration_6ch( 12_060, QuantityStyle::Plain ), "03h21m" );
  assert_eq!( duration_6ch( 36_480, QuantityStyle::Plain ), "10h08m" );
  assert_eq!( duration_6ch( 86_399, QuantityStyle::Plain ), "23h59m" );
}

#[ test ]
fn duration_days_hours_band()
{
  assert_eq!( duration_6ch( 86_400, QuantityStyle::Plain ), "01d00h" );
  assert_eq!( duration_6ch( 93_600, QuantityStyle::Plain ), "01d02h" );
  assert_eq!( duration_6ch( 604_799, QuantityStyle::Plain ), "06d23h" );
}

#[ test ]
fn duration_weeks_days_band()
{
  assert_eq!( duration_6ch( 604_800, QuantityStyle::Plain ), "01w00d" );
  assert_eq!( duration_6ch( 1_209_600, QuantityStyle::Plain ), "02w00d" );
  // 99w06d exactly — genuine (not clamped).
  assert_eq!( duration_6ch( 60_393_600, QuantityStyle::Plain ), "99w06d" );
}

#[ test ]
fn duration_clamps_at_99w06d()
{
  // 100 weeks and beyond clamp to the largest representable value.
  assert_eq!( duration_6ch( 60_480_000, QuantityStyle::Plain ), "99w06d" );
  assert_eq!( duration_6ch( 100_000_000, QuantityStyle::Plain ), "99w06d" );
  assert_eq!( duration_6ch( u64::MAX, QuantityStyle::Plain ), "99w06d" );
}

#[ test ]
fn duration_width_is_always_6_visible_columns()
{
  // Representative values spanning every band + boundaries. The visible width
  // (ANSI stripped) must be exactly 6 in both styles.
  let samples : [ u64; 14 ] =
  [
    0, 5, 146, 3599, 3600, 12_060, 86_399, 86_400,
    93_600, 604_799, 604_800, 60_393_600, 60_480_000, u64::MAX,
  ];
  for secs in samples
  {
    let plain = duration_6ch( secs, QuantityStyle::Plain );
    let colored = duration_6ch( secs, QuantityStyle::Colored );
    assert_eq!( visual_len( &plain ), 6, "plain width for secs={secs}: {plain:?}" );
    assert_eq!( visual_len( &colored ), 6, "colored width for secs={secs}: {colored:?}" );
  }
}

#[ test ]
fn duration_colored_dims_units_only()
{
  let colored = duration_6ch( 146, QuantityStyle::Colored );
  println!( "colored duration: {colored:?}" );
  // Same visible glyphs as plain, but carrying ANSI escapes.
  assert_eq!( visual_len( &colored ), 6 );
  assert!( colored.contains( '\x1b' ), "colored form must carry ANSI escapes" );
  assert!( colored.contains( '2' ) && colored.contains( '6' ), "digits present" );
  // Plain form is clean ASCII, no escapes.
  let plain = duration_6ch( 146, QuantityStyle::Plain );
  assert!( !plain.contains( '\x1b' ), "plain form must not carry ANSI escapes" );
}

// ── number_compact: SI magnitude golden values ────────────────────────────────

#[ test ]
fn number_below_1000_is_verbatim()
{
  assert_eq!( number_compact( 0, QuantityStyle::Plain ), "0" );
  assert_eq!( number_compact( 42, QuantityStyle::Plain ), "42" );
  assert_eq!( number_compact( 999, QuantityStyle::Plain ), "999" );
}

#[ test ]
fn number_scales_to_magnitude()
{
  assert_eq!( number_compact( 1_000, QuantityStyle::Plain ), "1k" );
  assert_eq!( number_compact( 14_464, QuantityStyle::Plain ), "14k" );
  assert_eq!( number_compact( 1_000_000, QuantityStyle::Plain ), "1M" );
  assert_eq!( number_compact( 5_000_000, QuantityStyle::Plain ), "5M" );
  assert_eq!( number_compact( 26_301_958, QuantityStyle::Plain ), "26.3M" );
  assert_eq!( number_compact( 1_000_000_000, QuantityStyle::Plain ), "1G" );
  assert_eq!( number_compact( 2_500_000_000, QuantityStyle::Plain ), "2.5G" );
}

#[ test ]
fn number_rounding_rollover_promotes()
{
  // 999_999 rounds to "1000" in the k tier -> promoted to "1M", never "1000k".
  assert_eq!( number_compact( 999_999, QuantityStyle::Plain ), "1M" );
}

#[ test ]
fn number_colored_dims_unit_only()
{
  let colored = number_compact( 26_301_958, QuantityStyle::Colored );
  println!( "colored number: {colored:?}" );
  // "26.3" + dimmed "M" -> 5 visible columns, same as the plain form's length.
  assert_eq!( visual_len( &colored ), "26.3M".chars().count() );
  assert!( colored.contains( '\x1b' ), "colored form must carry ANSI escapes" );
  assert!( colored.starts_with( "26.3" ), "digits are not dimmed" );
}

// ── bytes_iec: IEC binary magnitude golden values ─────────────────────────────

#[ test ]
fn bytes_below_1024_is_verbatim()
{
  assert_eq!( bytes_iec( 0, QuantityStyle::Plain ), "0B" );
  assert_eq!( bytes_iec( 512, QuantityStyle::Plain ), "512B" );
  assert_eq!( bytes_iec( 1023, QuantityStyle::Plain ), "1023B" );
}

#[ test ]
fn bytes_scales_to_binary_magnitude()
{
  assert_eq!( bytes_iec( 1024, QuantityStyle::Plain ), "1K" );
  assert_eq!( bytes_iec( 1536, QuantityStyle::Plain ), "1.5K" );
  assert_eq!( bytes_iec( 1_048_576, QuantityStyle::Plain ), "1M" );
  assert_eq!( bytes_iec( 1_572_864, QuantityStyle::Plain ), "1.5M" );
  assert_eq!( bytes_iec( 1_073_741_824, QuantityStyle::Plain ), "1G" );
}

#[ test ]
fn bytes_rounding_rollover_promotes()
{
  // 1 MiB minus one byte rounds up out of the K tier -> "1M", never "1024K".
  assert_eq!( bytes_iec( 1_048_575, QuantityStyle::Plain ), "1M" );
}

#[ test ]
fn bytes_colored_dims_unit_only()
{
  let colored = bytes_iec( 1536, QuantityStyle::Colored );
  println!( "colored bytes: {colored:?}" );
  assert_eq!( visual_len( &colored ), "1.5K".chars().count() );
  assert!( colored.contains( '\x1b' ), "colored form must carry ANSI escapes" );
  assert!( colored.starts_with( "1.5" ), "digits are not dimmed" );
}

// ── QuantityStyle::resolve policy ─────────────────────────────────────────────

#[ test ]
fn resolve_non_tty_is_always_plain()
{
  // Non-TTY forces Plain regardless of ambient NO_COLOR — deterministic.
  assert_eq!( QuantityStyle::resolve( false ), QuantityStyle::Plain );
}

// ── duration_6ch: band-edge carries and exact boundaries ──────────────────────

#[ test ]
fn duration_minute_and_hour_carries()
{
  assert_eq!( duration_6ch( 60, QuantityStyle::Plain ), "01m00s" );
  assert_eq!( duration_6ch( 61, QuantityStyle::Plain ), "01m01s" );
  assert_eq!( duration_6ch( 3660, QuantityStyle::Plain ), "01h01m" );
  assert_eq!( duration_6ch( 82_800, QuantityStyle::Plain ), "23h00m" );
}

#[ test ]
fn duration_day_and_week_carries()
{
  assert_eq!( duration_6ch( 90_000, QuantityStyle::Plain ), "01d01h" );
  assert_eq!( duration_6ch( 691_200, QuantityStyle::Plain ), "01w01d" );
  // 99 weeks exactly — days component is zero, still genuine (not clamped).
  assert_eq!( duration_6ch( 59_875_200, QuantityStyle::Plain ), "99w00d" );
}

// ── number_compact: tier rounding and cross-tier rollover ─────────────────────

#[ test ]
fn number_k_tier_rounding_detail()
{
  assert_eq!( number_compact( 1_000, QuantityStyle::Plain ), "1k" );
  assert_eq!( number_compact( 1_499, QuantityStyle::Plain ), "1k" );
  assert_eq!( number_compact( 1_500, QuantityStyle::Plain ), "2k" );
  assert_eq!( number_compact( 9_999, QuantityStyle::Plain ), "10k" );
  assert_eq!( number_compact( 12_345, QuantityStyle::Plain ), "12k" );
  assert_eq!( number_compact( 100_000, QuantityStyle::Plain ), "100k" );
  assert_eq!( number_compact( 999_000, QuantityStyle::Plain ), "999k" );
}

#[ test ]
fn number_large_tiers_and_rollover()
{
  assert_eq!( number_compact( 1_000_000, QuantityStyle::Plain ), "1M" );
  assert_eq!( number_compact( 1_500_000, QuantityStyle::Plain ), "1.5M" );
  assert_eq!( number_compact( 1_500_000_000, QuantityStyle::Plain ), "1.5G" );
  // 999_999_999 rounds up through the M tier into G — "1G", never "1000M".
  assert_eq!( number_compact( 999_999_999, QuantityStyle::Plain ), "1G" );
  assert_eq!( number_compact( 1_000_000_000_000, QuantityStyle::Plain ), "1T" );
  assert_eq!( number_compact( 5_500_000_000_000, QuantityStyle::Plain ), "5.5T" );
}

#[ test ]
fn number_huge_counts_widen_at_top_tier()
{
  // Beyond the T tier there is no larger unit, so the mantissa simply widens.
  assert_eq!( number_compact( 2_000_000_000_000_000, QuantityStyle::Plain ), "2000T" );
}

// ── bytes_iec: boundaries and higher binary tiers ─────────────────────────────

#[ test ]
fn bytes_boundaries_and_tiers()
{
  assert_eq!( bytes_iec( 1023, QuantityStyle::Plain ), "1023B" );
  assert_eq!( bytes_iec( 1024, QuantityStyle::Plain ), "1K" );
  assert_eq!( bytes_iec( 2048, QuantityStyle::Plain ), "2K" );
  assert_eq!( bytes_iec( 10_240, QuantityStyle::Plain ), "10K" );
  assert_eq!( bytes_iec( 3_221_225_472, QuantityStyle::Plain ), "3G" );
  assert_eq!( bytes_iec( 1_099_511_627_776, QuantityStyle::Plain ), "1T" );
  assert_eq!( bytes_iec( 5_497_558_138_880, QuantityStyle::Plain ), "5T" );
}

// ── colored/plain parity: color adds only zero-width escapes ───────────────────

/// Strip ANSI SGR sequences (`\x1b[...m`) so a colored result can be compared to
/// its plain counterpart glyph-for-glyph.
fn strip_ansi( s : &str ) -> String
{
  let mut out = String::with_capacity( s.len() );
  let mut in_escape = false;
  for c in s.chars()
  {
    if in_escape
    {
      if c == 'm'
      {
        in_escape = false;
      }
    }
    else if c == '\x1b'
    {
      in_escape = true;
    }
    else
    {
      out.push( c );
    }
  }
  out
}

#[ test ]
fn colored_stripped_equals_plain()
{
  // For every formatter, Colored with ANSI removed must equal Plain exactly —
  // proving color adds only zero-width escapes, never visible glyphs.
  let durations : [ u64; 5 ] = [ 5, 146, 36_480, 604_800, 60_480_000 ];
  for secs in durations
  {
    let plain = duration_6ch( secs, QuantityStyle::Plain );
    let colored = duration_6ch( secs, QuantityStyle::Colored );
    assert_eq!( strip_ansi( &colored ), plain, "duration secs={secs}" );
  }

  let counts : [ u64; 5 ] = [ 42, 14_464, 999_999, 26_301_958, 1_000_000_000 ];
  for n in counts
  {
    let plain = number_compact( n, QuantityStyle::Plain );
    let colored = number_compact( n, QuantityStyle::Colored );
    assert_eq!( strip_ansi( &colored ), plain, "number n={n}" );
  }

  let sizes : [ u64; 5 ] = [ 512, 1024, 1536, 1_048_576, 3_221_225_472 ];
  for n in sizes
  {
    let plain = bytes_iec( n, QuantityStyle::Plain );
    let colored = bytes_iec( n, QuantityStyle::Colored );
    assert_eq!( strip_ansi( &colored ), plain, "bytes n={n}" );
  }
}

#[ test ]
fn colored_below_threshold_unit_handling()
{
  // Sub-threshold values still carry a unit letter (s / B) that must be dimmed.
  let d = duration_6ch( 5, QuantityStyle::Colored );
  assert!( d.contains( '\x1b' ), "duration unit must be dimmed" );
  assert_eq!( strip_ansi( &d ), "00m05s" );

  let b = bytes_iec( 512, QuantityStyle::Colored );
  assert!( b.contains( '\x1b' ), "byte unit must be dimmed" );
  assert_eq!( strip_ansi( &b ), "512B" );

  // A sub-1000 count has NO unit letter, so Colored == Plain (nothing to dim).
  let n = number_compact( 42, QuantityStyle::Colored );
  assert_eq!( n, "42" );
  assert!( !n.contains( '\x1b' ), "no unit -> no escapes" );
}

// ── duration_human: variable-width tier selection (Plain) ─────────────────────

#[ test ]
fn duration_human_seconds_and_minutes()
{
  assert_eq!( duration_human( 0, QuantityStyle::Plain ), "0s" );
  assert_eq!( duration_human( 45, QuantityStyle::Plain ), "45s" );
  assert_eq!( duration_human( 59, QuantityStyle::Plain ), "59s" );
  // Trailing zero-unit dropped: 60s and 120s carry no "0s" tail.
  assert_eq!( duration_human( 60, QuantityStyle::Plain ), "1m" );
  assert_eq!( duration_human( 90, QuantityStyle::Plain ), "1m 30s" );
  assert_eq!( duration_human( 120, QuantityStyle::Plain ), "2m" );
  assert_eq!( duration_human( 3599, QuantityStyle::Plain ), "59m 59s" );
}

#[ test ]
fn duration_human_hours_and_days()
{
  assert_eq!( duration_human( 3600, QuantityStyle::Plain ), "1h" );       // 0m dropped
  assert_eq!( duration_human( 3660, QuantityStyle::Plain ), "1h 1m" );
  assert_eq!( duration_human( 3665, QuantityStyle::Plain ), "1h 1m" );    // sub-minute tier dropped
  assert_eq!( duration_human( 86_399, QuantityStyle::Plain ), "23h 59m" );
  assert_eq!( duration_human( 86_400, QuantityStyle::Plain ), "1d" );     // 0h dropped
  assert_eq!( duration_human( 90_000, QuantityStyle::Plain ), "1d 1h" );
  assert_eq!( duration_human( 90_061, QuantityStyle::Plain ), "1d 1h" );  // sub-hour tiers dropped
}

// ── duration_ms: sub-second precision + human fallback (Plain) ────────────────

#[ test ]
fn duration_ms_milliseconds_and_seconds()
{
  assert_eq!( duration_ms( 0, QuantityStyle::Plain ), "0ms" );
  assert_eq!( duration_ms( 500, QuantityStyle::Plain ), "500ms" );
  assert_eq!( duration_ms( 999, QuantityStyle::Plain ), "999ms" );
  assert_eq!( duration_ms( 1000, QuantityStyle::Plain ), "1.00s" );
  assert_eq!( duration_ms( 1500, QuantityStyle::Plain ), "1.50s" );
  assert_eq!( duration_ms( 12_340, QuantityStyle::Plain ), "12.34s" );
}

#[ test ]
fn duration_ms_truncates_not_rounds_at_minute_edge()
{
  // Fix(BUG-1071): hundredths truncate, never round up across the minute edge.
  assert_eq!( duration_ms( 59_990, QuantityStyle::Plain ), "59.99s" );
  assert_eq!( duration_ms( 59_999, QuantityStyle::Plain ), "59.99s" );
  // >= 1 minute delegates to duration_human tiers.
  assert_eq!( duration_ms( 60_000, QuantityStyle::Plain ), "1m" );
  assert_eq!( duration_ms( 65_000, QuantityStyle::Plain ), "1m 5s" );
  assert_eq!( duration_ms( 125_000, QuantityStyle::Plain ), "2m 5s" );
}

// ── bytes_human: verbose binary sizes (Plain) ─────────────────────────────────

#[ test ]
fn bytes_human_sub_kib_counts()
{
  assert_eq!( bytes_human( 0, QuantityStyle::Plain ), "0 bytes" );
  assert_eq!( bytes_human( 1, QuantityStyle::Plain ), "1 byte" );   // singular special case
  assert_eq!( bytes_human( 2, QuantityStyle::Plain ), "2 bytes" );
  assert_eq!( bytes_human( 512, QuantityStyle::Plain ), "512 bytes" );
  assert_eq!( bytes_human( 1023, QuantityStyle::Plain ), "1023 bytes" );
}

#[ test ]
fn bytes_human_scales_with_two_decimals()
{
  assert_eq!( bytes_human( 1024, QuantityStyle::Plain ), "1.00 KB" );
  assert_eq!( bytes_human( 1536, QuantityStyle::Plain ), "1.50 KB" );
  assert_eq!( bytes_human( 1_048_576, QuantityStyle::Plain ), "1.00 MB" );
  assert_eq!( bytes_human( 2_621_440, QuantityStyle::Plain ), "2.50 MB" );
  assert_eq!( bytes_human( 1_073_741_824, QuantityStyle::Plain ), "1.00 GB" );
  assert_eq!( bytes_human( 1_610_612_736, QuantityStyle::Plain ), "1.50 GB" );
}

// ── prose formatters: colored/plain parity + unit dimming ─────────────────────

#[ test ]
fn prose_formatters_colored_stripped_equals_plain()
{
  let secs : [ u64; 6 ] = [ 0, 45, 90, 3600, 90_000, 90_061 ];
  for s in secs
  {
    assert_eq!
    (
      strip_ansi( &duration_human( s, QuantityStyle::Colored ) ),
      duration_human( s, QuantityStyle::Plain ),
      "duration_human secs={s}"
    );
  }

  let millis : [ u64; 6 ] = [ 0, 500, 1000, 1500, 59_990, 65_000 ];
  for ms in millis
  {
    assert_eq!
    (
      strip_ansi( &duration_ms( ms, QuantityStyle::Colored ) ),
      duration_ms( ms, QuantityStyle::Plain ),
      "duration_ms ms={ms}"
    );
  }

  let sizes : [ u64; 6 ] = [ 0, 1, 512, 1536, 1_048_576, 1_073_741_824 ];
  for n in sizes
  {
    assert_eq!
    (
      strip_ansi( &bytes_human( n, QuantityStyle::Colored ) ),
      bytes_human( n, QuantityStyle::Plain ),
      "bytes_human n={n}"
    );
  }
}

#[ test ]
fn prose_formatters_colored_dims_units_only()
{
  // Multi-tier duration: both unit letters dimmed, digits + separating space plain.
  let d = duration_human( 90, QuantityStyle::Colored );
  assert!( d.contains( '\x1b' ), "duration_human units must be dimmed" );
  assert_eq!( strip_ansi( &d ), "1m 30s" );

  // Sub-second ms: "ms" dimmed, digits plain.
  let ms = duration_ms( 500, QuantityStyle::Colored );
  assert!( ms.contains( '\x1b' ), "duration_ms unit must be dimmed" );
  assert_eq!( strip_ansi( &ms ), "500ms" );

  // Verbose bytes: spelled-out unit dimmed, count + separating space left plain.
  let b = bytes_human( 1536, QuantityStyle::Colored );
  assert!( b.contains( '\x1b' ), "bytes_human unit must be dimmed" );
  assert!( b.starts_with( "1.50 " ), "digits and separating space are not dimmed" );
  assert_eq!( strip_ansi( &b ), "1.50 KB" );
}

// ── parse_duration: inverse of the formatters (opt-in `quantity_parse`) ───────

#[ cfg( feature = "quantity_parse" ) ]
use data_fmt::{ parse_duration, DurationError };

#[ cfg( feature = "quantity_parse" ) ]
#[ test ]
fn parse_duration_single_units()
{
  // Assert on whole seconds — avoids constructing a `Duration` via the larger-unit
  // helpers (`from_hours`/`from_mins`) that post-date the workspace MSRV.
  assert_eq!( parse_duration( "1h" ).unwrap().as_secs(), 3600 );
  assert_eq!( parse_duration( "30m" ).unwrap().as_secs(), 1800 );
  assert_eq!( parse_duration( "45s" ).unwrap().as_secs(), 45 );
  assert_eq!( parse_duration( "7d" ).unwrap().as_secs(), 7 * 86_400 );
  assert_eq!( parse_duration( "2w" ).unwrap().as_secs(), 14 * 86_400 );
}

#[ cfg( feature = "quantity_parse" ) ]
#[ test ]
fn parse_duration_combined_units()
{
  assert_eq!( parse_duration( "1d6h" ).unwrap().as_secs(), 86_400 + 21_600 );
  assert_eq!( parse_duration( "1h30m" ).unwrap().as_secs(), 5400 );
  assert_eq!( parse_duration( "1d6h30m" ).unwrap().as_secs(), 86_400 + 21_600 + 1800 );
}

#[ cfg( feature = "quantity_parse" ) ]
#[ test ]
fn parse_duration_round_trips_with_duration_human()
{
  // parse_duration ∘ duration_human is identity on values duration_human renders
  // exactly — single- or two-tier with no dropped sub-tier remainder.
  for secs in [ 45u64, 90, 3600, 3660, 86_400, 90_000 ]
  {
    let text = duration_human( secs, QuantityStyle::Plain ).replace( ' ', "" );
    assert_eq!( parse_duration( &text ).unwrap().as_secs(), secs, "round-trip {text}" );
  }
}

#[ cfg( feature = "quantity_parse" ) ]
#[ test ]
fn parse_duration_rejects_empty_and_garbage()
{
  assert_eq!( parse_duration( "" ), Err( DurationError::Empty ) );
  assert_eq!( parse_duration( "soon" ), Err( DurationError::InvalidFormat( "soon".to_string() ) ) );
  assert_eq!( parse_duration( "1x" ), Err( DurationError::InvalidFormat( "1x".to_string() ) ) );
}
