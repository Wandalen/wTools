#[ allow( unused_imports ) ]
use super::*;

//

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn vec()
{

  // test.case( "empty" );
  let got : the_module::Vec< i32 > = the_module::vec!{};
  let exp: the_module::Vec< i32 > = the_module::Vec::new();
  assert_eq!( got, exp );

  // test.case( "single entry" );
  let got = the_module::vec!{ 3, 13 };
  let mut exp = the_module::Vec::new();
  exp.push( 3 );
  exp.push( 13 );
  assert_eq!( got, exp );

}

//

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn hash_map()
{

  // test.case( "empty" );
  let got : the_module::HashMap< i32, i32 > = the_module::hmap!{};
  let exp = the_module::HashMap::new();
  assert_eq!( got, exp );


  // test.case( "single entry" );
  let got = the_module::hmap!{ 3 => 13 };
  let mut exp = the_module::HashMap::new();
  exp.insert( 3, 13 );
  assert_eq!( got, exp );

}

//

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn hash_set()
{

  // test.case( "empty" );
  let got : the_module::HashSet< i32 > = the_module::hset!{};
  let exp = the_module::HashSet::new();
  assert_eq!( got, exp );

  // test.case( "single entry" );
  let got = the_module::hset!{ 13 };
  let mut exp = the_module::HashSet::new();
  exp.insert( 13 );
  assert_eq!( got, exp );

}