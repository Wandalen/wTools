use super::*;


#[ test ]
fn reexport()
{

  let mut map1 : the_module::HashMap< i32, i32 > = the_module::HashMap::new();
  map1.insert( 1, 2 );
  let exp = 2;
  let got = *map1.get( &1 ).unwrap();
  assert_eq!( exp, got );

  let mut map2 : the_module::Map< i32, i32 > = the_module::Map::new();
  map2.insert( 1, 2 );
  let exp = 2;
  let got = *map2.get( &1 ).unwrap();
  assert_eq!( exp, got );

  assert_eq!( map1, map2 );

}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
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

  let _got = the_module::hmap!( "a" => "b" );
  let _got = the_module::exposed::hmap!( "a" => "b" );

}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
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

  let _got : Hmap< &str, &str > = the_module::into_hmap!( "a" => "b" );
  let _got : Hmap< &str, &str > = the_module::exposed::into_hmap!( "a" => "b" );

}

#[ test ]
fn iters()
{

  struct MyContainer
  {
    entries : the_module::HashMap< i32, i32 >,
  }

  impl IntoIterator for MyContainer
  {
    type Item = ( i32, i32 );
    type IntoIter = the_module::hash_map::IntoIter< i32, i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.into_iter()
    }
  }

  impl< 'a > IntoIterator for &'a MyContainer
  {
    type Item = ( &'a i32, &'a i32 );
    type IntoIter = the_module::hash_map::Iter< 'a, i32, i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.iter()
    }
  }

  impl< 'a > IntoIterator for &'a mut MyContainer
  {
    type Item = ( &'a i32, &'a mut i32 );
    type IntoIter = the_module::hash_map::IterMut< 'a, i32, i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.iter_mut()
    }
  }

  let instance = MyContainer { entries : the_module::HashMap::from( [ ( 1 , 3 ), ( 2, 2 ), ( 3, 1 ) ] ) };
  let got : the_module::HashMap< _, _ > = instance.into_iter().collect();
  let exp = the_module::HashMap::from( [ ( 1 , 3 ), ( 2, 2 ), ( 3, 1 ) ] );
  a_id!( got, exp );

  let instance = MyContainer { entries : the_module::HashMap::from( [ ( 1 , 3 ), ( 2, 2 ), ( 3, 1 ) ] ) };
  let got : the_module::HashMap< _, _ > = ( &instance ).into_iter().map( | ( k, v ) | ( k.clone(), v.clone() ) ).collect();
  let exp = the_module::HashMap::from( [ ( 1 , 3 ), ( 2, 2 ), ( 3, 1 ) ] );
  a_id!( got, exp );

  let mut instance = MyContainer { entries : the_module::HashMap::from( [ ( 1 , 3 ), ( 2, 2 ), ( 3, 1 ) ] ) };
  ( &mut instance ).into_iter().for_each( | ( _, v ) | *v *= 2 );
  let exp = the_module::HashMap::from( [ ( 1, 6 ), ( 2 ,4 ), ( 3, 2 ) ] );
  a_id!( instance.entries, exp );

}
