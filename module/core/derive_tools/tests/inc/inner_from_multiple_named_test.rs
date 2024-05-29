use super::*;

#[ derive( Debug, PartialEq, Eq, the_module::InnerFrom ) ]
struct StructNamedFields
{
  a: i32,
  b: bool,
}

include!( "./only_test/inner_from_multiple_named.rs" );
