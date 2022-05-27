
#[ allow( unused_imports ) ]
use super::TheModule;

#[ cfg( feature = "type_constructor" ) ]
mod single_test;
#[ cfg( feature = "type_constructor" ) ]
mod pair_test;
#[ cfg( feature = "type_constructor" ) ]
mod homo_pair_test;
#[ cfg( feature = "type_constructor" ) ]
mod many_test;

#[ cfg( feature = "make" ) ]
mod make_interface_test;
