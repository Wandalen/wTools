
/// Get time right now.
#[ path = "./now.rs" ]
pub mod now;

/// Dependencies.
pub mod dependencies
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
  pub use super::now::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
}
