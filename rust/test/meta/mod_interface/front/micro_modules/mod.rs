
use super::TheModule;
use TheModule::prelude::*;

/// Private namespace of the module.
mod private
{
}

mod_interface!
{

  protected mod mod_protected;
  orphan mod mod_orphan;
  exposed mod mod_exposed;
  prelude mod mod_prelude;

}

//

include!( "../../test/non_standard_micro_modules_only_test.rs" );
