#[ allow( unused_imports ) ]
use super::*;

///
/// Options1
///

#[ derive( Debug, Default, PartialEq, TheModule::ComponentFrom ) ]
// #[ debug ]
// xxx : finish with debug, add test and sample
pub struct Options1
{
  field1 : i32,
  field2 : String,
  field3 : f32,
}

//

include!( "only_test/components_from.rs" );
