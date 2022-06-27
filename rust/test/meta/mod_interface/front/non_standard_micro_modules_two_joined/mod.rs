
use super::TheModule;
use TheModule::prelude::*;

/// Private namespace of the module.
mod private
{
}

mod_interface!
{

// = use

  // use private::Type1;
  // use private::{ Type1, Type2 };
  // protected use private::Type1;
  // prelude use private::Type1;

// = ?

  // protected protected1;
  // orphan orphan1;
  // exposed exposed1;
  // prelude prelude1;
  // prelude { prelude1, prelude2 };

// = macro module

  // macromod mod1;
  // macromod mod2;
  // macromod { mod1, mod2 };

  // - narrowing

  // protected macromod mod_protected1;
  // orphan macromod mod_orphan1;
  // exposed macromod mod_exposed1;
  // prelude macromod mod_prelude1;

  // - extending

  // prelude exposed macromod mod_protected1;
  // : protected -> exposed
  // : orphan -> exposed
  // : exposed -> exposed
  // : prelude -> prelude

  // prelude protected macromod mod_exposed1;
  // : protected -> protected
  // : orphan -> orphan
  // : exposed -> exposed
  // : prelude -> prelude

  // - selective

  // exposed exposed macromod mod_exposed1;
  // : protected -> exposed
  // : orphan -> exposed
  // : exposed -> exposed
  // : prelude -> exposed

  // exposed orphan macromod mod_exposed1;
  // : protected -> orphan
  // : orphan -> orphan
  // : exposed -> exposed
  // : prelude -> exposed

// = micro module

  // mod mod1;
  // mod mod2;
  // mod { mod1, mod2 };

  protected mod { mod_protected1, mod_protected2 };
  orphan mod { mod_orphan1, mod_orphan2 };
  exposed mod { mod_exposed1, mod_exposed2 };
  prelude mod { mod_prelude1, mod_prelude2 };

}

//

include!( "../../test/non_standard_micro_modules_two_only_test.rs" );
