
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
pub mod layer_a;
/// layer_b
pub mod layer_b;

the_module::mod_interface!
{

  /// layer_a
  use super::layer_a;
  /// layer_b
  use super::layer_b;

}

//

include!( "../../only_test/layer_simple_only_test.rs" );
