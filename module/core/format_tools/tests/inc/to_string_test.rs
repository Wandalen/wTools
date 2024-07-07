#[ allow( unused_imports ) ]
use super::*;

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
  let got = ToStringWith::< WithDebug >::to_string_with( &src );
  let exp = "13".to_string();
  a_id!( got, exp );

  let src = "abc".to_string();
  let got = ToStringWith::< WithDebug >::to_string_with( &src );
  let exp = "\"abc\"".to_string();
  a_id!( got, exp );

  // -

  let src = 13i32;
  let got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let exp = "13".to_string();
  a_id!( got, exp );

  let src = "abc".to_string();
  let got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let exp = "abc".to_string();
  a_id!( got, exp );

  // -

}

//

#[ test ]
fn borrowed()
{

  let src = 13;
  let got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let exp : Cow< '_, str > = Cow::Owned( "13".to_string() );
  a_id!( got, exp );
  a_true!( matches!( got, Cow::Owned( _ ) ) );

  let src = "str";
  let got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let exp : Cow< '_, str > = Cow::Borrowed( "str" );
  a_id!( got, exp );
  a_true!( matches!( got, Cow::Borrowed( _ ) ) );

  let src = "string".to_string();
  let got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let exp : Cow< '_, str > = Cow::Borrowed( "string" );
  a_id!( got, exp );
  a_true!( matches!( got, Cow::Borrowed( _ ) ) );

}

//

#[ test ]
fn borrowed_str()
{

  let src = "str";
  let got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let exp : Cow< '_, str > = Cow::Borrowed( "str" );
  a_id!( got, exp );
  a_true!( matches!( got, Cow::Borrowed( _ ) ) );

}

//

#[ test ]
fn borrowed_string()
{

  let src = "string".to_string();
  let got = ToStringWith::< WithDisplay >::to_string_with( &src );
  let exp : Cow< '_, str > = Cow::Borrowed( "string" );
  a_id!( got, exp );
  a_true!( matches!( got, Cow::Borrowed( _ ) ) );

}
