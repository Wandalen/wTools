/// Internal namespace.
mod private
{
}

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
}

/// Shared with parent namespace of the module
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

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ cfg( feature = "either" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::either::Either;

  // #[ cfg( feature = "type_constructor" ) ]
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use ::type_constructor::exposed::*;

  #[ cfg( feature = "dt_interval" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use crate::dependency::interval_adapter::exposed::*;

  #[ cfg( feature = "dt_collection" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use crate::dependency::collection_tools::exposed::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  // #[ cfg( feature = "either" ) ]
  // pub use ::either::*;
  // #[ cfg( feature = "type_constructor" ) ]
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use ::type_constructor::prelude::*;

  #[ cfg( feature = "dt_interval" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use crate::dependency::interval_adapter::prelude::*;

  #[ cfg( feature = "dt_collection" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use crate::dependency::collection_tools::prelude::*;

}
