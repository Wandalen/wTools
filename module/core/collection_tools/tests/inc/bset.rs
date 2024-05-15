use super::*;

#[ cfg( not( feature = "no_std" ) ) ]
#[ test ]
fn reexport()
{

  let mut map : the_module::BTreeSet< i32 > = the_module::BTreeSet::new();
  map.insert( 1 );
  assert_eq!( map.contains( &1 ), true );
  assert_eq!( map.contains( &2 ), false );

}

#[ test ]
#[ cfg( feature = "collection_constructors" ) ]
fn constructor()
{

  // test.case( "empty" );
  let got : the_module::BTreeSet< i32 > = the_module::bset!{};
  let exp = the_module::BTreeSet::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::bset!{ 3, 13 };
  let mut exp = the_module::BTreeSet::new();
  exp.insert(3);
  exp.insert(13);
  assert_eq!( got, exp );

}

#[ test ]
#[ cfg( feature = "collection_into_constructors" ) ]
fn into_constructor()
{

  // test.case( "empty" );
  let got : the_module::BTreeSet< i32 > = the_module::into_bset!{};
  let exp = the_module::BTreeSet::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::into_bset!{ 3, 13 };
  let mut exp = the_module::BTreeSet::new();
  exp.insert(3);
  exp.insert(13);
  assert_eq!( got, exp );

}
