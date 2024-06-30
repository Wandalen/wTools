//!
//! Typed parsing.
//!

/// Internal namespace.
pub( crate ) mod private
{
  // use crate::*;

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{

  #[ doc( inline ) ]
  pub use super::orphan::*;

  #[ doc( inline ) ]
  pub use super::private::
  {
  };

  pub use syn::{ parse_quote, parse_quote as qt };

}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::super::typed;

  // pub use super::protected as typ;

  #[ doc( inline ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
}
