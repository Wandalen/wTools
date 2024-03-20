use super::*;

#[ derive( Debug, PartialEq, Eq, the_module::exposed::InnerFrom ) ]
struct MyStruct
{
  a: i32,
}

include!( "./only_test/inner_from_named.rs" );
