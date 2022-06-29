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

pub use wmath as math;
pub use wtools::prelude::*;

/// Describe colors.
pub mod color;
/// Abstraction.
#[ cfg( feature = "use_std" ) ]
pub mod abs;
/// Concrete system.
#[ cfg( feature = "use_std" ) ]
pub mod sys;

/// Namespace with dependencies.
pub mod dependency
{
  pub use ::image;
  pub use ::open;
  pub use ::wmath;
  pub use ::rgb;
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::
  {
    sys::orphan::*,
    abs::orphan::*,
    color::orphan::*,
  };
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::
  {
    sys::exposed::*,
    abs::exposed::*,
    color::exposed::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "use_std" ) ]
  pub use super::
  {
    sys::prelude::*,
    abs::prelude::*,
    color::prelude::*,
  };
  pub use ::wmath::prelude::*;
}
