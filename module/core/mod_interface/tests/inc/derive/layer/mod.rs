
use super::*;

/// Private namespace of the module.
mod private
{
}

the_module::mod_interface!
{

  /// layer_a
  layer layer_a;
  /// layer_b
  layer layer_b;

}

//

include!( "../../only_test/layer_simple_only_test.rs" );
