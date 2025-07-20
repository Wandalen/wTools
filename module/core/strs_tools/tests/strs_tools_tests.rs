//! Test suite for the `strs_tools` crate.

#[ allow( unused_imports ) ]
use strs_tools as the_module;
mod inc;



#[ path = "./inc/split_test/split_behavior_tests.rs" ]
mod split_behavior_tests;
