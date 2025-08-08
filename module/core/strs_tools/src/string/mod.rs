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
/// Split string with a delimiter.
#[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
pub mod split;
/// Zero-copy string operations.
#[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
pub mod zero_copy;
/// Parser integration for single-pass processing.
#[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
pub mod parser;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own {
  #[ allow( unused_imports ) ]
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
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use super::zero_copy::{ ZeroCopyStringExt, ZeroCopySplit, ZeroCopySegment, zero_copy_split };
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use super::parser::{ ParserIntegrationExt, CommandParser, ParsedToken, ParseError, parse_and_split };
}

/// Parented namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed {
  #[ allow( unused_imports ) ]
  use super::*;
  pub use prelude::*;
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
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use super::zero_copy::{ ZeroCopyStringExt, zero_copy_split };
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use super::parser::{ ParserIntegrationExt, ParsedToken, parse_and_split };
}

/// Namespace of the module to include with `use module::*`.
#[ allow( unused_imports ) ]
pub mod prelude {
  #[ allow( unused_imports ) ]
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
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use super::zero_copy::ZeroCopyStringExt;
  #[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
  pub use super::parser::ParserIntegrationExt;
}
