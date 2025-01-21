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

  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use ::anyhow::
  {
    Chain,
    Context,
    Error,
    Ok,
    Result,
    format_err,
    bail as return_err,
    ensure,
    bail,
  };

}

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use super::super::untyped;
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use super::super::untyped as for_app;

  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use exposed::*;

  // #[ doc( inline ) ]
  // pub use ::anyhow::
  // {
  //   format_err,
  //   ensure,
  //   bail,
  // };

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
  use super::*;
}