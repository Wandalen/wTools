#![ deny( missing_docs ) ]

#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, former::Former ) ]
// #[ derive( Debug, PartialEq, former::Former ) ] #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Struct1
{
  pub int_1 : i32,
}

// == begin of generated

// == end of generated

include!( "./only_test/basic.rs" );
