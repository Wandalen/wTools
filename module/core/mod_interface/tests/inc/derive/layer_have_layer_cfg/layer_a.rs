
use super::tools::*;

/// Private namespace of the module.
mod private
{

  /// layer_a_own
  pub fn layer_a_own() -> bool
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

  // orphan use super::private::
  // {
  //   protected where layer_a_own as layer_a_own2,
  //   layer_a_orphan,
  //   exposed where layer_a_exposed,
  //   prelude where layer_a_prelude,
  // };

  own use { layer_a_own };
  orphan use layer_a_orphan;
  exposed use layer_a_exposed;
  prelude use layer_a_prelude;

}
