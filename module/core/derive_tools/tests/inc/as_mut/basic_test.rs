#![allow(unused_imports)]
use super :: *;
use derive_tools ::AsMut;

#[ derive( AsMut ) ]
struct StructNamed 
{
  #[ as_mut ]
  field1: i32,
}

include!("only_test/struct_named.rs");
