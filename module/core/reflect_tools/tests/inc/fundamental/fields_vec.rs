#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  Fields,
  OptionalCow,
};

// xxx : implement for other collections

use std::
{
  borrow::Cow,
};

#[ test ]
fn vec_string_fields()
{
  let collection = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];

  // k, v
  let got : Vec< _ > = Fields::< usize, &str >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = vec![ ( 0, "a" ), ( 1, "b" ) ];
  assert_eq!( got, exp );

  // k, Option< Cow< '_, str > >
  let got : Vec< _ > = Fields::< usize, Option< Cow< '_, str > > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = vec![ ( 0, Some( Cow::Borrowed( "a" ) ) ), ( 1, Some( Cow::Borrowed( "b" ) ) ) ];
  assert_eq!( got, exp );

  // k, OptionalCow< '_, str, () >
  let got : Vec< _ > = Fields::< usize, OptionalCow< '_, str, () > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = vec![ ( 0, OptionalCow::from( "a" ) ), ( 1, OptionalCow::from( "b" ) ) ];
  assert_eq!( got, exp );

}

#[ test ]
fn vec_str_fields()
{
  let collection = vec!
  [
    "a",
    "b",
  ];

  // k, v
  let got : Vec< _ > = Fields::< usize, &str >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = vec![ ( 0, "a" ), ( 1, "b" ) ];
  assert_eq!( got, exp );

  // k, Option< Cow< '_, str > >
  let got : Vec< _ > = Fields::< usize, Option< Cow< '_, str > > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = vec![ ( 0, Some( Cow::Borrowed( "a" ) ) ), ( 1, Some( Cow::Borrowed( "b" ) ) ) ];
  assert_eq!( got, exp );

  // k, OptionalCow< '_, str, () >
  let got : Vec< _ > = Fields::< usize, OptionalCow< '_, str, () > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = vec![ ( 0, OptionalCow::from( "a" ) ), ( 1, OptionalCow::from( "b" ) ) ];
  assert_eq!( got, exp );

}
