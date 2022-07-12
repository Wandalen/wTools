
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

TheModule::mod_interface!
{

  // xxx : test with `layer { layer_a, layer_a };`
  // xxx : test with `use { layer_a, layer_a };`

  /// layer_a
  layer layer_a;
  /// layer_b
  layer layer_b;

}

//

// include!( "../../only_test/layer_divergent_only_test.rs" );
include!( "../../only_test/layer_simple_only_test.rs" );

