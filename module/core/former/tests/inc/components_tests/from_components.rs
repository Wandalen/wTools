#[ allow( unused_imports ) ]
use super::*;

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

///
/// Options2
///

#[ derive( Debug, Default, PartialEq, the_module::FromComponents ) ]
pub struct Options2
{
  field1 : i32,
  field2 : String,
}

// impl< T > From< T > for Options2
// where
//   T : Into< i32 >,
//   T : Into< String >,
//   T : Clone,
// {
//   #[ inline( always ) ]
//   fn from( src : T ) -> Self
//   {
//     let field1 = Into::< i32 >::into( src.clone() );
//     let field2 = Into::< String >::into( src.clone() );
//     Options2
//     {
//       field1,
//       field2,
//     }
//   }
// }

//

include!( "./only_test/from_components.rs" );
