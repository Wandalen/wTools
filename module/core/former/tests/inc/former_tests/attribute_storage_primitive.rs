#[ allow( unused_imports ) ]
use super::*;
// xxx2 : implement

#[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
  #[ storage_fields( a : i32, b : Option< String > ) ]
pub struct Struct1
{
  // #[ former( only_storage = true ) ]
  pub a : i32,
  // #[ former( only_storage = true ) ]
  b : Option< String >,
}

//

tests_impls!
{
  fn test_complex()
  {
    // let got = Struct1::former().a( 13 ).b( "abc" ).form();
    // let exp = Struct1
    // {
    //   a : 13,
    //   b : Some( "abc".to_string() ),
    // };
    // a_id!( got, exp );
  }
}

// == begin of generated

// == end of generated

tests_index!
{
  test_complex,
}