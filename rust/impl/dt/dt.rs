/// Internal namespace.
pub( crate ) mod private
{
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Shared with parent namespace of the module
pub mod orphan
{
  pub use super::exposed::*;

}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
  #[ cfg( feature = "either" ) ]
  pub use ::either::Either;
  #[ cfg( feature = "type_constructor" ) ]
  pub use ::type_constructor::exposed::*;
  #[ cfg( feature = "interval" ) ]
  pub use ::winterval::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "either" ) ]
  pub use ::either::*;
  #[ cfg( feature = "type_constructor" ) ]
  pub use ::type_constructor::prelude::*;
  #[ cfg( feature = "interval" ) ]
  pub use ::winterval::prelude::*;
}
