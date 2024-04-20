use super::*;

#[ derive( Debug, PartialEq, former::Former ) ]
// #[ derive( Debug, PartialEq, former::Former ) ] #[ debug ]
// #[ derive( Debug, PartialEq ) ]
pub struct Struct1< 'a >
{
  pub string_slice_1 : &'a str,
}

// === begin_coercing of generated

// === end of generated

include!( "./only_test/string_slice.rs" );
