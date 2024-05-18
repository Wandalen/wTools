#[ allow( unused_imports ) ]
use super::*;

use variadic_from::{ from, From1, From2, Into1 };


#[ derive( Debug, PartialEq, variadic_from::VariadicFrom ) ]
struct Struct1( i32, i32 );

include!( "./only_test/from2_unnamed.rs" );
