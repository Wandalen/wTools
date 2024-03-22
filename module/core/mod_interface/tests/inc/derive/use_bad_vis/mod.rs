
use super::*;

/// Private
mod private
{

  pub fn f1(){}

}

the_module::mod_interface!
{

  /// layer_a
  pub use f1;

}
