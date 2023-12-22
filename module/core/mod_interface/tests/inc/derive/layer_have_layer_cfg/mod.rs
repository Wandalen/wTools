
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

mod_interface!
{

  /// layer_a
  layer layer_a;

  #[ cfg( all() ) ]
  /// layer_b
  layer layer_b;

  #[ cfg( any() ) ]
  /// layer_c
  layer layer_c;

}

//

include!( "../../only_test/layer_simple_only_test.rs" );
