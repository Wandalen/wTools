use super::*;

#[ derive( Debug, PartialEq, Eq, TheModule::FromInner ) ]
struct MyStruct
{
  a: i32,
}

include!( "./only_test/from_inner_named.rs" );
