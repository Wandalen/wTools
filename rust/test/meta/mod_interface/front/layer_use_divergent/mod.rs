
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

/// Super struct.
#[ derive( Debug, PartialEq ) ]
pub struct SuperStruct1
{
}

TheModule::mod_interface!
{

  /// layer_a
  layer layer_a;
  /// layer_b
  layer layer_b;

}

//

include!( "../../only_test/layer_divergent_only_test.rs" );
