
use super::*;

/// Private namespace of the module.
mod private
{
}

mod_interface!
{

  /// mod_protected
  protected mod mod_protected;
  /// mod_orphan
  orphan mod mod_orphan;
  /// mod_exposed
  exposed mod mod_exposed;
  /// mod_prelude
  prelude mod mod_prelude;

}

//

include!( "../../only_test/micro_modules_only_test.rs" );
