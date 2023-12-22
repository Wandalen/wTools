pub use error_tools::err;

// pub use error_tools::BasicError;

pub use mod_interface::*;

/// error tools
pub mod error
{
  pub use error_tools::*;
  pub use error_tools::for_lib::*;
  pub use ::error_tools::dependency::*;
}
