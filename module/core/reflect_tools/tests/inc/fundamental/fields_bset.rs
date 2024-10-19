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
fn bset_string_fields()
{
  let collection : BTreeSet< String >  = bset!
  [
    "a".to_string(),
    "b".to_string(),
  ];

  // k, v
  let got : BTreeSet< _ > = Fields::< usize, &str >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = bset![ ( 0, "a" ), ( 1, "b" ) ];
  assert_eq!( got, exp );

  // k, Option< Cow< '_, str > >
  let got : BTreeSet< _ > = Fields::< usize, Option< Cow< '_, str > > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = bset![ ( 0, Some( Cow::Borrowed( "a" ) ) ), ( 1, Some( Cow::Borrowed( "b" ) ) ) ];
  assert_eq!( got, exp );

}

#[ test ]
fn bset_str_fields()
{
  let collection : BTreeSet< &str > = bset!
  [
    "a",
    "b",
  ];

  // k, v
  let got : BTreeSet< _ > = Fields::< usize, &str >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = bset![ ( 0, "a" ), ( 1, "b" ) ];
  assert_eq!( got, exp );

  // k, Option< Cow< '_, str > >
  let got : BTreeSet< _ > = Fields::< usize, Option< Cow< '_, str > > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = bset![ ( 0, Some( Cow::Borrowed( "a" ) ) ), ( 1, Some( Cow::Borrowed( "b" ) ) ) ];
  assert_eq!( got, exp );

}
