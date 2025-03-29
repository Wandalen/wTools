/// Define a private namespace for all its items.
mod private
{
  use crate::own::*;

  /// ChangerInterface of brush stroke.
  #[ allow( dead_code ) ]
  #[ derive( Debug, Clone ) ]
  pub struct StrokeBrushChangeColor
  {
    pub( crate ) id : Id,
    pub( crate ) val : Rgba< f32 >,
  }

  impl StrokeBrushChangeColor
  {
    /// Constructor.
    pub fn new< Color >( id : Id, val : Color ) -> Self
    where
      Color : RgbaInterface< f32 >,
    {
      Self{ id, val : val.into_rgba() }
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
