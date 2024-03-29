
/// Private namespace of the module.
mod private
{
}

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  /// layer_a_protected
  pub fn layer_a_protected() -> bool
  {
    true
  }
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
  /// layer_a_orphan
  pub fn layer_a_orphan() -> bool
  {
    true
  }
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  /// layer_a_exposed
  pub fn layer_a_exposed() -> bool
  {
    true
  }
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  /// layer_a_prelude
  pub fn layer_a_prelude() -> bool
  {
    true
  }
}
