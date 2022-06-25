#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wplot/latest/wplot/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

#![ feature( trace_macros ) ]

//!
//! Collection of general purpose tools to iterate. Currently it simply reexport itertools.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

pub use wmath as math;
pub use wtools::prelude::*;

/// Describe colors.
pub mod color;
/// Abstraction.
pub mod abs;
/// Concrete system.
pub mod sys;

// /// Describe change.
// pub mod change;
// /// Describe changer.
// pub mod changer;
// /// Describe colors.
// pub mod color;
// /// Main aggregating object.
// pub mod context;
// /// Context changer.
// pub mod context_changer;
//
// /// Draw changer.
// pub mod drawing;
// /// Draw changer.
// pub mod drawing_changer;
// /// ChangeInterface for drawing constructor.
// pub mod drawing_change_new;
//
// /// Identity of resource.
// pub mod identity;
// /// Registry.
// pub mod registry;
// /// Brush stroke.
// pub mod stroke_brush;
// /// ChangerInterface of brush stroke.
// pub mod stroke_brush_changer;
// /// ChangeInterface of brush stroke constructor.
// pub mod stroke_brush_change_new;
// /// ChangeInterface of brush stroke color.
// pub mod stroke_brush_change_color;
// /// Target to draw.
// pub mod target;

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
  pub use super::
  {
    prelude::*,
    sys::exposed::*,
    abs::exposed::*,
    color::exposed::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::
  {
    sys::prelude::*,
    abs::prelude::*,
    color::prelude::*,
  };
  pub use ::wmath::prelude::*;
}
