#[ cfg(not(feature = "no_std")) ]
use std ::time;

///
/// Get current time. Units are milliseconds.
///
/// Returns the number of milliseconds elapsed since the UNIX epoch
/// (1970-01-01 00:00:00 UTC).
///
/// # Panics
///
/// Panics if the system clock is set to a time before the UNIX epoch
/// (1970-01-01 00:00:00 UTC). This indicates a misconfigured system clock
/// and is extremely rare in practice.
///
/// If your application must handle such systems, consider using
/// `std::time::SystemTime::now()` directly and handling the error.
///
#[ cfg(not(feature = "no_std")) ]
#[ allow( clippy ::cast_possible_truncation ) ]
#[ must_use ] pub fn now() -> i64 {
  time ::SystemTime ::now()
    .duration_since(time ::UNIX_EPOCH)
    .expect("System clock is before UNIX epoch (1970-01-01). Please check your system time configuration.")
    .as_millis() as i64
}

///
/// Default units are seconds.
///
pub mod s 
{
  #[ allow( unused_imports ) ]
  use super :: *;

  ///
  /// Get current time. Units are seconds.
  ///
  /// Returns the number of seconds elapsed since the UNIX epoch
  /// (1970-01-01 00:00:00 UTC).
  ///
  /// # Panics
  ///
  /// Panics if the system clock is set to a time before the UNIX epoch.
  /// This indicates a misconfigured system clock.
  ///
  #[ cfg(not(feature = "no_std")) ]
  #[ allow( clippy ::cast_possible_wrap ) ]
  #[ must_use ] pub fn now() -> i64 {
    time ::SystemTime ::now()
      .duration_since(time ::UNIX_EPOCH)
      .expect("System clock is before UNIX epoch (1970-01-01). Please check your system time configuration.")
      .as_secs() as i64
  }
}

///
/// Default units are milliseconds.
///
pub mod ms 
{
  #[ allow( unused_imports ) ]
  use super :: *;

  ///
  /// Get current time. Units are milliseconds.
  ///
  /// Returns the number of milliseconds elapsed since the UNIX epoch
  /// (1970-01-01 00:00:00 UTC).
  ///
  /// # Panics
  ///
  /// Panics if the system clock is set to a time before the UNIX epoch.
  /// This indicates a misconfigured system clock.
  ///
  #[ cfg(not(feature = "no_std")) ]
  #[ allow( clippy ::cast_possible_truncation ) ]
  #[ must_use ] pub fn now() -> i64 {
    time ::SystemTime ::now()
      .duration_since(time ::UNIX_EPOCH)
      .expect("System clock is before UNIX epoch (1970-01-01). Please check your system time configuration.")
      .as_millis() as i64
  }
}

// xxx: qqq for Dima: problem. ms should not be part of `wtools ::ms`, something is wrong. fix it, please
/* aaa: Dmytro: all routines and modules is inside wtools and wtools ::time, added test suite to test it */

///
/// Default units are nanoseconds.
///
pub mod ns 
{
  #[ allow( unused_imports ) ]
  use super :: *;

  ///
  /// Get current time. Units are nanoseconds.
  ///
  /// Returns the number of nanoseconds elapsed since the UNIX epoch
  /// (1970-01-01 00:00:00 UTC).
  ///
  /// # Panics
  ///
  /// Panics if the system clock is set to a time before the UNIX epoch.
  /// This indicates a misconfigured system clock.
  ///
  /// # Overflow Warning
  ///
  /// On 64-bit systems, i64 can represent ~292 years of nanoseconds.
  /// Overflow will occur around year 2262. Behavior after overflow
  /// is wrapping (incorrect time values).
  ///
  #[ cfg(not(feature = "no_std")) ]
  #[ allow( clippy ::cast_possible_truncation ) ]
  #[ must_use ] pub fn now() -> i64 {
    time ::SystemTime ::now()
      .duration_since(time ::UNIX_EPOCH)
      .expect("System clock is before UNIX epoch (1970-01-01). Please check your system time configuration.")
      .as_nanos() as i64
  }
}
