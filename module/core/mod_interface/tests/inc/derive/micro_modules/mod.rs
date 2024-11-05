
use super::*;

/// Private namespace of the module.
mod private
{
}

mod_interface!
{
  // #![ debug ]

  /// mod_own
  own mod mod_own;
  /// mod_orphan
  orphan mod mod_orphan;
  /// mod_exposed
  exposed mod mod_exposed;
  /// mod_prelude
  prelude mod mod_prelude;

}

//

include!( "../../only_test/micro_modules_only_test.rs" );
