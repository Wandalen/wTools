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

/// Describe change.
pub mod change;
/// Describe changer.
pub mod changer;
/// Describe colors.
pub mod color;
/// Main aggregating object.
pub mod context;
/// Context changer.
pub mod context_changer;
/// Draw commands.
pub mod draw;
/// Identity of resource.
pub mod identity;
/// Registry.
pub mod registry;
/// Brush stroke.
pub mod stroke_brush;
/// Changer of brush stroke.
pub mod stroke_brush_changer;
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
    change::orphan::*,
    changer::orphan::*,
    color::orphan::*,
    context::orphan::*,
    context_changer::orphan::*,
    draw::orphan::*,
    identity::orphan::*,
    registry::orphan::*,
    stroke_brush::orphan::*,
    stroke_brush_changer::orphan::*,
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
    change::exposed::*,
    changer::exposed::*,
    color::exposed::*,
    context::exposed::*,
    context_changer::exposed::*,
    draw::exposed::*,
    identity::exposed::*,
    registry::orphan::*,
    stroke_brush::exposed::*,
    stroke_brush_changer::exposed::*,
    target::exposed::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::
  {
    change::prelude::*,
    changer::prelude::*,
    color::prelude::*,
    context::prelude::*,
    context_changer::prelude::*,
    draw::prelude::*,
    identity::prelude::*,
    registry::orphan::*,
    stroke_brush::prelude::*,
    stroke_brush_changer::prelude::*,
    target::prelude::*,
  };
  pub use ::wmath::prelude::*;
}
