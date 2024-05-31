#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use former::{ Assign, AssignWithType };

///
/// Options1
///

#[ derive( Debug, Default, PartialEq, the_module::Assign, the_module::ComponentsAssign ) ]
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

///
/// Options2
///

#[ derive( Debug, Default, PartialEq, the_module::Assign, the_module::ComponentsAssign ) ]
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

//

include!( "./only_test/components_assign.rs" );
