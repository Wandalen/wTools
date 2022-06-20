/// Internal namespace.
pub( crate ) mod private
{
  // use crate::*;
  use crate::draw::*;

  /// Queue of draw commands.
  #[ derive( Debug ) ]
  pub struct Queue
  {
    /// Container to store commands.
    pub container : Vec< Box< dyn DrawCommandInterface > >,
  }

  impl Queue
  {
    /// Constructor.
    pub fn new() -> Self
    {
      let container = Vec::new();
      Self { container }
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
    private::Queue,
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
