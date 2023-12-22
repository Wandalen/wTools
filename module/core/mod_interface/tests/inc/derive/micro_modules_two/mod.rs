
use super::*;

/// Private namespace of the module.
mod private
{
}

mod_interface!
{

  /// mod_protected1
  protected mod mod_protected1;
  /// mod_protected2
  protected mod mod_protected2;
  /// mod_orphan1
  orphan mod mod_orphan1;
  /// mod_orphan2
  orphan mod mod_orphan2;
  /// mod_exposed1
  exposed mod mod_exposed1;
  /// mod_exposed2
  exposed mod mod_exposed2;
  /// mod_prelude1
  prelude mod mod_prelude1;
  /// mod_prelude2
  prelude mod mod_prelude2;

}

//

include!( "../../only_test/micro_modules_two_only_test.rs" );
