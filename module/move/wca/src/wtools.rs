/// internal module
pub mod private {

  pub use iter_tools;
  /// strs tools
  pub mod string
  {
    pub use strs_tools::string::*;
  }

  pub use error_tools::err;

  // pub use error_tools::BasicError;

  pub use mod_interface;

  /// error tools
  pub mod error 
  {
    pub use error_tools::*;
  }
}


// crate::mod_interface! 
// {
//   exposed use iter_tools::Itertools;
//   exposed use error::err;

//   use string;
//   use error;
//   use mod_interface;
// }

pub use iter_tools::*;
  /// strs tools
pub mod string
{
  pub use strs_tools::string::*;
}

pub use error_tools::err;

// pub use error_tools::BasicError;

pub use mod_interface;

  /// error tools
pub mod error 
{
  pub use error_tools::*;
}