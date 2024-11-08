
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

the_module::mod_interface!
{

  /// mod_a
  orphan mod mod_a;
  #[ cfg( all() ) ]
  /// mod_b
  orphan mod mod_b;
  #[ cfg( any() ) ]
  /// mod_c
  orphan mod mod_c;

}

//

include!( "../../only_test/layer_have_mod_cfg_test_only.rs" );
