
#[ allow( unused_imports ) ]
use super::TheModule;

#[ cfg( feature = "types" ) ]
mod single_test;
#[ cfg( feature = "types" ) ]
mod pair_test;
#[ cfg( feature = "types" ) ]
mod homo_pair_test;
#[ cfg( feature = "types" ) ]
mod many_test;

#[ cfg( feature = "make" ) ]
mod make_interface_test;
