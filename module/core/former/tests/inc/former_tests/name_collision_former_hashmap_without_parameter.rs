use super::*;
use the_module::Former;

pub mod core {}
pub mod std {}
pub mod marker {}
pub trait CloneAny{}
pub trait Context{}
pub trait Formed{}
pub trait OnEnd{}

#[ derive( Debug, PartialEq ) ]
struct HashMap< T >
{
  pub f1 : T,
}

#[ derive( Debug, PartialEq, Former ) ]
pub struct Struct1
{
  f2 : HashMap< i32 >,
}

tests_impls!
{

  // Name conflict is not a problem.
  fn basic()
  {

    let got = Struct1::former().f2( HashMap { f1 : 3 } ).form();
    let expected = Struct1 { f2 : HashMap { f1 : 3 } };
    a_id!( got, expected );

  }

}

//

tests_index!
{
  basic,
}
