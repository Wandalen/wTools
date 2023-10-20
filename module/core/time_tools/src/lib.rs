#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/time_tools/latest/time_tools/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

//!
//! Collection of time tools.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

///
/// Collection of general purpose time tools.
///
// /// ### Basic use-case.
// /// ```
// /// use time_tools::*;
// ///
// /// fn main()
// /// {
// ///   /* get milliseconds from UNIX epoch */
// ///   let now = time::now();
// ///   let now_chrono = chrono::prelude::Utc::now().timestamp_millis();
// ///   assert_eq!( now, now_chrono );
// ///
// ///   /* get nanoseconds from UNIX epoch */
// ///   let now = time::now();
// ///   let now_ns = time::ns::now();
// ///   assert_eq!( now, now_ns / 1000000 );
// ///
// ///   /* get seconds from UNIX epoch */
// ///   let now = time::now();
// ///   let now_s = time::s::now();
// ///   assert_eq!( now / 1000, now_s );
// /// }
// /// ```

// pub mod time
// {
//   include!( "./now.rs" );
// }
//
// pub use time::*;

/// Collection of time tools.
// pub mod time;

/// Dependencies.
pub mod dependency
{
}

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Shared with parent namespace of the module
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
  // #[ doc( inline ) ]
  // pub use super::time::time::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // #[ doc( inline ) ]
  // pub use super::time::time::prelude::*;
}

use std::time;

///
/// Get current time. Units are milliseconds.
///

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
  pub fn now() -> i64
  {
    time::SystemTime::now()
    .duration_since( time::UNIX_EPOCH ).unwrap()
    .as_nanos() as i64
  }
}