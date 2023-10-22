//!
//! Types, which are extension of std.
//!

/// Internal namespace.
pub( crate ) mod private
{
}

#[ cfg( feature = "type_from" ) ]
pub mod from;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
  #[ cfg( feature = "type_from" ) ]
  pub use super::from::orphan::*;
}

#[ doc( inline ) ]
pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "type_from" ) ]
  #[ doc( inline ) ]
  pub use super::from::prelude::*;
}
