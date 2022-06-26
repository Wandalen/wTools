/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

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

/// Protected namespace of the module.
pub mod protected
{
  pub use super::
  {
    orphan::*,
  };
}

pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::
  {
    prelude::*,
    private::DrawingChangeNew,
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
