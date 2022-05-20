
/// Collection of primal data types.
pub mod single;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  pub use super::single::orphan::*;
}

pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  pub use super::single::exposed::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::single::prelude::*;
}
