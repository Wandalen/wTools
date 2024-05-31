#![ allow( dead_code ) ]
#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, the_module::From ) ]
#[ from( off ) ]
// #[ debug ]
pub enum GetData
{
  Nothing,
  Nothing2,
  FromString( String ),
  #[ from( on ) ]
  // #[ from( debug ) ]
  FromString2( String ),
  FromPair( String, String ),
  #[ from( on ) ]
  FromPair2( String, String ),
  #[ from( on ) ]
  FromBin( &'static [ u8 ] ),
  Nothing3,
}

// == begin of generated
// == end of generated

include!( "./only_test/variants_duplicates.rs" );
