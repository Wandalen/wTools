use super::*;

#[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
#[ test ]
fn reexport()
{

  let mut set1 : the_module::HashSet< i32 > = the_module::HashSet::new();
  set1.insert( 1 );
  assert_eq!( set1.contains( &1 ), true );
  assert_eq!( set1.contains( &2 ), false );

  let mut set2 : the_module::Set< i32 > = the_module::Set::new();
  set2.insert( 1 );
  assert_eq!( set2.contains( &1 ), true );
  assert_eq!( set2.contains( &2 ), false );

  assert_eq!( set1, set2 );

}

#[ test ]
#[ cfg( feature = "collection_constructors" ) ]
fn constructor()
{

  // test.case( "empty" );
  let got : the_module::HashSet< i32 > = the_module::hset!{};
  let exp = the_module::HashSet::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::hset!{ 13, 11 };
  let mut exp = the_module::HashSet::new();
  exp.insert( 11 );
  exp.insert( 13 );
  assert_eq!( got, exp );

}

#[ test ]
#[ cfg( feature = "collection_into_constructors" ) ]
fn into_constructor()
{

  // test.case( "empty" );
  let got : the_module::HashSet< i32 > = the_module::into_hset!{};
  let exp = the_module::HashSet::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::into_hset!{ 13, 11 };
  let mut exp = the_module::HashSet::new();
  exp.insert( 11 );
  exp.insert( 13 );
  assert_eq!( got, exp );

}
