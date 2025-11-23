//! Core functionality tests for `time_tools`.
//!
//! Tests time retrieval functions across different unit variants.

#![ allow( missing_docs ) ]
#![ allow( clippy::similar_names ) ]
#[ allow( unused_imports ) ]
use test_tools ::*;
#[ allow( unused_imports ) ]
use time_tools as the_module;

/// Tests that `now()` returns a positive timestamp.
///
/// This verifies basic functionality of time retrieval.
#[ test ]
#[ cfg( feature = "time_now" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
fn test_now_positive()
{
  let got = the_module ::now();
  assert!( got > 0, "now() should return positive timestamp, got: {got}" );
}

/// Tests that `ms ::now()` returns consistent value with `now()`.
///
/// Both should return milliseconds, so values should be nearly identical.
#[ test ]
#[ cfg( feature = "time_now" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
fn test_ms_now_consistent()
{
  let got1 = the_module ::now();
  let got2 = the_module ::ms ::now();
  let diff = if got2 > got1 { got2 - got1 } else { got1 - got2 };
  assert!( diff <= 10, "now() and ms ::now() differ by {diff} ms (expected <=10)" );
}

/// Tests that `ns ::now()` conversion matches `now()`.
///
/// Nanoseconds divided by 1,000,000 should equal milliseconds.
#[ test ]
#[ cfg( feature = "time_now" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
fn test_ns_now_conversion()
{
  let got_ms = the_module ::now();
  let got_ns = the_module ::ns ::now();
  let got_ns_as_ms = got_ns / 1_000_000;
  let diff = if got_ns_as_ms > got_ms { got_ns_as_ms - got_ms } else { got_ms - got_ns_as_ms };
  assert!( diff <= 10, "ns ::now() conversion differs by {diff} ms (expected <=10)" );
}

/// Tests that `s ::now()` conversion matches `now()`.
///
/// Seconds times 1000 should equal milliseconds.
#[ test ]
#[ cfg( feature = "time_now" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
fn test_s_now_conversion()
{
  let got_ms = the_module ::now();
  let got_s = the_module ::s ::now();
  assert_eq!( got_ms / 1000, got_s, "s ::now() should equal now()/1000" );
}

//
// Edge Case Tests
//

/// Tests behavior near UNIX epoch boundary.
///
/// We cannot test system time < epoch without mocking system clock,
/// but we can verify that current time is well past epoch (positive).
///
/// # Test Coverage
///
/// - Verifies all time functions return positive values
/// - Documents expected behavior: panic if system time < 1970
#[ test ]
#[ cfg( feature = "time_now" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
fn test_epoch_boundary()
{
  let now_ms = the_module ::now();
  let now_s = the_module ::s ::now();
  let now_ns = the_module ::ns ::now();

  assert!( now_ms > 0, "now() should return positive (past 1970), got: {now_ms}" );
  assert!( now_s > 0, "s ::now() should return positive (past 1970), got: {now_s}" );
  assert!( now_ns > 0, "ns ::now() should return positive (past 1970), got: {now_ns}" );

  // All values should be reasonably large (well past 1970)
  // As of 2025, we're ~55 years past epoch = ~1.7 trillion milliseconds
  assert!( now_ms > 1_000_000_000_000, "Time should be well past epoch (year 2000+)" );
}

/// Tests cross-unit consistency over multiple samples.
///
/// Verifies that unit conversions remain consistent across
/// multiple time samples, not just a single snapshot.
///
/// # Test Coverage
///
/// - Multiple samples reduce flakiness from execution timing
/// - Verifies ms, s, ns conversions are mathematically consistent
#[ test ]
#[ cfg( feature = "time_now" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
fn test_cross_unit_consistency_extended()
{
  // Take 10 samples to ensure consistency
  for i in 0..10
  {
    let ms = the_module ::now();
    let s = the_module ::s ::now();
    let ns = the_module ::ns ::now();

    // ms/1000 should equal s (with tolerance for rounding)
    let diff_s = ( ms / 1000 - s ).abs();
    assert!( diff_s <= 1, "Sample {i}: ms and s inconsistent (diff: {diff_s} s)" );

    // ns/1_000_000 should equal ms (with tolerance for execution time)
    let diff_ms = ( ns / 1_000_000 - ms ).abs();
    assert!( diff_ms <= 1, "Sample {i}: ns and ms inconsistent (diff: {diff_ms} ms)" );

    // Small delay to ensure time progresses
    std ::thread ::sleep( core ::time ::Duration ::from_micros( 100 ) );
  }
}

/// Tests that time is monotonic within same process.
///
/// System time should progress forward (or stay same if called
/// very quickly). This test uses sleep to ensure time advances.
///
/// # Note
///
/// This tests monotonicity under normal conditions. System time
/// can jump backwards due to NTP adjustments or manual clock changes,
/// but this is rare and not tested here.
///
/// # Test Coverage
///
/// - Verifies time advances after sleep
/// - Verifies time never goes backwards during test
/// - Documents expected behavior under normal system conditions
#[ test ]
#[ cfg( feature = "time_now" ) ]
#[ cfg( not( feature = "no_std" ) ) ]
fn test_monotonic_property()
{
  let t1 = the_module ::now();

  // Sleep for 10ms to ensure time advances
  std ::thread ::sleep( core ::time ::Duration ::from_millis( 10 ) );

  let t2 = the_module ::now();

  assert!( t2 >= t1, "Time should not go backwards (t1={t1}, t2={t2})" );
  let delta = t2 - t1;
  assert!( delta >= 10, "Time should advance at least 10ms (delta={delta} ms)" );

  // Verify all units are monotonic
  let s1 = the_module ::s ::now();
  std ::thread ::sleep( core ::time ::Duration ::from_millis( 1000 ) );
  let s2 = the_module ::s ::now();

  assert!( s2 >= s1, "Time in seconds should not go backwards" );
  assert!( s2 - s1 >= 1, "Time should advance at least 1 second" );
}

/// Tests no_std compilation compatibility.
///
/// This is a compile-time check. If this test compiles with
/// no_std feature enabled, it verifies the crate can build
/// in no_std environments.
///
/// # Note
///
/// Time functions are disabled in no_std mode (require std::time),
/// so this just verifies the crate compiles, not functionality.
#[ test ]
#[ cfg( all( feature = "no_std", not( feature = "time_now" ) ) ) ]
fn test_no_std_compilation()
{
  // If this compiles, no_std works
  // No functionality to test since time_now is disabled
}
