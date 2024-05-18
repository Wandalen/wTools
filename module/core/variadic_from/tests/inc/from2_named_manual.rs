#[ allow( unused_imports ) ]
use super::*;

use the_module::prelude::*;

#[ derive( Debug, PartialEq, the_module::VariadicFrom ) ]
struct StructNamedFields
{
  a : i32,
  b : i32,
}

include!( "./only_test/from2_named.rs" );
