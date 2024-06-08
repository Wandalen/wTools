#[ allow( unused_imports ) ]
use super::*;

use assistant::
{
  // Fields,
  // IteratorTrait,
  // MaybeAs,
  ToStringWith,
  ToStringWithFallback,
  WithDebug,
  WithDisplay,
};

// use std::
// {
//   // fmt,
//   // collections::HashMap,
//   // borrow::Cow,
// };

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
fn to_string_with_fallback_test()
{

  // -

  let src = 13i32;
  let got = ToStringWithFallback::< WithDisplay, WithDebug >::to_string_with_fallback( &( &src, ) );
  let exp = "13".to_string();
  a_id!( got, exp );

  // -

}
