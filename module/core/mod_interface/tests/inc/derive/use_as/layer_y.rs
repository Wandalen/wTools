
/// Private namespace of the module.
mod private
{
}

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  /// layer_b_own
  pub fn layer_b_own() -> bool
  {
    true
  }
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
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
  pub use prelude::*;
  /// layer_b_exposed
  pub fn layer_b_exposed() -> bool
  {
    true
  }
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  /// layer_b_prelude
  pub fn layer_b_prelude() -> bool
  {
    true
  }
}
