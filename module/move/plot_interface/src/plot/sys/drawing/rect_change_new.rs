/// Internal namespace.
mod private
{
  use crate::own::*;

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

crate::mod_interface!
{
  exposed use RectChangeNew;
}
