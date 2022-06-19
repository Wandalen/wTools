/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  #[ derive( Debug ) ]
  pub struct Rect
  {
    pub left_top : X2< i32 >,
    pub right_bottom : X2< i32 >,
  }

  impl Rect
  {

    pub fn new( left_top : X2< i32 >, right_bottom : X2< i32 > ) -> Self
    {
      Self{ left_top, right_bottom }
    }

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
    private::Rect,
  };
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::
  {
    prelude::*,
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
