/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;

  /// ChangerInterface of brush stroke.
  #[ allow( dead_code ) ]
  #[ derive( Debug, Clone ) ]
  pub struct StrokeBrushChangeWidth
  {
    pub( crate ) id : Id,
    pub( crate ) val : f32,
  }

  impl StrokeBrushChangeWidth
  {
    /// Constructor.
    pub fn new( id : Id, val : f32 ) -> Self
    {
      Self { id, val }
    }
  }

  impl ChangeInterface for StrokeBrushChangeWidth
  {
  }

}

crate::mod_interface!
{
  exposed use StrokeBrushChangeWidth;
}
