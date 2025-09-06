#![ allow( clippy::no_effect_underscore_binding ) ]

#[ allow( unused_imports ) ]
use super::*;
use test_tools::{ a_id, a_true };

use the_module::
{
  ToStringWith,
  WithDebug,
  WithDisplay,
};

use std::
{
  borrow::Cow,
};

//

#[ test ]
fn to_string_with_test()
{

  // -

  let src = 13i32;
  let _got = ToStringWith::< WithDebug >::to_string_with( &src );
  let _exp = "13".to_string();
  a_id!( got, _exp );

  let src = "abc".to_string();
  let _got = ToStringWith::< WithDebug >::to_string_with( &src );
  let _exp = "\"abc\"".to_string();
  a_id!( got, _exp );

  // -

  let src = 13i32;
  let _got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let _exp = "13".to_string();
  a_id!( got, _exp );

  let src = "abc".to_string();
  let _got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let _exp = "abc".to_string();
  a_id!( got, _exp );

  // -

}

//

#[ test ]
fn borrowed()
{

  let src = 13;
  let _got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let _exp : Cow< '_, str > = Cow::Owned( "13".to_string() );
  a_id!( got, _exp );
  a_true!( matches!( got, Cow::Owned( _ ) ) );

}

//

#[ test ]
fn borrowed_str()
{
  use the_module::{ ToStringWith };

  // let src = "str";
  // let got = to_string::Ref::< '_, str, WithDisplay >::from( src ).to_string_with();
  // let exp : Cow< '_, str > = Cow::Borrowed( "str" );
  // a_id!( got, exp );
  // a_true!( matches!( got, Cow::Borrowed( _ ) ) );

  let src = "str";
  let _got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let _exp : Cow< '_, str > = Cow::Borrowed( "str" );
  a_id!( got, _exp );
  a_true!( !matches!( got, Cow::Borrowed( _ ) ) );

}

//

#[ test ]
fn borrowed_string()
{
  use the_module::{ ToStringWith };

  // let src = "string".to_string();
  // let got = to_string::Ref::< '_, String, WithDisplay >::from( &src ).to_string_with();
  // let exp : Cow< '_, str > = Cow::Borrowed( "string" );
  // a_id!( got, exp );
  // a_true!( matches!( got, Cow::Borrowed( _ ) ) );

  let src = "string".to_string();
  let _got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let _exp : Cow< '_, str > = Cow::Borrowed( "string" );
  a_id!( got, _exp );
  a_true!( !matches!( got, Cow::Borrowed( _ ) ) );

}
