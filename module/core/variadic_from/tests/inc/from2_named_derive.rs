#[ allow( unused_imports ) ]
use super::*;

use variadic_from::{ from, From1, From2, Into1 };


#[ derive( Debug, PartialEq, variadic_from::VariadicFrom ) ]
struct Struct1
{
  a : i32,
  b : i32,
}

include!( "./only_test/from2_named.rs" );
