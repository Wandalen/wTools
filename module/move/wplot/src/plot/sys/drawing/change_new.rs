/// Internal namespace.
pub( crate ) mod private
{
  // use crate::protected::*;
  use crate::abs::{change::private::ChangeInterface, identity::private::Id};

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

::meta_tools::mod_interface!
{
  exposed use DrawingChangeNew;
}
