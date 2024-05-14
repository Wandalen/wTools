use super::*;

#[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
#[ test ]
fn reexport()
{

  let mut map : the_module::LinkedList< i32 > = the_module::LinkedList::new();
  map.push_back( 1 );
  assert_eq!( map.contains( &1 ), true );
  assert_eq!( map.contains( &2 ), false );

}

#[ test ]
#[ cfg( feature = "collection_constructors" ) ]
fn constructor()
{

  // test.case( "empty" );
  let got : the_module::LinkedList< i32 > = the_module::list!{};
  let exp = the_module::LinkedList::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::list!{ 13, 15 };
  let mut exp = the_module::LinkedList::new();
  exp.push_front( 15 );
  exp.push_front( 13 );
  assert_eq!( got, exp );

}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_constructor()
{

  // test.case( "empty" );
  let got : the_module::LinkedList< i32 > = the_module::into_list!{};
  let exp = the_module::LinkedList::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::into_list!{ 13, 15 };
  let mut exp = the_module::LinkedList::new();
  exp.push_front( 15 );
  exp.push_front( 13 );
  assert_eq!( got, exp );

}
