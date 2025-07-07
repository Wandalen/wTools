use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive( Deref ) ]
struct StructNamedWithAttr
{
  a : String,
  #[ deref ]
  b : i32,
}

include!( "./only_test/struct_named_with_attr.rs" );