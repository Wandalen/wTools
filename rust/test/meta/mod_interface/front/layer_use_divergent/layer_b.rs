
use super::tools::*;

/// Private namespace of the module.
mod private
{

  /// layer_b_protected
  pub fn layer_b_protected() -> bool
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

//

TheModule::mod_interface!
{

  use private::layer_b_protected;
  orphan use private::layer_b_orphan;
  exposed use private::layer_b_exposed;
  prelude use private::layer_b_prelude;

}
