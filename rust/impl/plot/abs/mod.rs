
// pub use wmath as math;
// pub use wtools::prelude::*;

/// Describe change.
pub mod change;
/// Describe changer.
pub mod changer;
/// Describe system.
pub mod context;

/// Identity of resource.
pub mod identity;
/// Registry.
pub mod registry;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::
  {
    orphan::*,
    change::orphan::*,
    changer::orphan::*,
    context::orphan::*,
    identity::orphan::*,
    registry::orphan::*,
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
    change::exposed::*,
    changer::exposed::*,
    context::exposed::*,
    identity::exposed::*,
    registry::exposed::*,
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::
  {
    change::prelude::*,
    changer::prelude::*,
    context::prelude::*,
    identity::prelude::*,
    registry::prelude::*,
  };
  // pub use ::wmath::prelude::*;
}
