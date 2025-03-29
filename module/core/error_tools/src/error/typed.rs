/// Define a private namespace for all its items.
mod private
{

}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ allow( clippy::pub_use ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use orphan::*;
}

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use super::super::typed;
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use super::super::typed as for_lib;

  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use exposed::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ allow( clippy::pub_use ) ]
  pub use ::thiserror::
  {
    Error,
  };

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;

  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use prelude::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ allow( clippy::pub_use ) ]
  pub use thiserror;

}