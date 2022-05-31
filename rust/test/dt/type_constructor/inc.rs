
#[ allow( unused_imports ) ]
use super::TheModule;

mod single_test;
mod pair_test;
mod homo_pair_test;
#[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
mod many_test;

#[ cfg( any( feature = "make", feature = "dt_make" ) ) ]
mod make_interface_test;
