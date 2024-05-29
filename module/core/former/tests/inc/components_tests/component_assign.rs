#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use former::ComponentAssign;


#[ derive( Default, PartialEq, Debug, former::ComponentAssign ) ]
// #[ debug ]
struct Person
{
  age : i32,
  name : String,
}

//

include!( "./only_test/component_assign.rs" );
