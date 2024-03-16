#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use former::ComponentAssign;


#[ derive( Default, PartialEq, Debug ) ]
struct Person
{
  age : i32,
  name : String,
}

impl< IntoT > ComponentAssign< i32, IntoT > for Person
where
  IntoT : Into< i32 >,
{
  fn assign( &mut self, component : IntoT )
  {
    self.age = component.into();
  }
}

impl< IntoT > ComponentAssign< String, IntoT > for Person
where
  IntoT : Into< String >,
{
  fn assign( &mut self, component : IntoT )
  {
    self.name = component.into();
  }
}

//

include!( "only_test/components_component_assign.rs" );
