#[ allow( unused_imports ) ]
use super::*;

#[ allow( unused_imports ) ]
use the_module::exposed::*;

#[ derive( Debug, PartialEq, Default, VariadicFrom ) ]
struct StructNamedFields
{
  a : i32,
  b : i32,
}

// Standard From and Into auto derive From_1 and To_1.

include!( "./only_test/variadic_from2_named.rs" );
include!( "./only_test/variadic_std_from2_named.rs" );
