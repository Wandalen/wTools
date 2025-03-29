use super::*;

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

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
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

  let _got = the_module::hset!( "b" );
  let _got = the_module::exposed::hset!( "b" );

}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
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

  let _got : Hset< &str > = the_module::into_hset!( "b" );
  let _got : Hset< &str > = the_module::exposed::into_hset!( "b" );

}

#[ test ]
fn iters()
{

  struct MyContainer
  {
    entries : the_module::HashSet< i32 >,
  }

  impl IntoIterator for MyContainer
  {
    type Item = i32;
    type IntoIter = the_module::hash_set::IntoIter< i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.into_iter()
    }
  }

  impl< 'a > IntoIterator for &'a MyContainer
  {
    type Item = &'a i32;
    type IntoIter = the_module::hash_set::Iter< 'a, i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.iter()
    }
  }

  let instance = MyContainer { entries : the_module::HashSet::from( [ 1, 2, 3 ] ) };
  let got : the_module::HashSet< _ > = instance.into_iter().collect();
  let exp = the_module::HashSet::from( [ 1, 2, 3 ] );
  a_id!( got, exp );

  let instance = MyContainer { entries : the_module::HashSet::from( [ 1, 2, 3 ] ) };
  let got : the_module::HashSet< _ > = ( &instance ).into_iter().cloned().collect();
  let exp = the_module::HashSet::from( [ 1, 2, 3 ] );
  a_id!( got, exp );

}
