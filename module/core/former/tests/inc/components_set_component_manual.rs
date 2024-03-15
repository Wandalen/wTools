#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use former::ComponentSet;


#[ derive( Default, PartialEq, Debug ) ]
struct Person
{
  age : i32,
  name : String,
}

impl< IntoT > ComponentSet< i32, IntoT > for Person
where
  IntoT : Into< i32 >,
{
  fn set( &mut self, component : IntoT )
  {
    self.age = component.into();
  }
}

impl< IntoT > ComponentSet< String, IntoT > for Person
where
  IntoT : Into< String >,
{
  fn set( &mut self, component : IntoT )
  {
    self.name = component.into();
  }
}

//

include!( "only_test/components_set_component.rs" );
