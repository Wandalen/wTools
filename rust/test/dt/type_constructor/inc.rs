
#[ allow( unused_imports ) ]
use super::*;

mod single_test;
mod pair_test;
mod homo_pair_test;

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

