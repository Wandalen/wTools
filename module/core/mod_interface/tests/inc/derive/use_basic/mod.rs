
use super::*;

// private layer
pub mod layer_a;
// private layer
pub mod layer_b;

mod private {}

the_module::mod_interface!
{

  /// layer_a
  use super::layer_a;

  /// layer_b
  use super::layer_b;

}

//

include!( "../../only_test/layer_simple_only_test.rs" );
