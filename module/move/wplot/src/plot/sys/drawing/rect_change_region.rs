/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;

  /// Command to draw rectangle.
  #[ allow( dead_code ) ]
  #[ derive( Debug, Clone ) ]
  pub struct RectChangeRegion
  {
    /// Id.
    pub( crate ) id : Id,
    /// Left-top corner.
    pub( crate ) left_top : X2< f32 >,
    /// Right-bottom corner.
    pub( crate )  right_bottom : X2< f32 >,
  }

  impl RectChangeRegion
  {

    /// Constructor
    pub fn new( id : Id ) -> Self
    {
      let left_top = X2::make( -1.0, -1.0 );
      let right_bottom = X2::make( 1.0, 1.0 );
      Self{ left_top, right_bottom, id }
    }

    /// Constructor
    pub fn region( mut self, left_top : X2< f32 >, right_bottom : X2< f32 > ) -> Self
    {
      self.left_top = left_top;
      self.right_bottom = right_bottom;
      self
    }

  }

  impl ChangeInterface for RectChangeRegion
  {
  }

}

crate::mod_interface!
{
  exposed use RectChangeRegion;
}
