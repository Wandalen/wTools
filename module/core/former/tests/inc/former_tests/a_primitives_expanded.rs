#[ allow( unused_imports ) ]
use super::*;

// #[ derive( Debug, PartialEq ) ]
#[ derive( Debug, PartialEq, the_module::Former ) ] #[ debug ]
pub struct Struct1
{
  pub int_1 : i32,
  string_1 : String,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : Option< String >,
}

// = generated

// = end of generated

include!( "./only_test/primitives.rs" );
