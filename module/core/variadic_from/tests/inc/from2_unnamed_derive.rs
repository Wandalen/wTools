#[ allow( unused_imports ) ]
use super::*;

use the_module::{ from, From1, From2, Into1 };


#[ derive( Debug, PartialEq, the_module::VariadicFrom ) ]
struct Struct1( i32, i32 );

include!( "./only_test/from2_unnamed.rs" );
