#![ allow( dead_code ) ]

use super::*;
use the_module::Former;

pub mod core {}
pub mod std {}
pub mod marker {}
pub trait CloneAny{}
pub trait Context{}
pub trait Formed{}
pub trait OnEnd{}
pub struct None{}
pub struct Some{}

#[ derive( Debug, PartialEq ) ]
struct Vec
{
  f1 : i32,
}

#[ derive( Debug, PartialEq, Former ) ]
pub struct Struct1
{
  f2 : Vec<>,
  i : ::std::option::Option< i32 >,
}

tests_impls!
{

  // Name conflict is not a problem.
  fn basic()
  {

    let got = Struct1::former().f2( Vec { f1 : 3 } ).form();
    let expected = Struct1 { f2 : Vec { f1 : 3 }, i : ::std::option::Option::None };
    a_id!( got, expected );

  }

}

//

tests_index!
{
  basic,
}
