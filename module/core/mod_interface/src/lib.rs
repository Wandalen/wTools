/// Private namespace of the module. Should contain the actual implementation.
mod private
{
}

/// Own namespace of the module. Contains items public within this layer, but not propagated.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*; // Includes items from the orphan level
  /// Function specific to layer_a's `own` level.
  pub fn layer_a_own() -> bool
  {
    true
  }
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*; // Exports items from `own` and `orphan` levels to the root of the layer.

/// Orphan namespace of the module. Contains items propagated to the immediate parent.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*; // Includes items from the exposed level
  /// Function specific to layer_a's `orphan` level.
  pub fn layer_a_orphan() -> bool
  {
    true
  }
}

/// Exposed namespace of the module. Contains items propagated to all ancestor layers.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*; // Includes items from the prelude level
  /// Function specific to layer_a's `exposed` level.
  pub fn layer_a_exposed() -> bool
  {
    true
  }
}

/// Prelude namespace of the module. Contains items propagated to all ancestors and intended for glob import.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  /// Function specific to layer_a's `prelude` level.
  pub fn layer_a_prelude() -> bool
  {
    true
  }
}