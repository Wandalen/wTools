use super::*;

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
  let got : the_module::VecDeque< i32 > = the_module::deque!{};
  let exp = the_module::VecDeque::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::deque!{ 3, 13 };
  let mut exp = the_module::VecDeque::new();
  exp.push_front( 13 );
  exp.push_front( 3 );
  assert_eq!( got, exp );

  let _got = the_module::deque!( "b" );
  let _got = the_module::exposed::deque!( "b" );

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

  let _got = the_module::deque!( "b" );
  let _got = the_module::exposed::deque!( "b" );

}

#[ test ]
fn iters()
{
  struct MyContainer
  {
    entries : the_module::VecDeque< i32 >,
  }

  impl IntoIterator for MyContainer
  {
    type Item = i32;
    type IntoIter = the_module::deque::IntoIter< i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.into_iter()
    }
  }

  impl< 'a > IntoIterator for &'a MyContainer
  {
    type Item = &'a i32;
    type IntoIter = the_module::deque::Iter< 'a, i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.iter()
    }
  }

  impl< 'a > IntoIterator for &'a mut MyContainer
  {
    type Item = &'a mut i32;
    type IntoIter = the_module::deque::IterMut< 'a, i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.iter_mut()
    }
  }

  let instance = MyContainer { entries : the_module::VecDeque::from( [ 1, 2, 3 ] ) };
  let got : the_module::VecDeque< _ > = instance.into_iter().collect();
  let exp = the_module::VecDeque::from( [ 1, 2, 3 ] );
  a_id!( got, exp );

  let instance = MyContainer { entries : the_module::VecDeque::from( [ 1, 2, 3 ] ) };
  let got : the_module::VecDeque< _ > = ( &instance ).into_iter().cloned().collect();
  let exp = the_module::VecDeque::from( [ 1, 2, 3 ] );
  a_id!( got, exp );

  let mut instance = MyContainer { entries : the_module::VecDeque::from( [ 1, 2, 3 ] ) };
  ( &mut instance ).into_iter().for_each( | v | *v *= 2 );
  let exp = the_module::VecDeque::from( [ 2, 4, 6 ] );
  a_id!( instance.entries, exp );

}
