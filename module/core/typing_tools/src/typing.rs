
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ cfg( inspect_type ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::inspect_type::orphan::*;
  #[ cfg( is_slice ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::is_slice::orphan::*;
  // #[ cfg( implements ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::implements::orphan::*;
}

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
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( inspect_type ) ]
  pub use ::inspect_type::exposed::*;
  #[ cfg( is_slice ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::is_slice::exposed::*;
  // #[ cfg( implements ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::implements::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( inspect_type ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::inspect_type::prelude::*;
  #[ cfg( is_slice ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::is_slice::prelude::*;
  // #[ cfg( implements ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::implements::prelude::*;
}
