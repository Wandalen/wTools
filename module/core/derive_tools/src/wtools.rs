//!
//! Types, which are extension of std.
//!

/// Internal namespace.
pub( crate ) mod private
{
}

#[ cfg( feature = "type_variadic_from" ) ]
pub use ::variadic_from::wtools::from;
// pub mod from;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ cfg( feature = "type_variadic_from" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::from::orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "type_variadic_from" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::from::prelude::*;
}
