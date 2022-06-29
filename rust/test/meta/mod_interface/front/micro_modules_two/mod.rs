
use super::*;

/// Private namespace of the module.
mod private
{
}

TheModule::mod_interface!
{

  protected mod mod_protected1;
  protected mod mod_protected2;
  orphan mod mod_orphan1;
  orphan mod mod_orphan2;
  exposed mod mod_exposed1;
  exposed mod mod_exposed2;
  prelude mod mod_prelude1;
  prelude mod mod_prelude2;

}

//

include!( "../../only_test/micro_modules_two_only_test.rs" );
