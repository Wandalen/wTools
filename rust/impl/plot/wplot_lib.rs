#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wplot/latest/wplot/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trace_macros ) ]

//!
//! Plot interface.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

// pub use ::wmath as math;
// use ::wtools::prelude::*;
use ::wtools::mod_interface;

/// Namespace with dependencies.
pub mod dependency
{
  pub use ::image;
  pub use ::open;
  pub use ::wmath;
  pub use ::rgb;
}

crate::mod_interface!
{

  /// Describe colors.
  #[ cfg( feature = "use_std" ) ]
  layer color;
  /// Abstraction.
  #[ cfg( feature = "use_std" ) ]
  layer abs;
  /// Concrete system.
  #[ cfg( feature = "use_std" ) ]
  layer sys;

  use super::math;
  protected use ::wmath as math;
  protected( crate ) use ::wtools::prelude::*;

}
