/// Internal namespace.
pub( crate ) mod private
{
}

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Shared with parent namespace of the module
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
  #[ cfg( feature = "either" ) ]
  #[ doc( inline ) ]
  pub use ::either::Either;
  #[ cfg( feature = "type_constructor" ) ]
  #[ doc( inline ) ]
  pub use ::type_constructor::exposed::*;
  #[ cfg( feature = "interval" ) ]
  #[ doc( inline ) ]
  pub use ::winterval::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  // #[ cfg( feature = "either" ) ]
  // pub use ::either::*;
  #[ cfg( feature = "type_constructor" ) ]
  #[ doc( inline ) ]
  pub use ::type_constructor::prelude::*;
  #[ cfg( feature = "interval" ) ]
  #[ doc( inline ) ]
  pub use ::winterval::prelude::*;
}
