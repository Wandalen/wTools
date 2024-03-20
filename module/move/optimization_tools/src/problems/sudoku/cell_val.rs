//! Contains CellVal structure that corresponds to single digit on Sudoku field.
//! 

use derive_tools::exposed::Display;

/// Represents the value of a cell in Sudoku. It can have a value from 1 to 9 or 0 if the cell is not assigned.
#[ derive( Default, Debug, Display, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash ) ]
pub struct CellVal( u8 );

impl CellVal
{
  /// Returns inner u8 value of CellVal.
  #[ inline ]
  pub fn unwrap( self ) -> u8
  {
    self.0
  }
}

/// Converts usize value into CellVal.
impl From< usize > for CellVal
{
  #[ inline ]
  fn from( src : usize ) -> Self
  {
    debug_assert!( src < 10 );
    Self ( src as u8 )
  }
}

/// Converts i32 value into CellVal.
impl From< i32 > for CellVal
{
  #[ inline ]
  fn from( src : i32 ) -> Self
  {
    debug_assert!( 0 <= src && src < 10 );
    Self ( src as u8 )
  }
}

/// Converts u32 value into CellVal.
impl From< u32 > for CellVal
{
  #[ inline ]
  fn from( src : u32 ) -> Self
  {
    debug_assert!( src < 10 );
    Self ( src as u8 )
  }
}

/// Converts u8 value into CellVal.
impl From< u8 > for CellVal
{
  #[ inline ]
  fn from( src : u8 ) -> Self
  {
    debug_assert!( src < 10 );
    Self ( src )
  }
}

/// Converts CellVal value into usize.
impl From< CellVal > for usize
{
  #[ inline ]
  fn from( src : CellVal ) -> Self
  {
    src.0 as usize
  }
}
