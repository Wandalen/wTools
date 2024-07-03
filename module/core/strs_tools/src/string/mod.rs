
/// Add indentation to each line.
#[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
pub mod indentation;
/// Isolate parts of string.
#[ cfg( all( feature = "string_isolate", not( feature = "no_std" ) ) ) ]
pub mod isolate;
/// Parsing of numbers.
#[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
pub mod number;
/// Parse string.
#[ cfg( all( feature = "string_parse_request", not( feature = "no_std" ) ) ) ]
pub mod parse_request;
/// Spit string with a delimeter.
#[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
pub mod split;

// /// Set of modules.
// pub( crate ) mod modules
// {
//   pub use super::indentation;
//   pub use super::isolate;
//   pub use super::number;
//   pub use super::parse_request;
//   pub use super::split;
// }

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  pub use orphan::*;
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  pub use super::indentation::orphan::*;
  #[ cfg( all( feature = "string_isolate", not( feature = "no_std" ) ) ) ]
  pub use super::isolate::orphan::*;
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use super::number::orphan::*;
  #[ cfg( all( feature = "string_parse_request", not( feature = "no_std" ) ) ) ]
  pub use super::parse_request::orphan::*;
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use super::split::orphan::*;
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use super::indentation::exposed::*;
  #[ cfg( all( feature = "string_isolate", not( feature = "no_std" ) ) ) ]
  pub use super::isolate::exposed::*;
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use super::number::exposed::*;
  #[ cfg( all( feature = "string_parse_request", not( feature = "no_std" ) ) ) ]
  pub use super::parse_request::exposed::*;
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use super::split::exposed::*;
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use super::indentation::prelude::*;
  #[ cfg( all( feature = "string_isolate", not( feature = "no_std" ) ) ) ]
  pub use super::isolate::prelude::*;
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use super::number::prelude::*;
  #[ cfg( all( feature = "string_parse_request", not( feature = "no_std" ) ) ) ]
  pub use super::parse_request::prelude::*;
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use super::split::prelude::*;
}
