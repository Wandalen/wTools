
use super::*;

/// Private namespace of the module.
mod private
{
}

TheModule::mod_interface!
{

  layer layer_a;
  layer layer_b;

}

//

include!( "../../only_test/layer_simple_only_test.rs" );
