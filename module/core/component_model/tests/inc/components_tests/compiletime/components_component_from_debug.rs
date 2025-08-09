#[ allow( unused_imports ) ]
use super::*;

///
/// Options1
///
#[ derive( Debug, Default, PartialEq, the_module::ComponentFrom ) ]
// #[ debug ]  // Disabled - this file doesn't actually test debug functionality
// zzz : enable the test
pub struct Options1
{
  field1 : i32,
  field2 : String,
  field3 : f32,
}

//
