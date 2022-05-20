
/// A type wrapping a another type into a tuple.
pub mod single;
/// A type wrapping two types into a tuple.
pub mod pair;
// /// A type vector into a tuple.
// pub mod many;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  pub use super::single::orphan::*;
  pub use super::pair::orphan::*;
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
  pub use super::pair::exposed::*;
}

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::single::prelude::*;
  pub use super::pair::prelude::*;
}
