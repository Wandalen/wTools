#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, the_module::From ) ]
#[ debug ]
pub enum GetData
{
  #[ allow( dead_code ) ]
  Nothing,
  FromString( String ),
  FromBin( &'static [ u8 ] ),
}

// == begin of generated

#[ automatically_derived ]
impl From< (String) > for GetData
{
  #[ inline ]
  fn from( src : (String) ) -> Self
  {
    Self::FromString( src )
  }
}

#[ automatically_derived ]
impl From< ( & 'static [u8] ) > for GetData
{
  #[ inline ]
  fn from( src : ( & 'static [u8] ) ) -> Self
  {
    Self::FromBin( src )
  }
}

// == end of generated

include!( "./only_test/from_inner_variants.rs" );

// xxx2 : get completed
