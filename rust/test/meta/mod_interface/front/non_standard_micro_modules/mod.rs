
only_for_local_module!( use ::mod_interface::prelude::* );
only_for_wtools!( use ::wtools::prelude::* );

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

  // private mod mod_private;
  protected mod mod_protected;
  orphan mod mod_orphan;
  exposed mod mod_exposed;
  prelude mod mod_prelude;

}

//

include!( "../../test/non_standard_micro_modules_only_test.rs" );
