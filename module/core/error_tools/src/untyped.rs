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

  #[ doc( inline ) ]
  pub use ::anyhow::
  {
    Chain,
    Context,
    Error,
    Ok,
    Result,
  };

}

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  pub use super::super::untyped;
  pub use super::super::untyped as for_app;

  #[ doc( inline ) ]
  pub use super::exposed::*;

  #[ doc( inline ) ]
  pub use ::anyhow::
  {
    anyhow,
    format_err,
    ensure,
    bail,
  };

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use super::prelude::*;

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
}