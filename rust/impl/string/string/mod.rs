
/// Add indentation to each line.
pub mod indentation;
/// Spit string with a delimeter.
pub mod split;
/// Parse string.
pub mod parse;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  pub use super::indentation::orphan::*;
  pub use super::split::orphan::*;
  pub use super::parse::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Parented namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::indentation::exposed::*;
  pub use super::split::exposed::*;
  pub use super::parse::exposed::*;
}

pub use exposed::*;

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  pub use super::indentation::prelude::*;
  pub use super::split::prelude::*;
  pub use super::parse::prelude::*;
}
