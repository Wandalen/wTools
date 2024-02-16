
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

mod_interface!
{

  /// layer_a
  use super::layer_a;
  #[ cfg( all() ) ]
  /// layer_b
  use super::layer_b;
  #[ cfg( any() ) ]
  /// layer_c
  use super::layer_c;

}

//

include!( "../../only_test/layer_simple_only_test.rs" );

