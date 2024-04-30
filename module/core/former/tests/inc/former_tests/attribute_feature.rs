#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq ) ]
pub struct BaseCase
{
  #[ cfg( feature = "enabled" ) ]
  enabled : i32,
  #[ cfg( feature = "disabled" ) ]
  disabled : i32,
}

#[ derive( Debug, PartialEq, former::Former ) ]
// #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Foo
{
  #[ cfg( feature = "enabled" ) ]
  #[ allow( dead_code ) ]
  enabled : i32,
  #[ cfg( feature = "disabled" ) ]
  disabled : i32,
}

// == begin of generated

// == end of generated

#[ test ]
fn basecase()
{
  let got = BaseCase { enabled : 13 };
  let exp = BaseCase { enabled : 13 };
  a_id!( got, exp );
}

#[ test ]
fn basic()
{
  let got = Foo::former().enabled( 13 ).form();
  let exp = Foo { enabled : 13 };
  a_id!( got, exp );
}
