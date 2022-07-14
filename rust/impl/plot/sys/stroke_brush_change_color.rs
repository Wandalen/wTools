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

crate::mod_interface!
{
  exposed use StrokeBrushChangeColor;
}
