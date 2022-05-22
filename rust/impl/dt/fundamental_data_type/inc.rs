
/// Type constructors.
pub mod types;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  pub use super::types::orphan::*;
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
  pub use super::types::exposed::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::types::prelude::*;
}
