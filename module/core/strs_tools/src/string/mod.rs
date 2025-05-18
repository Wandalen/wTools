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
  // Removed: #[allow(unused_imports)] use super::*;

  pub use super::orphan::*; // Corrected
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  // pub use self::indentation; // Removed
  // #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ] // Redundant cfg
  pub use super::indentation::orphan::*; // Corrected
  #[ cfg( all( feature = "string_isolate", not( feature = "no_std" ) ) ) ]
  // pub use self::isolate; // Removed
  // #[ cfg( all( feature = "string_isolate", not( feature = "no_std" ) ) ) ] // Redundant cfg
  pub use super::isolate::orphan::*; // Corrected
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  // pub use self::number; // Removed
  // #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ] // Redundant cfg
  #[ allow( unused_imports ) ]
  pub use super::number::orphan::*; // Corrected
  #[ cfg( all( feature = "string_parse_request", not( feature = "no_std" ) ) ) ]
  // pub use self::parse_request; // Removed
  // #[ cfg( all( feature = "string_parse_request", not( feature = "no_std" ) ) ) ] // Redundant cfg
  pub use super::parse_request::orphan::*; // Corrected
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  // pub use self::split; // Removed
  // #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ] // Redundant cfg
  pub use super::split::orphan::*; // Corrected
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[allow(unused_imports)] use super::*;
  pub use super::exposed::*; // Corrected
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  // Removed: #[allow(unused_imports)] use super::*;
  pub use super::prelude::*; // Corrected
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use super::indentation::exposed::*; // Corrected
  #[ cfg( all( feature = "string_isolate", not( feature = "no_std" ) ) ) ]
  pub use super::isolate::exposed::*; // Corrected
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use super::number::exposed::*; // Corrected
  #[ cfg( all( feature = "string_parse_request", not( feature = "no_std" ) ) ) ]
  pub use super::parse_request::exposed::*; // Corrected
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use super::split::exposed::*; // Corrected
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[allow(unused_imports)] use super::*;
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use super::indentation::prelude::*; // Corrected
  #[ cfg( all( feature = "string_isolate", not( feature = "no_std" ) ) ) ]
  pub use super::isolate::prelude::*; // Corrected
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use super::number::prelude::*; // Corrected
  #[ cfg( all( feature = "string_parse_request", not( feature = "no_std" ) ) ) ]
  pub use super::parse_request::prelude::*; // Corrected
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use super::split::prelude::*; // Corrected
}
