
/// Add indentation to each line.
#[ cfg( feature = "indentation" ) ]
pub mod indentation;
/// Spit string with a delimeter.
#[ cfg( feature = "split" ) ]
pub mod split;
/// Parse string.
#[ cfg( feature = "parse" ) ]
pub mod parse;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  #[ cfg( feature = "indentation" ) ]
  pub use super::indentation::orphan::*;
  #[ cfg( feature = "split" ) ]
  pub use super::split::orphan::*;
  #[ cfg( feature = "parse" ) ]
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
  #[ cfg( feature = "indentation" ) ]
  pub use super::indentation::exposed::*;
  #[ cfg( feature = "split" ) ]
  pub use super::split::exposed::*;
  #[ cfg( feature = "parse" ) ]
  pub use super::parse::exposed::*;
}

pub use exposed::*;

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  #[ cfg( feature = "indentation" ) ]
  pub use super::indentation::prelude::*;
  #[ cfg( feature = "split" ) ]
  pub use super::split::prelude::*;
  #[ cfg( feature = "parse" ) ]
  pub use super::parse::prelude::*;
}
