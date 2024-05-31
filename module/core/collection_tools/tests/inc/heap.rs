use super::*;

#[ test ]
fn reexport()
{

  let mut map : the_module::BinaryHeap< i32 > = the_module::BinaryHeap::new();
  map.push( 1 );
  let exp = Some(1).as_ref();
  let got = map.peek();
  assert_eq!( exp, got );

}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
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

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
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

#[ test ]
fn iters()
{

  struct MyContainer
  {
    entries : the_module::BinaryHeap< i32 >,
  }

  impl IntoIterator for MyContainer
  {
    type Item = i32;
    type IntoIter = the_module::heap::IntoIter< i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.into_iter()
    }
  }

  impl< 'a > IntoIterator for &'a MyContainer
  {
    type Item = &'a i32;
    type IntoIter = the_module::heap::Iter< 'a, i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.iter()
    }
  }

  let instance = MyContainer { entries : the_module::BinaryHeap::from( [ 1, 2, 3 ] ) };
  let got : the_module::BinaryHeap< i32 > = instance.into_iter().collect();
  let exp : the_module::BinaryHeap< i32 > = the_module::BinaryHeap::from( [ 1, 2, 3 ] );
  a_id!( got.into_sorted_vec(), exp.into_sorted_vec() );

  let instance = MyContainer { entries : the_module::BinaryHeap::from( [ 1, 2, 3 ] ) };
  let got : the_module::BinaryHeap< i32 > = ( &instance ).into_iter().cloned().collect();
  let exp : the_module::BinaryHeap< i32 > = the_module::BinaryHeap::from( [ 1, 2, 3 ] );
  a_id!( got.into_sorted_vec(), exp.into_sorted_vec() );

}
