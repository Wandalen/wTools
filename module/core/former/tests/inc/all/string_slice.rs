use super::*;

// only_for_aggregating_module!
// {
//   #[ allow( unused_imports ) ]
//   use wtools::meta::*;
//   #[ allow( unused_imports ) ]
//   use wtools::former::Former;
// }
//
// only_for_terminal_module!
// {
//   #[ allow( unused_imports ) ]
//   use meta_tools::*;
//   #[ allow( unused_imports ) ]
//   use former::Former;
// }

#[derive( Debug, PartialEq, TheModule::Former )]
pub struct Struct1< 'a >
{
  pub string_slice_1 : &'a str,
}

//

include!( "./string_slice_only_test.rs" );