
use super::*;

/// Private namespace of the module.
mod private
{
}

the_module::mod_interface!
{

  own mod
  {
    /// mod_own1
    mod_own1,
    /// mod_own2
    mod_own2,
  };
  orphan mod
  {
    /// mod_orphan1
    mod_orphan1,
    /// mod_orphan2
    mod_orphan2,
  };
  exposed mod
  {
    /// mod_exposed1
    mod_exposed1,
    /// mod_exposed2
    mod_exposed2
  };
  /// Prelude
  prelude mod
  {
    mod_prelude1,
    mod_prelude2
  };

}

//

include!( "../../only_test/micro_modules_two_only_test.rs" );
