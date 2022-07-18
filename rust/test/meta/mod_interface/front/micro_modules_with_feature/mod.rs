
use super::*;

/// Private namespace of the module.
mod private
{
}

TheModule::mod_interface!
{
  /// mod_protected
  #[ cfg( feature = "use_std" ) ]
  protected mod mod_protected;
}

//

include!( "../../only_test/micro_modules_with_feature_only_test.rs" );
