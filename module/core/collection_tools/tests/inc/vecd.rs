use super::*;

#[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
#[ test ]
fn reexport()
{

  let mut map : the_module::VecDeque< i32 > = the_module::VecDeque::new();
  map.push_back( 1 );
  assert_eq!( map.contains( &1 ), true );
  assert_eq!( map.contains( &2 ), false );

}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn constructor()
{

  // test.case( "empty" );
  let got : the_module::VecDeque< i32 > = the_module::vecd!{};
  let exp = the_module::VecDeque::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::vecd!{ 3, 13 };
  let mut exp = the_module::VecDeque::new();
  exp.push_front( 13 );
  exp.push_front( 3 );
  assert_eq!( got, exp );

}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_constructor()
{

  // test.case( "empty" );
  let got : the_module::VecDeque< i32 > = the_module::into_vecd!{};
  let exp = the_module::VecDeque::new();
  assert_eq!( got, exp );

  // test.case( "single entry" );
  let got = the_module::into_vecd!{ 3, 13 };
  let mut exp = the_module::VecDeque::new();
  exp.push_front( 13 );
  exp.push_front( 3 );
  assert_eq!( got, exp );

}
