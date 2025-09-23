#[ allow( unused_imports ) ]
use super :: *;

use the_module ::
{
  Fields,
  OptionalCow,
};

use std ::
{
  borrow ::Cow,
};

use collection_tools ::HashMap as Hmap;

#[ test ]
fn vec_string_fields()
{
  let mut collection = Hmap :: < usize, String > ::new();
  collection.insert( 1_usize, "a".to_string() );
  collection.insert( 2_usize, "b".to_string() );

  // k, v
  let got: Hmap< _, _ > = Fields :: < usize, &str > ::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let mut exp = Hmap ::new();
  exp.insert( &1, "a" );
  exp.insert( &2, "b" );
  assert_eq!( got, exp );

  // k, Option< Cow< '_, str > >
  let got: Hmap< _, _ > = Fields :: < usize, Option< Cow< '_, str > > > ::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let mut exp = Hmap ::new();
  exp.insert( &1, Some( Cow ::Borrowed( "a" ) ) );
  exp.insert( &2, Some( Cow ::Borrowed( "b" ) ) );
  assert_eq!( got, exp );

  // k, OptionalCow< '_, str, () >
  let got: Hmap< _, _ > = Fields :: < usize, OptionalCow< '_, str, () > > ::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let mut exp = Hmap ::new();
  exp.insert( &1, OptionalCow ::from( "a" ) );
  exp.insert( &2, OptionalCow ::from( "b" ) );
  assert_eq!( got, exp );

}

#[ test ]
fn vec_str_fields()
{
  let mut collection = Hmap :: < usize, String > ::new();
  collection.insert( 1_usize, "a".to_string() );
  collection.insert( 2_usize, "b".to_string() );

  // k, v
  let got: Hmap< _, _ > = Fields :: < usize, &str > ::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let mut exp = Hmap ::new();
  exp.insert( &1, "a" );
  exp.insert( &2, "b" );
  assert_eq!( got, exp );

  // k, Option< Cow< '_, str > >
  let got: Hmap< _, _ > = Fields :: < usize, Option< Cow< '_, str > > > ::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let mut exp = Hmap ::new();
  exp.insert( &1, Some( Cow ::Borrowed( "a" ) ) );
  exp.insert( &2, Some( Cow ::Borrowed( "b" ) ) );
  assert_eq!( got, exp );

  // k, OptionalCow< '_, str, () >
  let got: Hmap< _, _ > = Fields :: < usize, OptionalCow< '_, str, () > > ::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let mut exp = Hmap ::new();
  exp.insert( &1, OptionalCow ::from( "a" ) );
  exp.insert( &2, OptionalCow ::from( "b" ) );
  assert_eq!( got, exp );

}
