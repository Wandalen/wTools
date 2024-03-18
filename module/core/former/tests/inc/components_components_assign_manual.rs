#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use former::{ ComponentAssign, AssignWithType };

///
/// Options1
///

#[ derive( Debug, Default, PartialEq ) ]
pub struct Options1
{
  field1 : i32,
  field2 : String,
  field3 : f32,
}

impl From< &Options1 > for i32
{
  #[ inline( always ) ]
  fn from( src : &Options1 ) -> Self
  {
    src.field1.clone()
  }
}

impl From< &Options1 > for String
{
  #[ inline( always ) ]
  fn from( src : &Options1 ) -> Self
  {
    src.field2.clone()
  }
}

impl From< &Options1 > for f32
{
  #[ inline( always ) ]
  fn from( src : &Options1 ) -> Self
  {
    src.field3.clone()
  }
}

impl< IntoT > former::ComponentAssign< i32, IntoT > for Options1
where
  IntoT : Into< i32 >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.field1 = component.into().clone();
  }
}

impl< IntoT > former::ComponentAssign< String, IntoT > for Options1
where
  IntoT : Into< String >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.field2 = component.into().clone();
  }
}

impl< IntoT > former::ComponentAssign< f32, IntoT > for Options1
where
  IntoT : Into< f32 >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.field3 = component.into().clone();
  }
}

///
/// Options1ComponentsAssign.
///

// #[ allow( dead_code ) ]
pub trait Options1ComponentsAssign< IntoT >
where
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Into< f32 >,
  IntoT : Clone,
{
  fn options_1_assign( &mut self, component : IntoT );
}

// #[ allow( dead_code ) ]
impl< T, IntoT > Options1ComponentsAssign< IntoT > for T
where
  T : former::ComponentAssign< i32, IntoT >,
  T : former::ComponentAssign< String, IntoT >,
  T : former::ComponentAssign< f32, IntoT >,
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Into< f32 >,
  IntoT : Clone,
{
  #[ inline( always ) ]
  fn options_1_assign( &mut self, component : IntoT )
  {
    former::ComponentAssign::< i32, _ >::assign( self, component.clone() );
    former::ComponentAssign::< String, _ >::assign( self, component.clone() );
    former::ComponentAssign::< f32, _ >::assign( self, component.clone() );
  }
}

///
/// Options2
///

#[ derive( Debug, Default, PartialEq ) ]
pub struct Options2
{
  field1 : i32,
  field2 : String,
}

impl From< &Options2 > for i32
{
  #[ inline( always ) ]
  fn from( src : &Options2 ) -> Self
  {
    src.field1.clone()
  }
}

impl From< &Options2 > for String
{
  #[ inline( always ) ]
  fn from( src : &Options2 ) -> Self
  {
    src.field2.clone()
  }
}

impl< IntoT > former::ComponentAssign< i32, IntoT > for Options2
where
  IntoT : Into< i32 >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.field1 = component.into().clone();
  }
}

impl< IntoT > former::ComponentAssign< String, IntoT > for Options2
where
  IntoT : Into< String >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.field2 = component.into().clone();
  }
}

///
/// Options2ComponentsAssign.
///

pub trait Options2ComponentsAssign< IntoT >
where
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Clone,
{
  fn options_2_assign( &mut self, component : IntoT );
}

impl< T, IntoT > Options2ComponentsAssign< IntoT > for T
where
  T : former::ComponentAssign< i32, IntoT >,
  T : former::ComponentAssign< String, IntoT >,
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Clone,
{
  #[ inline( always ) ]
  fn options_2_assign( &mut self, component : IntoT )
  {
    former::ComponentAssign::< i32, _ >::assign( self, component.clone() );
    former::ComponentAssign::< String, _ >::assign( self, component.clone() );
  }
}

//

include!( "only_test/components_components_assign.rs" );
