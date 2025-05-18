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
  #[allow(unused_imports)] use super::*;

  pub use orphan::*;
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  pub use self::indentation; // Corrected
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  pub use self::indentation::orphan::*; // Corrected
  #[ cfg( all( feature = "string_isolate", not( feature = "no_std" ) ) ) ]
  pub use self::isolate; // Corrected
  #[ cfg( all( feature = "string_isolate", not( feature = "no_std" ) ) ) ]
  pub use self::isolate::orphan::*; // Corrected
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  pub use self::number; // Corrected
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use self::number::orphan::*; // Corrected
  #[ cfg( all( feature = "string_parse_request", not( feature = "no_std" ) ) ) ]
  pub use self::parse_request; // Corrected
  #[ cfg( all( feature = "string_parse_request", not( feature = "no_std" ) ) ) ]
  pub use self::parse_request::orphan::*; // Corrected
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use self::split; // Corrected
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use self::split::orphan::*; // Corrected
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[allow(unused_imports)] use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[allow(unused_imports)] use super::*;
  pub use prelude::*;
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use self::indentation::exposed::*; // Corrected
  #[ cfg( all( feature = "string_isolate", not( feature = "no_std" ) ) ) ]
  pub use self::isolate::exposed::*; // Corrected
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use self::number::exposed::*; // Corrected
  #[ cfg( all( feature = "string_parse_request", not( feature = "no_std" ) ) ) ]
  pub use self::parse_request::exposed::*; // Corrected
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use self::split::exposed::*; // Corrected
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[allow(unused_imports)] use super::*;
  #[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use self::indentation::prelude::*; // Corrected
  #[ cfg( all( feature = "string_isolate", not( feature = "no_std" ) ) ) ]
  pub use self::isolate::prelude::*; // Corrected
  #[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
  #[ allow( unused_imports ) ]
  pub use self::number::prelude::*; // Corrected
  #[ cfg( all( feature = "string_parse_request", not( feature = "no_std" ) ) ) ]
  pub use self::parse_request::prelude::*; // Corrected
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use self::split::prelude::*; // Corrected
}
