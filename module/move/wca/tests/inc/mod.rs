#[ allow( unused_imports ) ]
use super::*;

#[ allow( unused_imports ) ]
use the_module::tool::*;
#[ allow( unused_imports ) ]
use std::collections::HashMap;

#[ cfg( not( feature = "no_std" ) ) ]
mod parser;
#[ cfg( not( feature = "no_std" ) ) ]
mod grammar;
#[ cfg( not( feature = "no_std" ) ) ]
mod executor;
#[ cfg( not( feature = "no_std" ) ) ]
mod commands_aggregator;

// qqq : for Bohdan : why commented out? resolve
// #[ cfg( not( feature = "no_std" ) ) ]
// mod adapter;
