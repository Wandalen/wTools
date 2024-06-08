#[ allow( unused_imports ) ]
use super::*;

use assistant::
{
  Fields,
  IteratorTrait,
  MaybeAs,
  ToStringWith,
  WithDebug,
};

use std::
{
  fmt,
  // collections::HashMap,
  // borrow::Cow,
};

//

#[ test ]
fn test_vec_fields()
{

  let src = 13i32;
  let got = ToStringWith::< WithDebug >::to_string_with( &src );
  let exp = "13".to_string();
  a_id!( got, exp );

}
