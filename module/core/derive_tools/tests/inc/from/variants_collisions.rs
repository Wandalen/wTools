#![ allow( non_snake_case ) ]
#![ allow( unused_imports ) ]
use super::*;

pub mod core {}
pub mod std {}
pub mod marker {}

pub mod FromString {}
pub mod FromPair {}
pub mod FromBin {}

// qqq : add collision nests for 4 outher branches

#[ derive( Debug, PartialEq, the_module::From ) ]
// #[ debug ]
pub enum GetData
{
  #[ allow( dead_code ) ]
  Nothing,
  FromString( String ),
  FromPair( String, String ),
  FromBin( &'static [ u8 ] ),
}

// == begin of generated
// == end of generated

include!( "./only_test/variants.rs" );
