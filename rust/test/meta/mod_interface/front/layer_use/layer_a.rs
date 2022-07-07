
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

TheModule::mod_interface!
{

  // orphan use super::private::
  // {
  //   protected where layer_a_protected as layer_a_protected2,
  //   layer_a_orphan,
  //   exposed where layer_a_exposed,
  //   prelude where layer_a_prelude,
  // };

  protected use private::layer_a_protected;
  orphan use private::layer_a_orphan;
  exposed use private::layer_a_exposed;
  prelude use private::layer_a_prelude;

}
