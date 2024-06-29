
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
  /// layer_b_protected
  pub fn layer_b_protected() -> bool
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
  /// layer_b_orphan
  pub fn layer_b_orphan() -> bool
  {
    true
  }
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
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
