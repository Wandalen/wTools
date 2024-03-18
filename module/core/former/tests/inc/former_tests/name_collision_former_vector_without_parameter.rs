use super::*;
use the_module::Former;

#[ derive( Debug, PartialEq ) ]
struct Vec
{
  f1 : i32,
}

#[ derive( Debug, PartialEq, Former ) ]
pub struct Struct1
{
  f2 : Vec<>,
}

tests_impls!
{

  // Name conflict is not a problem.
  fn basic()
  {

    let got = Struct1::former().f2( Vec { f1 : 3 } ).form();
    let expected = Struct1 { f2 : Vec { f1 : 3 } };
    a_id!( got, expected );

  }

}

//

tests_index!
{
  basic,
}
