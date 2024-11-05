
//!
//! Version of Rust compiler
//!

/// Internal namespace.
// #[ cfg( not( feature = "no_std" ) ) ]
mod private
{
}


// //
// // #[ cfg( not( feature = "no_std" ) ) ]
// crate::mod_interface!
// {
//
//   // exposed use super;
//   exposed use super::super::version;
//
//   prelude use ::rustversion::{ nightly, stable };
//
// }


#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use
  {
    private::*,
  };

}

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;

  #[ doc( inline ) ]
  pub use exposed::*;

  pub use super::super::version;

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use rustversion::{ nightly, stable };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  pub use
  {
  };

}
