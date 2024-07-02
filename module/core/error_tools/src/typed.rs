/// Internal namespace.
pub( crate ) mod private
{

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  pub use super::super::typed;
  pub use super::super::typed as for_lib;

  #[ doc( inline ) ]
  pub use super::exposed::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::thiserror::*;
  // xxx : qqq : be specific

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use super::prelude::*;

  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use ::thiserror::prelude::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use thiserror;

}