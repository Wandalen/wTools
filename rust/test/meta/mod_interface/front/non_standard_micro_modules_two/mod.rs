
use super::TheModule;
use TheModule::prelude::*;

/// Private namespace of the module.
mod private
{
}

mod_interface!
{

//   mod mod1;
//   mod mod2;
//
//   protected1 protected1;
//   orphan orphan1;
//   exposed exposed1;
//   prelude prelude1;

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

include!( "../../test/non_standard_micro_modules_two_only_test.rs" );
