
/// Get time right now.
#[ cfg( feature = "now" ) ]
#[ path = "./now.rs" ]
pub mod now;

/// Dependencies.
#[ cfg( feature = "enabled" ) ]
pub mod dependency
{
}

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Shared with parent namespace of the module
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
  #[ cfg( feature = "now" ) ]
  #[ doc( inline ) ]
  pub use super::now::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
