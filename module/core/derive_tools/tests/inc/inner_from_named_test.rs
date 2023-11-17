use super::*;

#[derive( Debug, PartialEq, Eq, TheModule::InnerFrom ) ]
struct MyStruct
{
  a: i32,
}

include!( "./only_test/inner_from_named.rs" );
