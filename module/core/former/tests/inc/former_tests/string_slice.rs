use super::*;

// #[ derive( Debug, PartialEq, former::Former ) ]
#[ derive( Debug, PartialEq, former::Former ) ] #[ debug ]
pub struct Struct1< 'a >
{
  pub string_slice_1 : &'a str,
}

// === begin of generated



// === end of generated

// include!( "./only_test/string_slice.rs" );
// xxx : uncomment
