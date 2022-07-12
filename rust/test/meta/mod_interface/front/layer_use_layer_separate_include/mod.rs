
use super::*;
mod tools
{
  #[ allow( unused_imports ) ]
  pub use super::super::*;
}

/// Private namespace of the module.
mod private
{
}

/// layer_a
mod layer_a;
/// layer_b
mod layer_b;

TheModule::mod_interface!
{

  // xxx : test with `layer { layer_a, layer_a };`
  // xxx : test with `use { layer_a, layer_a };`

  /// layer_a
  use layer_a;
  /// layer_b
  use layer_b;

}

//

include!( "../../only_test/layer_simple_only_test.rs" );
