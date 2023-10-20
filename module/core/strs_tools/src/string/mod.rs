
/// Add indentation to each line.
#[ cfg( all( feature = "indentation", feature = "use_std" ) ) ]
pub mod indentation;
/// Isolate parts of string.
#[ cfg( all( feature = "isolate", feature = "use_std" ) ) ]
pub mod isolate;
/// Parsing of numbers.
#[ cfg( all( feature = "parse_number", feature = "use_std" ) ) ]
pub mod number;
/// Parse string.
#[ cfg( all( feature = "parse_request", feature = "use_std" ) ) ]
pub mod parse_request;
/// Spit string with a delimeter.
#[ cfg( all( feature = "split", feature = "use_std" ) ) ]
pub mod split;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  #[ cfg( all( feature = "indentation", feature = "use_std" ) ) ]
  pub use super::indentation::orphan::*;
  #[ cfg( all( feature = "isolate", feature = "use_std" ) ) ]
  pub use super::isolate::orphan::*;
  #[ cfg( all( feature = "parse_number", feature = "use_std" ) ) ]
  pub use super::number::orphan::*;
  #[ cfg( all( feature = "parse_request", feature = "use_std" ) ) ]
  pub use super::parse_request::orphan::*;
  #[ cfg( all( feature = "split", feature = "use_std" ) ) ]
  pub use super::split::orphan::*;
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
  #[ cfg( all( feature = "indentation", feature = "use_std" ) ) ]
  pub use super::indentation::exposed::*;
  #[ cfg( all( feature = "isolate", feature = "use_std" ) ) ]
  pub use super::isolate::exposed::*;
  #[ cfg( all( feature = "parse_number", feature = "use_std" ) ) ]
  pub use super::number::exposed::*;
  #[ cfg( all( feature = "parse_request", feature = "use_std" ) ) ]
  pub use super::parse_request::exposed::*;
  #[ cfg( all( feature = "split", feature = "use_std" ) ) ]
  pub use super::split::exposed::*;
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  #[ cfg( all( feature = "indentation", feature = "use_std" ) ) ]
  pub use super::indentation::prelude::*;
  #[ cfg( all( feature = "isolate", feature = "use_std" ) ) ]
  pub use super::isolate::prelude::*;
  #[ cfg( all( feature = "parse_number", feature = "use_std" ) ) ]
  pub use super::number::prelude::*;
  #[ cfg( all( feature = "parse_request", feature = "use_std" ) ) ]
  pub use super::parse_request::prelude::*;
  #[ cfg( all( feature = "split", feature = "use_std" ) ) ]
  pub use super::split::prelude::*;
}
