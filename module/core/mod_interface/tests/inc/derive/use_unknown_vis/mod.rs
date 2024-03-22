
use super::*;

/// Private
mod private
{

  pub fn f1(){}

}

the_module::mod_interface!
{

  /// layer_a
  xyz use f1;

}
