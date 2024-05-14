
#[ derive( Debug, PartialEq ) ]
pub enum GetData
{
  FromString( String ),
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

impl From< &'static [ u8 ] > for GetData
{
  #[ inline ]
  fn from( src : &'static [ u8 ] ) -> Self
  {
    Self::FromBin( src )
  }
}

include!( "./only_test/from_inner_variants.rs" );
