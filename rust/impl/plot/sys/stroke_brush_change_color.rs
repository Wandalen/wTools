/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// ChangerInterface of brush stroke.
  #[ allow( dead_code ) ]
  #[ derive( Debug, Clone ) ]
  pub struct StrokeBrushChangeColor
  {
    pub( crate ) color : Rgba< f32 >,
  }

  impl StrokeBrushChangeColor
  {
    /// Constructor.
    pub fn new< Color >( color : Color ) -> Self
    where
      Color : RgbaInterface< f32 >,
    {
      Self{ color : color.into_rgba() }
    }
  }

  impl ChangeInterface for StrokeBrushChangeColor
  {
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::
  {
    orphan::*,
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
    private::StrokeBrushChangeColor,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
  };
}
