
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

/// SuperStruct1.
#[ derive( Debug, PartialEq ) ]
pub struct SuperStruct1
{
}

TheModule::mod_interface!
{

  /// layer_a
  layer layer_a;

}

//

include!( "../../only_test/use_non_layer_only_test.rs" );
