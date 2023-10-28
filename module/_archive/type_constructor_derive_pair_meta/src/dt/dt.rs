/// Internal namespace.
pub( crate ) mod private
{
}

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

/// Shared with parent namespace of the module
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
  #[ cfg( feature = "either" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::either::Either;
  #[ cfg( feature = "type_constructor" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::type_constructor::exposed::*;
  #[ cfg( feature = "interval" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::interval_adapter::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // #[ cfg( feature = "either" ) ]
  // pub use ::either::*;
  #[ cfg( feature = "type_constructor" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::type_constructor::prelude::*;
  #[ cfg( feature = "interval" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::interval_adapter::prelude::*;
}
