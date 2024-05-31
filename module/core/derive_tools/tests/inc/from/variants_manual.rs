#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq ) ]
pub enum GetData
{
  #[ allow( dead_code ) ]
  Nothing,
  FromString( String ),
  FromPair( String, String ),
  FromBin( &'static [ u8 ] ),
}

impl From< String > for GetData
{
  #[ inline ]
  fn from( src : String ) -> Self
  {
    Self::FromString( src )
  }
}

impl From< ( String, String ) > for GetData
{
  #[ inline ]
  fn from( src : ( String, String ) ) -> Self
  {
    Self::FromPair( src.0, src.1 )
  }
}

impl From< &'static [ u8 ] > for GetData
{
  #[ inline ]
  fn from( src : &'static [ u8 ] ) -> Self
  {
    Self::FromBin( src )
  }
}

include!( "./only_test/variants.rs" );
