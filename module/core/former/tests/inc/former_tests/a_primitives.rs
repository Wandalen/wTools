#![ deny( missing_docs ) ]

#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, former::Former ) ]
// #[ derive( Debug, PartialEq, former::Former ) ] #[ debug ]
// #[ derive( Debug, PartialEq ) ] #[ debug ]
pub struct Struct1
{
  pub int_1 : i32,
  string_1 : String,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : Option< String >,
}

// = begin_coercing of generated

// == end of generated

include!( "./only_test/primitives.rs" );
