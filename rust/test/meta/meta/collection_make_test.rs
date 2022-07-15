// #![ allow( dead_code ) ]
// #![ feature( trace_macros ) ]
use super::*;
// use test_tools::exposed::*;

//

tests_impls!
{
  #[ test ]
  fn hash_map()
  {

    // test.case( "empty" );
    let got : std::collections::HashMap< i32, i32 > = TheModule::hmap!{};
    let exp = std::collections::HashMap::new();
    a_id!( got, exp );

    // test.case( "single entry" );
    let got = TheModule::hmap!{ 3 => 13 };
    let mut exp = std::collections::HashMap::new();
    exp.insert( 3, 13 );
    a_id!( got, exp );

  }

  //

  #[ test ]
  fn hash_set()
  {

    // test.case( "empty" );
    let got : std::collections::HashSet< i32 > = TheModule::hset!{};
    let exp = std::collections::HashSet::new();
    a_id!( got, exp );

    // test.case( "single entry" );
    let got = TheModule::hset!{ 13 };
    let mut exp = std::collections::HashSet::new();
    exp.insert( 13 );
    a_id!( got, exp );

  }
}

//

tests_index!
{
  hash_map,
  hash_set,
}
