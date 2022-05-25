// #![ allow( dead_code ) ]
// #![ feature( trace_macros ) ]
use super::TheModule;
use test_tools::*;

//

tests_impls!
{
  #[ test ]
  fn hash_map()
  {

    // test.case( "empty" );
    let got : std::collections::HashMap< i32, i32 > = TheModule::hmap!{};
    let exp = std::collections::HashMap::new();
    assert_eq!( got, exp );

    // test.case( "single entry" );
    let got = TheModule::hmap!{ 3 => 13 };
    let mut exp = std::collections::HashMap::new();
    exp.insert( 3, 13 );
    assert_eq!( got, exp );

  }

  //

  #[ test ]
  fn hash_set()
  {

    // test.case( "empty" );
    let got : std::collections::HashSet< i32 > = TheModule::hset!{};
    let exp = std::collections::HashSet::new();
    assert_eq!( got, exp );

    // test.case( "single entry" );
    let got = TheModule::hset!{ 13 };
    let mut exp = std::collections::HashSet::new();
    exp.insert( 13 );
    assert_eq!( got, exp );

  }
}

//

tests_index!
{
  hash_map,
  hash_set,
}
