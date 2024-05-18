#[ allow( unused_imports ) ]
use super::*;

#[ allow( unused_imports ) ]
use the_module::exposed::*;

#[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
struct Struct1
{
  a : i32,
  b : i32,
}

// Standard From and Into auto derive From1 and To_1.

include!( "./only_test/from2_named.rs" );
include!( "./only_test/from2_std_named.rs" );
