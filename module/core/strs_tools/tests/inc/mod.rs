// #[ cfg( feature = "string" ) ]
// use super::*;
// use crate::TheModule::string as TheModule;

// #[ cfg( feature = "string" ) ]
// mod inc;

#[ allow( unused_imports ) ]
use test_tools::exposed::*;
#[ allow( unused_imports ) ]
use super::*;

#[ cfg( all( feature = "string_indentation", feature = "use_std" ) ) ]
mod indentation_test;
#[ cfg( all( feature = "string_isolate", feature = "use_std" ) ) ]
mod isolate_test;
#[ cfg( all( feature = "string_parse_number", feature = "use_std" ) ) ]
mod number_test;
#[ cfg( all( feature = "string_parse", feature = "use_std" ) ) ]
mod parse_test;
#[ cfg( all( feature = "string_split", feature = "use_std" ) ) ]
mod split_test;
