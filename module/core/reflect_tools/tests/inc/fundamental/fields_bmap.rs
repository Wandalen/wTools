#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  Fields,
  OptionalCow,
};

use std::
{
  borrow::Cow,
};

#[ test ]
fn vec_string_fields()
{
  let collection : Bmap< usize, String > = into_bmap!
  [
    1 as usize => "a".to_string(),
    2 as usize => "b".to_string(),
  ];

  // k, v
  let got : Bmap< _, _ > = Fields::< usize, &str >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = into_bmap![ &1 => "a", &2 => "b" ];
  assert_eq!( got, exp );

  // k, Option< Cow< '_, str > >
  let got : Bmap< _, _ > = Fields::< usize, Option< Cow< '_, str > > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = into_bmap![ &1 => Some( Cow::Borrowed( "a" ) ), &2 => Some( Cow::Borrowed( "b" ) ) ];
  assert_eq!( got, exp );

  // k, OptionalCow< '_, str, () >
  let got : Bmap< _, _ > = Fields::< usize, OptionalCow< '_, str, () > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = into_bmap![ &1 => OptionalCow::from( "a" ), &2 => OptionalCow::from( "b" ) ];
  assert_eq!( got, exp );

}

#[ test ]
fn vec_str_fields()
{
  let collection : Bmap< usize, String > = into_bmap!
  [
    1 as usize => "a",
    2 as usize => "b",
  ];

  // k, v
  let got : Bmap< _, _ > = Fields::< usize, &str >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = into_bmap![ &1 => "a", &2 => "b" ];
  assert_eq!( got, exp );

  // k, Option< Cow< '_, str > >
  let got : Bmap< _, _ > = Fields::< usize, Option< Cow< '_, str > > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = into_bmap![ &1 => Some( Cow::Borrowed( "a" ) ), &2 => Some( Cow::Borrowed( "b" ) ) ];
  assert_eq!( got, exp );

  // k, OptionalCow< '_, str, () >
  let got : Bmap< _, _ > = Fields::< usize, OptionalCow< '_, str, () > >::fields( &collection ).collect();
  assert_eq!( got.len(), 2 );
  let exp = into_bmap![ &1 => OptionalCow::from( "a" ), &2 => OptionalCow::from( "b" ) ];
  assert_eq!( got, exp );

}
