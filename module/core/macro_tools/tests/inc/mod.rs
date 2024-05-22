
#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;

#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
#[ path = "." ]
mod if_enabled
{

  use super::*;
  use the_module::exposed::*;
  // xxx : use protected

  mod attr_test;
  mod basic_test;
  mod derive_test;
  mod drop_test;
  mod equation_test;
  mod generic_args_test;
  mod generic_params_test;
  mod item_test;
  mod item_struct_test;
  mod phantom_test;
  mod quantifier_test;
  mod struct_like_test;
  mod syntax_test;
  mod tokens_test;
  mod typ_test;

}
