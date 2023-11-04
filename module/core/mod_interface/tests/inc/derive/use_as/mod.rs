
use super::*;

// /// Private namespace of the module.
// mod private
// {
// }

mod layer_x;
mod layer_y;

TheModule::mod_interface!
{
  #![ debug ]

  /// layer_a
  use super::layer_x as layer_a;

  // /// layer_b
  // use super::layer_y as layer_b;

}

//

// include!( "../../only_test/layer_simple_only_test.rs" );
