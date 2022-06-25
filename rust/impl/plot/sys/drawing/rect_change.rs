/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// Command to draw rectangle.
  #[ allow( dead_code ) ]
  #[ derive( Debug, Clone ) ]
  pub struct RectChange
  {
    /// Id.
    pub( crate ) id : Id,
    /// Left-top corner.
    pub( crate ) left_top : X2< f32 >,
    /// Right-bottom corner.
    pub( crate )  right_bottom : X2< f32 >,
  }

  impl RectChange
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

  impl ChangeInterface for RectChange
  {
  }

}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::
  {
    exposed::*,
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::
  {
    prelude::*,
    private::RectChange,
  };
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::
  {
  };
}
