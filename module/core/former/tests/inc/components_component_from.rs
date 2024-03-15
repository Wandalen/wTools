#[ allow( unused_imports ) ]
use super::*;

///
/// Options1
///

#[ derive( Debug, Default, PartialEq, TheModule::ComponentFrom ) ]
// #[ debug ]
pub struct Options1
{
  field1 : i32,
  field2 : String,
  field3 : f32,
}

//


include!( "only_test/components_component_from.rs" );
