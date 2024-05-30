#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use former::{ Assign, AssignWithType };

///
/// Options1
///

#[
  derive
  (
    Debug,
    Default,
    PartialEq,
    the_module::ComponentFrom,
    the_module::Assign,
    the_module::ComponentsAssign,
    the_module::FromComponents,
  )
]
// qqq : make these traits working for generic struct, use `split_for_impl`
pub struct Options1
{
  field1 : i32,
  field2 : String,
  field3 : f32,
}

///
/// Options2
///

#[
  derive
  (
    Debug,
    Default,
    PartialEq,
    the_module::ComponentFrom,
    the_module::Assign,
    the_module::ComponentsAssign,
    the_module::FromComponents,
  )
]
pub struct Options2
{
  field1 : i32,
  field2 : String,
}

//

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

include!( "./only_test/composite.rs" );
