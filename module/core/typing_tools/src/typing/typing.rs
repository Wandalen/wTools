
#[ doc( inline ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  #[ cfg( inspect_type ) ]
  #[ doc( inline ) ]
  pub use ::inspect_type::orphan::*;
  #[ cfg( is_slice ) ]
  #[ doc( inline ) ]
  pub use ::is_slice::orphan::*;
  // #[ cfg( implements ) ]
  #[ doc( inline ) ]
  pub use ::implements::orphan::*;
}

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
  #[ doc( inline ) ]
  #[ cfg( inspect_type ) ]
  pub use ::inspect_type::exposed::*;
  #[ cfg( is_slice ) ]
  #[ doc( inline ) ]
  pub use ::is_slice::exposed::*;
  // #[ cfg( implements ) ]
  #[ doc( inline ) ]
  pub use ::implements::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( inspect_type ) ]
  #[ doc( inline ) ]
  pub use ::inspect_type::prelude::*;
  #[ cfg( is_slice ) ]
  #[ doc( inline ) ]
  pub use ::is_slice::prelude::*;
  // #[ cfg( implements ) ]
  #[ doc( inline ) ]
  pub use ::implements::prelude::*;
}
