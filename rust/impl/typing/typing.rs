
/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  pub use ::inspect_type::orphan::*;
  #[ doc( inline ) ]
  pub use ::is_slice::orphan::*;
  #[ doc( inline ) ]
  pub use ::implements::orphan::*;
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
  #[ doc( inline ) ]
  pub use ::inspect_type::exposed::*;
  #[ doc( inline ) ]
  pub use ::is_slice::exposed::*;
  #[ doc( inline ) ]
  pub use ::implements::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use ::inspect_type::prelude::*;
  #[ doc( inline ) ]
  pub use ::is_slice::prelude::*;
  #[ doc( inline ) ]
  pub use ::implements::prelude::*;
}
