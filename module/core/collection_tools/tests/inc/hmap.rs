use super::*;

#[ cfg( not( feature = "no_std" ) ) ]
#[ test ]
fn reexport()
{

  let mut map1 : the_module::HashMap< i32, i32 > = the_module::HashMap::new();
  map1.insert( 1, 2 );
  let exp = 2;
  let got = *map1.get( &1 ).unwrap();
  assert_eq!( exp, got );

//   let mut map2 : the_module::Map< i32, i32 > = the_module::Map::new();
//   map2.insert( 1, 2 );
//   let exp = 2;
//   let got = *map2.get( &1 ).unwrap();
//   assert_eq!( exp, got );

//   assert_eq!( map1, map2 );

}

#[ test ]
#[ cfg( feature = "collection_constructors" ) ]
fn constructor()
{

  // test.case( "empty" );
  let got : the_module::HashMap< i32, i32 > = the_module::hmap!{};
  let exp = the_module::HashMap::new();
  assert_eq!( got, exp );


  // test.case( "multiple entry" );
  let got = the_module::hmap!{ 3 => 13, 4 => 1 };
  let mut exp = the_module::HashMap::new();
  exp.insert( 3, 13 );
  exp.insert( 4, 1 );
  assert_eq!( got, exp );

}

#[ test ]
#[ cfg( feature = "collection_into_constructors" ) ]
fn into_constructor()
{

  // test.case( "empty" );
  let got : the_module::HashMap< i32, i32 > = the_module::into_hmap!{};
  let exp = the_module::HashMap::new();
  assert_eq!( got, exp );


  // test.case( "multiple entry" );
  let got = the_module::into_hmap!{ 3 => 13, 4 => 1 };
  let mut exp = the_module::HashMap::new();
  exp.insert( 3, 13 );
  exp.insert( 4, 1 );
  assert_eq!( got, exp );

}
