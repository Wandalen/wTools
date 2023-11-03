/// internal module
pub mod private {

  pub use iter_tools;
  /// strs tools
  pub mod string
  {
    pub use strs_tools::string::*;
  }
  
  /// error tools
  pub mod error {
    pub use error_tools::*;
  }

  // pub use error_tools::err;

  // pub use error_tools::BasicError;

  pub use mod_interface::*;
}

crate::mod_interface! {
  prelude use iter_tools::Itertools;
  prelude use error::err;
  prelude use string;
  prelude use error;
  prelude use mod_interface;
}