#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/time_tools/latest/time_tools/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Collection of time tools.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

///
/// Collection of general purpose time tools.
///
// /// ### Sample
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
pub mod time;

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
  #[ doc( inline ) ]
  pub use super::time::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::time::prelude::*;
}
