#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, the_module::From ) ]
// #[ debug ]
pub enum GetData
{
  Nothing,
  Nothing2,
  #[ from( off ) ]
  FromString( String ),
  FromString2( String ),
  #[ from( off ) ]
  FromPair( String, String ),
  FromPair2( String, String ),
  FromBin( &'static [ u8 ] ),
  Nothing3,
}

// == begin of generated

// == end of generated

include!( "./only_test/variants_duplicates.rs" );
