use super::*;

#[ derive( Debug, PartialEq, Eq, the_module::New ) ]
struct MyStruct
{
  a : i32,
}

include!( "./only_test/named.rs" );
