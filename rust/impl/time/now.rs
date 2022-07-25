
use chrono::prelude::*;

///
/// Get current time. Units are milliseconds.
///

pub fn now() -> i64
{
  Utc::now().timestamp_millis()
}

///
/// Default units are seconds.
///

pub mod s
{
  use chrono::prelude::*;

  ///
  /// Get current time. Units are seconds.
  ///

  pub fn now() -> i64
  {
    Utc::now().timestamp()
  }

}

///
/// Default units are milliseconds.
///

pub mod ms
{
  use chrono::prelude::*;

  ///
  /// Get current time. Units are milliseconds.
  ///

  pub fn now() -> i64
  {
    Utc::now().timestamp_millis()
  }

}

// xxx : qqq for Dima : problem. ms should not be part of `wtools::ms`, something is wrong. fix it, please

///
/// Default units are nanoseconds.
///

pub mod ns
{
  use chrono::prelude::*;

  ///
  /// Get current time. Units are nanoseconds.
  ///

  pub fn now() -> i64
  {
    Utc::now().timestamp_nanos()
  }

}
