#[ allow( unused_imports ) ]
use super::*;

// #[ test ]
// fn from_unnamed()

use the_module::prelude::*;

#[ derive( Debug, PartialEq, the_module::VariadicFrom ) ]
struct StructTuple( i32, i32 );

include!( "./only_test/from2_unnamed.rs" );
