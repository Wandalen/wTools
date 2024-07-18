/// Internal namespace.
pub( crate ) mod private
{
  // use crate::own::*;
  use crate::abs::{identity::private::Id, change::private::ChangeInterface};

  /// Command to draw rectangle.
  #[ allow( dead_code ) ]
  #[ derive( Debug, Clone ) ]
  pub struct RectChangeNew
  {
    /// Id.
    pub( crate ) id : Id,
  }

  impl RectChangeNew
  {

    /// Constructor
    pub fn new( id : Id ) -> Self
    {
      Self{ id }
    }

  }

  impl ChangeInterface for RectChangeNew
  {
  }

}

::meta_tools::mod_interface!
{
  exposed use RectChangeNew;
}
