/// Define a private namespace for all its items.
mod private
{
  // use crate::own::*;

use crate::abs::{change::private::ChangeInterface, identity::private::Id};

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

::meta_tools::mod_interface!
{
  exposed use StrokeBrushChangeWidth;
}
