#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use former::ComponentSet;


#[ derive( Default, PartialEq, Debug, former::ComponentSet ) ]
#[ debug ]
struct Person
{
  age : i32,
  name : String,
}

//

include!( "only_test/components_set_component.rs" );