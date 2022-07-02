
#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use test_tools::*;

use TheModule::prelude::*;
use TheModule::qt;

#[ cfg( feature = "use_std" ) ]
mod basic_test;
mod syntax_test;
