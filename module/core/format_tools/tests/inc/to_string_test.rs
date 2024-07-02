#[ allow( unused_imports ) ]
use super::*;

use the_module::
{
  // Fields,
  // IteratorTrait,
  // MaybeAs,
  ToStringWith,
  // _ToStringWithFallback,
  // ToStringWithFallbackParams,
  WithDebug,
  WithDisplay,
  // Ref,
  // to_string_with_fallback,
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
