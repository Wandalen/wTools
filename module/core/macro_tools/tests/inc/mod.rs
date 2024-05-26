
#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;
#[ allow( unused_imports ) ]
use the_module::protected::*;

#[ allow( unused_imports ) ]
#[ cfg( feature = "enabled" ) ]
#[ path = "." ]
mod if_enabled
{

  use super::*;

  mod attr_test;
  mod attr_prop_test;
  mod basic_test;
  mod container_kind_test;
  mod derive_test;
  mod diag_test;
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
