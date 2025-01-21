use super::*;

#[ test ]
fn reexport()
{

  let mut map : the_module::BTreeMap< i32, i32 > = the_module::BTreeMap::new();
  map.insert( 1, 2 );
  let exp = 2;
  let got = *map.get( &1 ).unwrap();
  assert_eq!( exp, got );

}

#[ cfg( feature = "collection_constructors" ) ]
#[ test ]
fn constructor()
{

  // test.case( "empty" );
  let got : the_module::BTreeMap< i32, i32 > = the_module::bmap!{};
  let exp = the_module::BTreeMap::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::bmap!{ 3 => 13, 4 => 1 };
  let mut exp = the_module::BTreeMap::new();
  exp.insert(3, 13);
  exp.insert(4, 1);
  assert_eq!( got, exp );

  let _got = the_module::bmap!( "a" => "b" );
  let _got = the_module::exposed::bmap!( "a" => "b" );

}

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn into_constructor()
{

  // test.case( "empty" );
  let got : the_module::BTreeMap< i32, i32 > = the_module::into_bmap!{};
  let exp = the_module::BTreeMap::new();
  assert_eq!( got, exp );

  // test.case( "multiple entry" );
  let got = the_module::into_bmap!{ 3 => 13, 4 => 1 };
  let mut exp = the_module::BTreeMap::new();
  exp.insert(3, 13);
  exp.insert(4, 1);
  assert_eq!( got, exp );

  let _got : Bmap< &str, &str > = the_module::into_bmap!( "a" => "b" );
  let _got : Bmap< &str, &str > = the_module::exposed::into_bmap!( "a" => "b" );

}

#[ test ]
fn iters()
{

  struct MyContainer
  {
    entries : the_module::BTreeMap< i32, i32 >,
  }

  impl IntoIterator for MyContainer
  {
    type Item = ( i32, i32 );
    type IntoIter = the_module::btree_map::IntoIter< i32, i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.into_iter()
    }
  }

  impl< 'a > IntoIterator for &'a MyContainer
  {
    type Item = ( &'a i32, &'a i32 );
    type IntoIter = the_module::btree_map::Iter< 'a, i32, i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.iter()
    }
  }

  let instance = MyContainer { entries : the_module::BTreeMap::from( [ ( 1, 3 ), ( 2, 2 ), ( 3, 1 ) ] ) };
  let got : the_module::BTreeMap< _, _ > = instance.into_iter().collect();
  let exp = the_module::BTreeMap::from( [ ( 1, 3 ), ( 2, 2 ), ( 3, 1 ) ] );
  a_id!( got, exp );

  let instance = MyContainer { entries : the_module::BTreeMap::from( [ ( 1, 3 ), ( 2, 2 ), ( 3, 1 ) ] ) };
  let got : the_module::BTreeMap< _, _ > = ( &instance ).into_iter().map( | ( k, v ) | ( k.clone(), v.clone() ) ).collect();
  let exp = the_module::BTreeMap::from( [ ( 1, 3 ), ( 2, 2 ), ( 3, 1 ) ] );
  a_id!( got, exp );

}
