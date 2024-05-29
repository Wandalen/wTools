// #[ cfg( feature = "string" ) ]
// use super::*;
// use crate::the_module::string as the_module;

// #[ cfg( feature = "string" ) ]
// mod inc;

#[ allow( unused_imports ) ]
use test_tools::exposed::*;
#[ allow( unused_imports ) ]
use super::*;

#[ cfg( all( feature = "string_indentation", not( feature = "no_std" ) ) ) ]
mod indentation_test;
#[ cfg( all( feature = "string_isolate", not( feature = "no_std" ) ) ) ]
mod isolate_test;
#[ cfg( all( feature = "string_parse_number", not( feature = "no_std" ) ) ) ]
mod number_test;
#[ cfg( all( feature = "string_parse", not( feature = "no_std" ) ) ) ]
mod parse_test;
#[ cfg( all( feature = "string_split", not( feature = "no_std" ) ) ) ]
mod split_test;
