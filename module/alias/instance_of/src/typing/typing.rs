
/// Protected namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::inspect_type::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::is_slice::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::implements::orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::inspect_type::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::is_slice::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::implements::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::inspect_type::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::is_slice::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::implements::prelude::*;
}
