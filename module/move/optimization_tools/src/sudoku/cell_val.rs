use derive_tools::Display;
use derive_tools::{ Add, Sub, Mul, Div };

#[ derive( Default, Debug, Display, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash ) ]
#[ derive( Add, Sub, Mul, Div ) ]
pub struct CellVal( u8 );

impl CellVal
{
  #[ inline ]
  pub fn unwrap( self ) -> u8
  {
    self.0
  }
}

impl From< usize > for CellVal
{
  #[ inline ]
  fn from( src : usize ) -> Self
  {
    debug_assert!( src < 10 );
    Self ( src as u8 )
  }
}

impl From< i32 > for CellVal
{
  #[ inline ]
  fn from( src : i32 ) -> Self
  {
    debug_assert!( 0 <= src && src < 10 );
    Self ( src as u8 )
  }
}

impl From< u32 > for CellVal
{
  #[ inline ]
  fn from( src : u32 ) -> Self
  {
    debug_assert!( src < 10 );
    Self ( src as u8 )
  }
}

impl From< u8 > for CellVal
{
  #[ inline ]
  fn from( src : u8 ) -> Self
  {
    debug_assert!( src < 10 );
    Self ( src )
  }
}

impl From< CellVal > for usize
{
  #[ inline ]
  fn from( src : CellVal ) -> Self
  {
    src.0 as usize
  }
}
