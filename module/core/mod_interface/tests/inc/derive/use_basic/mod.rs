
use super::*;

mod layer_a;
mod layer_b;

mod private {}

mod_interface!
{

  /// layer_a
  use super::layer_a;

  /// layer_b
  use super::layer_b;

}

//

include!( "../../only_test/layer_simple_only_test.rs" );
