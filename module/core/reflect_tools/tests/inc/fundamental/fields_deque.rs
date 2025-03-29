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
fn deque_string_fields()
{
  let collection = deque!
  [
    "a".to_string(),
    "b".to_string(),
  ];

  // k, v
  let got : VecDeque< _ > = Fields::< usize, &str >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = deque![ ( 0, "a" ), ( 1, "b" ) ];
  assert_eq!( got, exp );

  // k, Option< Cow< '_, str > >
  let got : VecDeque< _ > = Fields::< usize, Option< Cow< '_, str > > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = deque![ ( 0, Some( Cow::Borrowed( "a" ) ) ), ( 1, Some( Cow::Borrowed( "b" ) ) ) ];
  assert_eq!( got, exp );

  // k, OptionalCow< '_, str, () >
  let got : VecDeque< _ > = Fields::< usize, OptionalCow< '_, str, () > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = deque![ ( 0, OptionalCow::from( "a" ) ), ( 1, OptionalCow::from( "b" ) ) ];
  assert_eq!( got, exp );

}

#[ test ]
fn deque_str_fields()
{
  let collection = deque!
  [
    "a",
    "b",
  ];

  // k, v
  let got : VecDeque< _ > = Fields::< usize, &str >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = deque![ ( 0, "a" ), ( 1, "b" ) ];
  assert_eq!( got, exp );

  // k, Option< Cow< '_, str > >
  let got : VecDeque< _ > = Fields::< usize, Option< Cow< '_, str > > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = deque![ ( 0, Some( Cow::Borrowed( "a" ) ) ), ( 1, Some( Cow::Borrowed( "b" ) ) ) ];
  assert_eq!( got, exp );

  // k, OptionalCow< '_, str, () >
  let got : VecDeque< _ > = Fields::< usize, OptionalCow< '_, str, () > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = deque![ ( 0, OptionalCow::from( "a" ) ), ( 1, OptionalCow::from( "b" ) ) ];
  assert_eq!( got, exp );

}
