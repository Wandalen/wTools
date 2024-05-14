use super::*;

#[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
#[ test ]
fn reexport()
{

  let mut map : the_module::BTreeMap< i32, i32 > = the_module::BTreeMap::new();
  map.insert( 1, 2 );
  let exp = 2;
  let got = *map.get( &1 ).unwrap();
  assert_eq!( exp, got );

}

#[ test ]
#[ cfg( feature = "collection_constructors" ) ]
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

}
