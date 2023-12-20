//! Provides structures for representetion of position of single digit on Sudoku board.
//!
//! CellFlatIndex is used for indexing Sudoku board as one-dimensional array.
//! CellIndex is used for two-dimensional Sudoku board representation, where first value of
//! the tuple if row index and second value is index of the column.
//! 

use super::*;
use deterministic_rand::{ Rng, distributions::{ Distribution, Standard } };
// use super::BlockIndex;

/// Represents an index of a Sudoku cell in one-dimensional board array.
#[ derive( Default, Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct CellFlatIndex( usize );

impl CellFlatIndex
{
  /// Converts CellFlatIndex into its inner usize value.
  #[ inline ]
  pub fn unwrap( self ) -> usize
  {
    self.0
  }
}

/// Convert usize value into CellFlatIndex value.
impl From< usize > for CellFlatIndex
{
  #[ inline ]
  fn from( src : usize ) -> Self
  {
    let a = src.into();
    debug_assert!( a < 81 );
    Self ( a )
  }
}

/// Convert two-dimensional CellIndex value into CellFlatIndex value.
impl From< CellIndex > for CellFlatIndex
{
  #[ inline ]
  fn from( src : CellIndex ) -> Self
  {
    Self( src.0 as usize + src.1 as usize * 9 )
  }
}

/// Convert CellFlatIndex value into usize.
impl From< CellFlatIndex > for usize
{
  #[ inline ]
  fn from( src : CellFlatIndex ) -> Self
  {
    src.0
  }
}

/// Represents an index of a Sudoku cell in two-dimensional board representation.
#[ derive( Default, Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct CellIndex( u8, u8 );

impl CellIndex
{
  /// Random cell in a block.
  pub fn random_in_block( block : BlockIndex, hrng : Hrng ) -> Self
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let intervals = block.cells_intervals();

    ( rng.gen_range( intervals.0 ) as u8, rng.gen_range( intervals.1 ) as u8 ).into()
  }

  /// Column index of cell.
  #[ inline ]
  pub fn col( &self ) -> u8
  {
    self.0
  }

  /// Row index of cell.
  #[ inline ]
  pub fn row( &self ) -> u8
  {
    self.1
  }
}

/// Get random CellIndex value.
impl Distribution< CellIndex > for Standard
{
  fn sample< R : Rng + ?Sized >( &self, rng : &mut R) -> CellIndex
  {
    ( rng.gen_range( 0..=8 ), rng.gen_range( 0..=8 ) ).into()
  }
}

/// Transform a tuple of elements, that can be converted to u8, into CellIndex value.
impl< T > From< ( T, T ) > for CellIndex
where
  T : Into< u8 >,
{
  fn from( src : ( T, T ) ) -> Self
  {
    let a = src.0.into();
    let b = src.1.into();
    debug_assert!( a <= 8 );
    debug_assert!( b <= 8 );
    Self ( a, b )
  }
}

/// Convert CellFlatIndex value into CellIndex value.
impl From< CellFlatIndex > for CellIndex
{
  #[ inline ]
  fn from( src : CellFlatIndex ) -> Self
  {
    Self( src.0 as u8 % 9, src.0 as u8 / 9  )
  }
}

/// Convert CellIndex value into usize value.
impl From< CellIndex > for usize
{
  #[ inline ]
  fn from( src : CellIndex ) -> Self
  {
    let index : CellFlatIndex = src.into();
    index.into()
  }
}
