use super::*;

#[ test ]
#[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
fn reexport()
{

  let mut vec1 : the_module::Vec< i32 > = the_module::Vec::new();
  vec1.push( 1 );
  vec1.push( 2 );
  let got = vec1.first().unwrap().clone();
  assert_eq!( got, 1 );
  let got = vec1.last().unwrap().clone();
  assert_eq!( got, 2 );

  use std::vec::Vec as DynList;
  let mut vec2 : DynList< i32 > = DynList::new();
  vec2.push( 1 );
  vec2.push( 2 );
  let got = vec2.first().unwrap().clone();
  assert_eq!( got, 1 );
  let got = vec2.last().unwrap().clone();
  assert_eq!( got, 2 );

  assert_eq!( vec1, vec2 );

}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn constructor()
{

  // test.case( "empty" );
  let got : the_module::Vec< i32 > = the_module::vec!{};
  let exp = the_module::Vec::< i32 >::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::vec!{ 3, 13 };
  let mut exp = the_module::Vec::new();
  exp.push( 3 );
  exp.push( 13 );
  assert_eq!( got, exp );

  let _got = the_module::vec!( "b" );
  let _got = the_module::dlist!( "b" );
  let _got = the_module::exposed::dlist!( "b" );

}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_constructor()
{

  // test.case( "empty" );
  let got : the_module::Vec< i32 > = the_module::into_vec!{};
  let exp = the_module::Vec::< i32 >::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got : the_module::Vec< i32 > = the_module::into_vec!{ 3, 13 };
  let mut exp = the_module::Vec::new();
  exp.push( 3 );
  exp.push( 13 );
  assert_eq!( got, exp );

  let _got : Vec< &str > = the_module::into_vec!( "b" );
  let _got : Vec< &str > = the_module::exposed::into_vec!( "b" );
  let _got : Vec< &str > = the_module::into_dlist!( "b" );
  let _got : Vec< &str > = the_module::exposed::into_dlist!( "b" );

}

// qqq : implement similar test for all containers -- done
#[ test ]
fn iters()
{

  struct MyContainer
  {
    entries : Vec< i32 >,
  }

  impl IntoIterator for MyContainer
  {
    type Item = i32;
    type IntoIter = the_module::vector::IntoIter< i32 >;
    // qqq : should work -- works

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.into_iter()
    }
  }

  impl< 'a > IntoIterator for &'a MyContainer
  {
    type Item = &'a i32;
    type IntoIter = the_module::vector::Iter< 'a, i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.iter()
    }
  }

  impl< 'a > IntoIterator for &'a mut MyContainer
  {
    type Item = &'a mut i32;
    type IntoIter = the_module::vector::IterMut< 'a, i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.iter_mut()
    }
  }

  let instance = MyContainer { entries : the_module::Vec::from( [ 1, 2, 3 ] ) };
  let got : Vec< _ > = instance.into_iter().collect();
  let exp = the_module::Vec::from( [ 1, 2, 3 ] );
  a_id!( got, exp );

  let instance = MyContainer { entries : the_module::Vec::from( [ 1, 2, 3 ] ) };
  let got : Vec< _ > = ( &instance ).into_iter().cloned().collect();
  let exp = the_module::Vec::from( [ 1, 2, 3 ] );
  a_id!( got, exp );

  let mut instance = MyContainer { entries : the_module::Vec::from( [ 1, 2, 3 ] ) };
  ( &mut instance ).into_iter().for_each( | v | *v *= 2 );
  let exp = the_module::Vec::from( [ 2, 4, 6 ] );
  a_id!( instance.entries, exp );

}
