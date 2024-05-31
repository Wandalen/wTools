#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use former::Assign;


#[ derive( Default, PartialEq, Debug, former::Assign ) ]
// #[ debug ]
struct Person
{
  age : i32,
  name : String,
}

//

include!( "./only_test/component_assign.rs" );
