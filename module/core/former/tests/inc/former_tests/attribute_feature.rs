#[ allow( unused_imports ) ]
use super::*;

// xxx : need to fix

#[ derive( former::Former ) ]
struct Foo
{
  // #[ cfg( feature = "baz" ) ]
  bar : i32,
}

// error => Unknown attribute #[cfg(feature = "baz")]
