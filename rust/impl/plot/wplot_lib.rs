#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wplot/latest/wplot/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Collection of general purpose tools to iterate. Currently it simply reexport itertools.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

pub use wmath as math;
pub use wtools::prelude::*;

/// Describe colors.
pub mod color;
/// Main aggregating object.
pub mod context;
/// Draw commands.
pub mod draw;
/// Brush.
pub mod brush;
/// Target to draw.
pub mod target;

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
  pub use super::
  {
    orphan::*,
    color::orphan::*,
    context::orphan::*,
    draw::orphan::*,
    brush::orphan::*,
    target::orphan::*,
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
  pub use super::
  {
    prelude::*,
    color::exposed::*,
    context::exposed::*,
    draw::exposed::*,
    brush::exposed::*,
    target::exposed::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::
  {
    color::prelude::*,
    context::prelude::*,
    draw::prelude::*,
    brush::prelude::*,
    target::prelude::*,
  };
  pub use ::wmath::prelude::*;
}
