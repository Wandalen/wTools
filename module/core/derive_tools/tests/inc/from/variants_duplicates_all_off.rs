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
  #[ from( off ) ]
  FromString2( String ),
  #[ from( off ) ]
  FromPair( String, String ),
  #[ from( off ) ]
  FromPair2( String, String ),
  FromBin( &'static [ u8 ] ),
  Nothing3,
}

impl From< String > for GetData
{
  #[ inline ]
  fn from( src : String ) -> Self
  {
    Self::FromString2( src )
  }
}

impl From< ( String, String ) > for GetData
{
  #[ inline ]
  fn from( src : ( String, String ) ) -> Self
  {
    Self::FromPair2( src.0, src.1 )
  }
}

// == begin of generated

// == end of generated

include!( "./only_test/variants_duplicates.rs" );
