// #[ cfg( feature = "string" ) ]
// use super :: *;
// use crate ::the_module ::string as the_module;

// #[ cfg( feature = "string" ) ]
// mod inc;

#![allow(unexpected_cfgs)]
#[ allow( unused_imports ) ]
use test_tools :: *;
#[ allow( unused_imports ) ]
use super :: *;

#[ cfg(all(feature = "string_indentation", feature = "std")) ]
mod indentation_test;
#[ cfg(all(feature = "string_isolate", feature = "std")) ]
mod isolate_test;
#[ cfg(all(feature = "string_parse_number", feature = "std")) ]
mod number_test;
#[ cfg(all(feature = "string_parse", feature = "std")) ]
mod parse_test;
#[ cfg(all(feature = "string_split", feature = "std")) ]
pub mod split_test;

pub mod iterator_vec_delimiter_test;

#[ cfg( feature = "ansi" ) ]
mod ansi_detect_test;
#[ cfg( feature = "ansi" ) ]
mod ansi_parse_test;
#[ cfg( feature = "ansi" ) ]
mod ansi_strip_test;
#[ cfg( feature = "ansi" ) ]
mod ansi_truncate_test;
#[ cfg( feature = "ansi" ) ]
mod ansi_visual_test;
#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
mod string_parser_test;
#[ cfg( all( feature = "string_split", feature = "std" ) ) ]
mod zero_copy_test;
#[ cfg( all( feature = "string_split", feature = "specialized_algorithms", feature = "std" ) ) ]
mod specialized_test;
