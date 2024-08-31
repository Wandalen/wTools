#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  Fields,
};

// xxx : implement for other collections

use std::
{
  borrow::Cow,
};

#[ test ]
fn hset_string_fields()
{
  let collection : HashSet< String > = hset!
  [
    "a".to_string(),
    "b".to_string(),
  ];

  // k, v
  let got : HashSet< _ > = Fields::< usize, &str >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  assert!( got.contains(&( 0, "a" ) ) || got.contains(&( 1, "a" ) ) );
  assert!( got.contains(&( 0, "b" ) ) || got.contains(&( 1, "b" ) ) );

  // k, Option< Cow< '_, str > >
  let got : HashSet< _ > = Fields::< usize, Option< Cow< '_, str > > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  assert!( got.contains(&( 0, Some( Cow::Borrowed( "a" ) ) ) ) || got.contains(&( 1, Some( Cow::Borrowed( "a" ) ) ) ) );
  assert!( got.contains(&( 0, Some( Cow::Borrowed( "b" ) ) ) ) || got.contains(&( 1, Some( Cow::Borrowed( "b" ) ) ) ) );

}

#[ test ]
fn hset_str_fields()
{
  let collection : HashSet< &str > = hset!
  [
    "a",
    "b",
  ];

  // k, v
  let got : HashSet< _ > = Fields::< usize, &str >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  assert!( got.contains(&( 0, "a" ) ) || got.contains(&( 1, "a" ) ) );
  assert!( got.contains(&( 0, "b" ) ) || got.contains(&( 1, "b" ) ) );

  // k, Option< Cow< '_, str > >
  let got : HashSet< _ > = Fields::< usize, Option< Cow< '_, str > > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  assert!( got.contains(&( 0, Some( Cow::Borrowed( "a" ) ) ) ) || got.contains(&( 1, Some( Cow::Borrowed( "a" ) ) ) ) );
  assert!( got.contains(&( 0, Some( Cow::Borrowed( "b" ) ) ) ) || got.contains(&( 1, Some( Cow::Borrowed( "b" ) ) ) ) );

}
