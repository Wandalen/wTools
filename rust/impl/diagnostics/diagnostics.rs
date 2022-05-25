
/// Own namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "a_pretty" ) ]
  #[ doc( inline ) ]
  pub use ::pretty_assertions::assert_eq as a_id;
  #[ cfg( feature = "a_pretty" ) ]
  #[ doc( inline ) ]
  pub use ::pretty_assertions::assert_ne as a_not_id;
}
