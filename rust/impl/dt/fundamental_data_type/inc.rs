
/// Type constructor of pair.
pub mod pair;
/// Type constructor of single.
pub mod single;
/// Type constructors.
pub mod types;
/// Generic traits.
pub mod traits;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  pub use super::pair::orphan::*;
  pub use super::single::orphan::*;
  pub use super::types::orphan::*;
  pub use super::traits::orphan::*;
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
  pub use super::pair::exposed::*;
  pub use super::single::exposed::*;
  pub use super::types::exposed::*;
  pub use super::traits::exposed::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::pair::prelude::*;
  pub use super::single::prelude::*;
  pub use super::types::prelude::*;
  pub use super::traits::prelude::*;
}
