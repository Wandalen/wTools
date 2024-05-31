#[ allow( unused_imports ) ]
use super::*;

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
