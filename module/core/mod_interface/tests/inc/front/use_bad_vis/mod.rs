
use super::*;

/// Private
mod private
{

  pub fn f1(){}

}

TheModule::mod_interface!
{

  /// layer_a
  pub use f1;

}
