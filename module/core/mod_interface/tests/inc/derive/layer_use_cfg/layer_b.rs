
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

/// Super struct.
#[ derive( Debug, PartialEq ) ]
pub struct SubStruct2
{
}

//

mod_interface!
{

  protected use layer_b_protected;
  orphan use { layer_b_orphan };
  exposed use { layer_b_exposed };
  prelude use layer_b_prelude;

}
