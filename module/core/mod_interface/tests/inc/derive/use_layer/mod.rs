
use super::*;
mod tools
{
  #[ allow( unused_imports ) ]
  pub use super::super::*;
}

pub mod layer_a;

/// SuperStruct1.
#[ derive( Debug, PartialEq ) ]
pub struct SuperStruct1
{
}

mod private {}

the_module::mod_interface!
{

  /// layer_a
  use super::layer_a;

}

//

include!( "../../only_test/use_non_layer_only_test.rs" );
