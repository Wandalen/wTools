use super::*;

#[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
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

#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
#[ test ]
fn iters()
{

  struct MyContainer
  {
    entries : the_module::BTreeSet< i32 >,
  }

  impl IntoIterator for MyContainer
  {
    type Item = i32;
    type IntoIter = the_module::bset::IntoIter< i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.into_iter()
    }
  }

  impl< 'a > IntoIterator for &'a MyContainer
  {
    type Item = &'a i32;
    type IntoIter = the_module::bset::Iter< 'a, i32 >;

    fn into_iter( self ) -> Self::IntoIter
    {
      self.entries.iter()
    }
  }

  let instance = MyContainer { entries : the_module::BTreeSet::from( [ 1, 2, 3 ] ) };
  let got : the_module::BTreeSet< _ > = instance.into_iter().collect();
  let exp = the_module::BTreeSet::from( [ 1, 2, 3 ] );
  a_id!( got, exp );

  let instance = MyContainer { entries : the_module::BTreeSet::from( [ 1, 2, 3 ] ) };
  let got : the_module::BTreeSet< _ > = ( &instance ).into_iter().cloned().collect();
  let exp = the_module::BTreeSet::from( [ 1, 2, 3 ] );
  a_id!( got, exp );

}
