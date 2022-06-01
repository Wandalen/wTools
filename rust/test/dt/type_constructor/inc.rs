
#[ allow( unused_imports ) ]
use super::TheModule;

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
mod many_test;

#[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
mod make_interface_test;

#[ cfg( any( feature = "vectorized_from", feature = "dt_vectorized_from" ) ) ]
mod vectorized_from_test;
