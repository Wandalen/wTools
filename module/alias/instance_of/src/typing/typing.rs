
/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use orphan::*;
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
pub use own::*;

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use prelude::*;
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
  use super::*;
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
