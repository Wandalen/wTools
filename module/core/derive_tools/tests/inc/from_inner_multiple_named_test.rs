use super::*;

#[ derive( Debug, PartialEq, Eq, the_module::exposed::FromInner ) ]
struct StructNamedFields
{
  a: i32,
  b: bool,
}

include!( "./only_test/from_inner_multiple_named.rs" );
