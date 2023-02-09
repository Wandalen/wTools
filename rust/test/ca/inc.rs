#[ allow( unused_imports ) ]
use test_tools::exposed::*;
#[ allow( unused_imports ) ]
use wtools::meta::prelude::*;
#[ allow( unused_imports ) ]
use std::collections::HashMap;

#[ cfg( feature = "use_std" ) ]
mod parser;
#[ cfg( feature = "use_std" ) ]
mod executor;
