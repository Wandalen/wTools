
/// Private namespace of the module.
mod private
{
}

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  /// layer_b_protected
  pub fn layer_b_protected() -> bool
  {
    true
  }
}

#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
  /// layer_b_orphan
  pub fn layer_b_orphan() -> bool
  {
    true
  }
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
  /// layer_b_exposed
  pub fn layer_b_exposed() -> bool
  {
    true
  }
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  /// layer_b_prelude
  pub fn layer_b_prelude() -> bool
  {
    true
  }
}
