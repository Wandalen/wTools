
/// Private namespace of the module.
mod private
{

  /// layer_a_protected
  pub fn layer_a_protected() -> bool
  {
    true
  }

  /// layer_a_orphan
  pub fn layer_a_orphan() -> bool
  {
    true
  }

  /// layer_a_exposed
  pub fn layer_a_exposed() -> bool
  {
    true
  }

  /// layer_a_prelude
  pub fn layer_a_prelude() -> bool
  {
    true
  }

}

/// Protected namespace of the module.
#[ allow( unused_imports ) ]
pub mod protected
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::layer_a_protected;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::layer_a_orphan;
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
  pub use private::layer_a_exposed;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use private::layer_a_prelude;
}
