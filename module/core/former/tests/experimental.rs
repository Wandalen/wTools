
include!( "../../../../module/step/meta/src/module/terminal.rs" );

// #[ allow( unused_imports ) ]
// use test_tools::meta::*;
#[ allow( unused_imports ) ]
use test_tools::exposed::*;
#[ allow( unused_imports ) ]
use former as TheModule;

// #[ path = "./inc/parametrized_struct.rs" ]
// mod experimental;

#[ path = "./inc/name_conflict.rs" ]
mod experimental;