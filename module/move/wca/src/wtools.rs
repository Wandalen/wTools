
crate::mod_interface!
{
  protected use ::iter_tools::Itertools;
  protected use ::error_tools::err;
  protected use ::error_tools::dependency::*;
  use ::strs_tools as string;
  use ::error_tools as error;
  use ::mod_interface;
}

// /// Requests parser.
// #[ cfg( not( feature = "no_std" ) ) ]
// pub mod string
// {
//   pub use strs_tools::string::*;
// }
