use super::*;
use deterministic_rand::{ Rng, distributions::{Distribution, Standard } };
use core::ops::Range;

#[ derive( Default, Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct BlockIndex( u8, u8 );

impl BlockIndex
{
  #[ inline ]
  pub fn first_cell( &self ) -> CellFlatIndex
  {
    ( self.0 as usize * 3 + ( self.1 as usize * 27 ) ).into()
  }
  /// Interval in which cell indcies of the block reside.
  #[ inline ]
  pub fn cells_intervals( &self ) -> ( Range< usize >, Range< usize > )
  {
    (
      self.0 as usize * 3 .. self.0 as usize * 3 + 3,
      self.1 as usize * 3 .. self.1 as usize * 3 + 3,
    )
  }
  #[ inline ]
  pub fn col( &self ) -> u8
  {
    self.0
  }
  #[ inline ]
  pub fn row( &self ) -> u8
  {
    self.1
  }
}

impl< T > From< ( T, T ) > for BlockIndex
where
  T : Into< u8 >,
{
  fn from( src : ( T, T ) ) -> Self
  {
    let a = src.0.into();
    let b = src.1.into();
    debug_assert!( a <= 2 );
    debug_assert!( b <= 2 );
    Self ( a, b )
  }
}

impl From< CellIndex > for BlockIndex
{
  #[ inline ]
  fn from( src : CellIndex ) -> Self
  {
    Self( src.col() / 3, src.row() / 3 )
  }
}

impl From< CellFlatIndex > for BlockIndex
{
  #[ inline ]
  fn from( src : CellFlatIndex ) -> Self
  {
    let src : CellIndex = src.into();
    src.into()
  }
}

impl Distribution< BlockIndex > for Standard
{
  fn sample< R : Rng + ?Sized >( &self, rng : &mut R) -> BlockIndex
  {
    ( rng.gen_range( 0..=2 ), rng.gen_range( 0..=2 ) ).into()
  }
}
