
/// Spit string with a delimeter.
pub mod split;

/// Parse string.
pub mod parse;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::split::exposed::*;
  pub use super::parse::exposed::*;
}

pub use exposed::*;

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  pub use super::split::prelude::*;
  pub use super::parse::prelude::*;
}
