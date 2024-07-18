//!
//! Compile-time tools.
//!

/// Internal namespace.
pub( crate ) mod private
{
}

/// Compile-time const expressions for strings.
pub mod str;

/// Compile-time tools.
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  pub use private::
  {
  };
  #[ doc( inline ) ]
  pub use ::const_format::*;
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::super::ct;

  // pub use super::own as ct;
  #[ doc( inline ) ]
  pub use prelude::*;
  #[ doc( inline ) ]
  pub use private::
  {
  };
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
