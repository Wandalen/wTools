use super::*;
use TheModule::Former;

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
