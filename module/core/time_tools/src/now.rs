#[ cfg( not( feature = "no_std" ) ) ]
use std::time;

///
/// Get current time. Units are milliseconds.
///
#[ cfg( not( feature = "no_std" ) ) ]
pub fn now() -> i64
{
  time::SystemTime::now()
  .duration_since( time::UNIX_EPOCH ).unwrap()
  .as_millis() as i64
}

///
/// Default units are seconds.
///

pub mod s
{
  use super::*;
  
  /// Get current time. Units are seconds.
  #[ cfg( not( feature = "no_std" ) ) ]
  pub fn now() -> i64
  {
    time::SystemTime::now()
    .duration_since( time::UNIX_EPOCH ).unwrap()
    .as_secs() as i64
  }
}

///
/// Default units are milliseconds.
///

pub mod ms
{
  use super::*;

  /// Get current time. Units are milliseconds.
  #[ cfg( not( feature = "no_std" ) ) ]
  pub fn now() -> i64
  {
    time::SystemTime::now()
    .duration_since( time::UNIX_EPOCH ).unwrap()
    .as_millis() as i64
  }
}

// xxx : qqq for Dima : problem. ms should not be part of `wtools::ms`, something is wrong. fix it, please
/* aaa : Dmytro : all routines and modules is inside wtools and wtools::time, added test suite to test it */

///
/// Default units are nanoseconds.
///

pub mod ns
{
  use super::*;

  /// Get current time. Units are nanoseconds.
  #[ cfg( not( feature = "no_std" ) ) ]
  pub fn now() -> i64
  {
    time::SystemTime::now()
    .duration_since( time::UNIX_EPOCH ).unwrap()
    .as_nanos() as i64
  }
}
