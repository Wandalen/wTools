/// Internal namespace.
pub( crate ) mod private
{
  // use crate::prelude::*;



}

/// Draw command.
mod command;
/// Draw queue.
mod queue;
/// Draw rect.
mod rect;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::
  {
    orphan::*,
    command::orphan::*,
    queue::orphan::*,
    rect::orphan::*,
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
    command::exposed::*,
    queue::exposed::*,
    rect::exposed::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::
  {
    command::prelude::*,
    queue::prelude::*,
    rect::prelude::*,
  };
}
