
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ cfg( feature = "typing_inspect_type" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::inspect_type::orphan::*;
  #[ cfg( feature = "typing_is_slice" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::is_slice::orphan::*;
  #[ cfg( feature = "typing_implements" ) ]
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
  #[ cfg( feature = "typing_inspect_type" ) ]
  pub use ::inspect_type::exposed::*;
  #[ cfg( feature = "typing_is_slice" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::is_slice::exposed::*;
  #[ cfg( feature = "typing_implements" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::implements::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "typing_inspect_type" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::inspect_type::prelude::*;
  #[ cfg( feature = "typing_is_slice" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::is_slice::prelude::*;
  #[ cfg( feature = "typing_implements" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::implements::prelude::*;
}
