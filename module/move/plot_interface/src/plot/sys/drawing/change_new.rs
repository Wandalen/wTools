/// Define a private namespace for all its items.
mod private
{
  use crate::own::*;

  /// ChangerInterface of brush stroke.
  #[ allow( dead_code ) ]
  #[ derive( Debug, Clone ) ]
  pub struct DrawingChangeNew
  {
    id : Id,
  }

  impl DrawingChangeNew
  {
    /// Constructor.
    pub fn new( id : Id ) -> Self
    {
      Self{ id }
    }
  }

  impl ChangeInterface for DrawingChangeNew
  {
  }

}

crate::mod_interface!
{
  exposed use DrawingChangeNew;
}
