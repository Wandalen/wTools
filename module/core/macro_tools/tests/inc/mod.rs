
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

  #[ cfg( feature = "attr" ) ]
  mod attr_test;
  #[ cfg( feature = "attr_prop" ) ]
  mod attr_prop_test;
  mod basic_test;
  #[ cfg( feature = "ct" ) ]
  mod compile_time_test;
  #[ cfg( feature = "container_kind" ) ]
  mod container_kind_test;
  #[ cfg( feature = "derive" ) ]
  mod derive_test;
  #[ cfg( feature = "diag" ) ]
  mod diag_test;
  mod drop_test;
  #[ cfg( feature = "equation" ) ]
  mod equation_test;
  #[ cfg( feature = "generic_args" ) ]
  mod generic_args_test;
  #[ cfg( feature = "generic_params" ) ]
  mod generic_params_test;
  #[ cfg( feature = "item" ) ]
  mod item_test;
  #[ cfg( feature = "item_struct" ) ]
  mod item_struct_test;
  #[ cfg( feature = "phantom" ) ]
  mod phantom_test;
  #[ cfg( feature = "quantifier" ) ]
  mod quantifier_test;
  #[ cfg( feature = "struct_like" ) ]
  mod struct_like_test;
  #[ cfg( feature = "tokens" ) ]
  mod tokens_test;
  #[ cfg( feature = "typ" ) ]
  mod typ_test;

}
