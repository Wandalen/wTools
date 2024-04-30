//! Structure must be public.
//! Otherwise public trait can't have it as type.

#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, former::Former ) ]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Foo
{
  bar : i32,
}

// == begin of generated

// == end of generated

#[ test ]
fn basic()
{
  let got = Foo::former().bar( 13 ).form();
  let exp = Foo { bar : 13 };
  a_id!( got, exp );
}