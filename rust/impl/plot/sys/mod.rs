
// pub use wmath as math;
// pub use wtools::prelude::*;

// /// Describe change.
// pub mod change;
// /// Describe changer.
// pub mod changer;
// /// Describe colors.
// pub mod color;
/// Main aggregating object.
pub mod context;
/// Context changer.
pub mod context_changer;

/// Draw changer.
pub mod drawing;
/// Draw changer.
pub mod drawing_changer;
/// ChangeInterface for drawing constructor.
pub mod drawing_change_new;

// /// Identity of resource.
// pub mod identity;
// /// Registry.
// pub mod registry;
/// Brush stroke.
pub mod stroke_brush;
/// ChangerInterface of brush stroke.
pub mod stroke_brush_changer;
/// ChangeInterface of brush stroke constructor.
pub mod stroke_brush_change_new;
/// ChangeInterface of brush stroke color.
pub mod stroke_brush_change_color;
/// Target to draw.
pub mod target;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::
  {
    orphan::*,
    context::orphan::*,
    context_changer::orphan::*,
    drawing::orphan::*,
    drawing_changer::orphan::*,
    drawing_change_new::orphan::*,
    stroke_brush::orphan::*,
    stroke_brush_changer::orphan::*,
    stroke_brush_change_new::orphan::*,
    stroke_brush_change_color::orphan::*,
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
    context::exposed::*,
    context_changer::exposed::*,
    drawing::exposed::*,
    drawing_changer::exposed::*,
    drawing_change_new::exposed::*,
    stroke_brush::exposed::*,
    stroke_brush_changer::exposed::*,
    stroke_brush_change_new::exposed::*,
    stroke_brush_change_color::exposed::*,
    target::exposed::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::
  {
    context::prelude::*,
    context_changer::prelude::*,
    drawing::prelude::*,
    drawing_changer::prelude::*,
    drawing_change_new::prelude::*,
    stroke_brush::prelude::*,
    stroke_brush_changer::prelude::*,
    stroke_brush_change_new::prelude::*,
    stroke_brush_change_color::prelude::*,
    target::prelude::*,
  };
  // pub use ::wmath::prelude::*;
}
