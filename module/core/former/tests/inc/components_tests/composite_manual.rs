#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use former::{ Assign, AssignWithType };

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

impl< IntoT > former::Assign< i32, IntoT > for Options1
where
  IntoT : Into< i32 >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.field1 = component.into().clone();
  }
}

impl< IntoT > former::Assign< String, IntoT > for Options1
where
  IntoT : Into< String >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.field2 = component.into().clone();
  }
}

impl< IntoT > former::Assign< f32, IntoT > for Options1
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

pub trait Options1ComponentsAssign< IntoT >
where
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Into< f32 >,
  IntoT : Clone,
{
  fn options_1_assign( &mut self, component : IntoT );
}

impl< T, IntoT > Options1ComponentsAssign< IntoT > for T
where
  T : former::Assign< i32, IntoT >,
  T : former::Assign< String, IntoT >,
  T : former::Assign< f32, IntoT >,
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Into< f32 >,
  IntoT : Clone,
{
  #[ inline( always ) ]
  fn options_1_assign( &mut self, component : IntoT )
  {
    former::Assign::< i32, _ >::assign( self, component.clone() );
    former::Assign::< String, _ >::assign( self, component.clone() );
    former::Assign::< f32, _ >::assign( self, component.clone() );
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

impl< IntoT > former::Assign< i32, IntoT > for Options2
where
  IntoT : Into< i32 >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.field1 = component.into().clone();
  }
}

impl< IntoT > former::Assign< String, IntoT > for Options2
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
  T : former::Assign< i32, IntoT >,
  T : former::Assign< String, IntoT >,
  IntoT : Into< i32 >,
  IntoT : Into< String >,
  IntoT : Clone,
{
  #[ inline( always ) ]
  fn options_2_assign( &mut self, component : IntoT )
  {
    former::Assign::< i32, _ >::assign( self, component.clone() );
    former::Assign::< String, _ >::assign( self, component.clone() );
  }
}

impl< T > From< T > for Options2
where
  T : Into< i32 >,
  T : Into< String >,
  T : Clone,
{
  #[ inline( always ) ]
  fn from( src : T ) -> Self
  {
    let field1 = Into::< i32 >::into( src.clone() );
    let field2 = Into::< String >::into( src.clone() );
    Options2
    {
      field1,
      field2,
    }
  }
}

//

include!( "./only_test/composite.rs" );
