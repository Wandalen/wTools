
#[ allow( unused_imports ) ]
use super::*;

// #[ cfg( any( feature = "type_constructor", feature = "dt_type_constructor" ) ) ]
// #[ cfg( any( feature = "type_constructor", feature = "dt_type_constructor" ) ) ]
// #[ cfg( feature = "enabled" ) ]
// mod type_constructor;

#[ cfg( feature = "enabled" ) ]
#[ cfg( any( feature = "prelude", feature = "dt_prelude" ) ) ]
mod prelude_test;

// #[ allow( unused_imports ) ]
// use super::*;

#[ cfg( feature = "enabled" ) ]
mod single
{
  use super::*;

  mod single_parameter_main_gen_test;
  mod single_parameter_main_manual_test;
  mod single_parameter_test;
  mod single_parametrized_main_gen_test;
  mod single_parametrized_main_manual_test;
  mod single_parametrized_test;
}

#[ cfg( feature = "enabled" ) ]
#[ cfg
(
  all
  (
    // feature = "make",
    any( not( feature = "no_std" ), feature = "use_alloc" ),
  )
)]
mod pair
{
  use super::*;

  mod pair_parameter_main_gen_test;
  mod pair_parameter_main_manual_test;
  // mod pair_parameter_test;
  mod pair_parametrized_main_gen_test;
  mod pair_parametrized_main_manual_test;
  // mod pair_parametrized_test;

  // mod homo_pair_parameter_main_gen_test;
  // mod homo_pair_parameter_main_manual_test;
  // mod homo_pair_parameter_test;
  // mod homo_pair_parametrized_main_gen_test;
  mod homo_pair_parametrized_main_manual_test;
  // mod homo_pair_parametrized_test;

}

#[ cfg( feature = "enabled" ) ]
#[ cfg
(
  all
  (
    feature = "many",
    any( not( feature = "no_std" ), feature = "use_alloc" ),
  )
)]
mod many
{
  use super::*;
  // mod many_parameter_main_manual_test;
  // mod many_parameter_main_gen_test;
  mod many_parameter_test;
  mod many_parametrized_main_manual_test;
  mod many_parametrized_main_gen_test;
  mod many_parametrized_test;
}

// #[ cfg( feature = "enabled" ) ]
// #[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
// mod make_interface_test;

#[ cfg( feature = "enabled" ) ]
#[ cfg( any( feature = "vectorized_from", feature = "dt_vectorized_from" ) ) ]
mod vectorized_from_test;

#[ cfg( feature = "enabled" ) ]
mod enumerable_test;
