
#[ allow( unused_imports ) ]
use super::*;

mod single_parameter_main_gen_test;
mod single_parameter_main_manual_test;
mod single_parameter_test;
mod single_parametrized_main_gen_test;
mod single_parametrized_main_manual_test;
mod single_parametrized_test;

/* qqq : for Dima : split this tests */
#[ cfg
(
  all
  (
    feature = "make",
    any( feature = "use_std", feature = "use_alloc" ),
  )
)]
#[ path = "." ]
mod pair
{
  use super::*;
  mod pair_parameter_main_gen_test;
  mod pair_parameter_main_manual_test;
  mod pair_parameter_test;
  mod pair_parametrized_main_gen_test;
  mod pair_parametrized_main_manual_test;
  mod pair_parametrized_test;
}

#[ cfg
(
  all
  (
    feature = "make",
    any( feature = "use_std", feature = "use_alloc" ),
  )
)]
#[ path = "." ]
mod homo_pair
{
  use super::*;
  mod homo_pair_parameter_main_gen_test;
  mod homo_pair_parameter_main_manual_test;
  mod homo_pair_parameter_test;
  mod homo_pair_parametrized_main_gen_test;
  mod homo_pair_parametrized_main_manual_test;
  mod homo_pair_parametrized_test;
}

#[ cfg
(
  all
  (
    feature = "many",
    any( feature = "use_std", feature = "use_alloc" ),
  )
)]
#[ path = "." ]
mod many
{
  use super::*;
  mod many_parameter_main_manual_test;
  mod many_parameter_main_gen_test;
  mod many_parameter_test;
  mod many_parametrized_main_manual_test;
  mod many_parametrized_main_gen_test;
  mod many_parametrized_test;
}

#[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
mod make_interface_test;

#[ cfg( any( feature = "vectorized_from", feature = "dt_vectorized_from" ) ) ]
mod vectorized_from_test;

mod enumerable_test;
