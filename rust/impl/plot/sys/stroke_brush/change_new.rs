/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;

  /// ChangerInterface of brush stroke.
  #[ allow( dead_code ) ]
  #[ derive( Debug, Clone ) ]
  pub struct StrokeBrushChangeNew
  {
    pub( crate ) id : Id,
  }

  impl StrokeBrushChangeNew
  {
    /// Constructor.
    pub fn new( id : Id ) -> Self
    {
      Self{ id }
    }
  }

  impl ChangeInterface for StrokeBrushChangeNew
  {
  }

}

crate::mod_interface!
{
  exposed use StrokeBrushChangeNew;
}
