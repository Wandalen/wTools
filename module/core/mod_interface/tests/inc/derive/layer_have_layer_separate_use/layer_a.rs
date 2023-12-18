
use super::tools::*;

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

//

mod_interface!
{

  protected use { layer_a_protected };
  orphan use layer_a_orphan;
  exposed use layer_a_exposed;
  prelude use layer_a_prelude;

}
