/// Internal namespace.
pub( crate ) mod private
{

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Shared with parent namespace of the module
pub mod orphan
{
  pub use super::super::untyped;
  pub use super::super::untyped as for_app;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use ::anyhow::*;

  // xxx : qqq : be specific

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
  pub use ::anyhow::Result;

  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use ::anyhow::prelude::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}