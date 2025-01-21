
/// Private namespace of the module.
mod private
{

  /// layer_b_own
  pub fn layer_b_own() -> bool
  {
    true
  }

  /// layer_b_orphan
  pub fn layer_b_orphan() -> bool
  {
    true
  }

  /// layer_b_exposed
  pub fn layer_b_exposed() -> bool
  {
    true
  }

  /// layer_b_prelude
  pub fn layer_b_prelude() -> bool
  {
    true
  }

}

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::layer_b_own;
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
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::layer_b_orphan;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::layer_b_exposed;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::layer_b_prelude;
}
