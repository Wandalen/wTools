use super::*;

#[ cfg( not( feature = "no_std" ) ) ]
#[ test ]
fn reexport()
{

  let mut map : the_module::BinaryHeap< i32 > = the_module::BinaryHeap::new();
  map.push( 1 );
  let exp = Some(1).as_ref();
  let got = map.peek();
  assert_eq!( exp, got );

}

#[ test ]
#[ cfg( feature = "collection_constructors" ) ]
fn constructor()
{

  // test.case( "empty" );
  let got : the_module::BinaryHeap< i32 > = the_module::heap!{};
  let exp: the_module::BinaryHeap< i32 > = the_module::BinaryHeap::new();
  assert_eq!( got.into_vec(), exp.into_vec() );

  // test.case( "multiple entry" );
  let got = the_module::heap!{ 3, 13 };
  let mut exp = the_module::BinaryHeap::new();
  exp.push(3);
  exp.push(13);
  assert_eq!( got.into_sorted_vec(), exp.into_sorted_vec() );

}

#[ test ]
#[ cfg( feature = "collection_into_constructors" ) ]
fn into_constructor()
{

  // test.case( "empty" );
  let got : the_module::BinaryHeap< i32 > = the_module::into_heap!{};
  let exp = the_module::BinaryHeap::< i32 >::new();
  assert_eq!( got.into_vec(), exp.into_vec() );

  // test.case( "multiple entry" );
  let got : the_module::BinaryHeap< i32 > = the_module::into_heap!{ 3, 13 };
  let mut exp = the_module::BinaryHeap::new();
  exp.push(3);
  exp.push(13);
  assert_eq!( got.into_sorted_vec(), exp.into_sorted_vec() );

}
