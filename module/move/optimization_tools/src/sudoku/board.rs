use super::*;
use std::fmt;
use std::collections::HashSet;
use iter_tools::Itertools;
use deterministic_rand::{ Hrng, IfDeterminismIteratorExt, seq::SliceRandom };

/// Represents a Sudoku board as vector of CellVal values.
#[ derive( PartialEq, Eq, Hash, Clone ) ]
pub struct Board
{
  storage : Vec< CellVal >,
}

impl Board
{
  /// Create new instance of Board from vector of CellVal.
  pub fn new( storage : Vec< CellVal > ) -> Self
  {
    debug_assert_eq!( storage.len(), 81 );
    Self { storage }
  }

  /// Get value of cell by given index.
  #[ inline ]
  pub fn cell< IntoCellFlatIndex >( &self, index : IntoCellFlatIndex ) -> CellVal
  where
    IntoCellFlatIndex : Into< CellFlatIndex >,
  {
    let index : usize = index.into().into();
    self.storage[ index ]
  }

  /// Get sequence of pairs of CellIndexes and CellVal values.
  pub fn cells( &self ) -> impl Iterator< Item = ( CellIndex, CellVal ) > + '_
  {
    self.storage.iter().enumerate().map( | ( k, e ) | ( CellIndex::from( CellFlatIndex::from( k ) ), *e ) )
  }

  /// Get sequence of values in given row.
  pub fn row( &self, index : usize ) -> impl Iterator< Item = CellVal > + '_
  {
    self.storage.iter().cloned().skip( index * 9 ).take( 9 )
  }

  /// Get sequence of rows in sudoku board.
  pub fn rows( &self ) -> impl Iterator< Item = impl Iterator< Item = CellVal > + '_ >
  {
    ( 0..9 ).map( move | i | self.row( i ) )
  }

  /// Get sequence of values of column by its index.
  pub fn col( &self, index : usize ) -> impl Iterator< Item = CellVal > + '_
  {
    self.storage.iter().cloned().skip( index ).step_by( 9 )
  }

  /// Get sequence columns columns in sudoku board.
  pub fn cols( &self ) -> impl Iterator< Item = impl Iterator< Item = CellVal > + '_ >
  {
    ( 0..9 ).map( move | i | self.col( i ) )
  }

  /// Get sequence of values of block by block index.
  pub fn block( &self, index : BlockIndex ) -> impl Iterator< Item = CellVal > + '_
  {
    let mut i = 0;
    let offset = index.first_cell().into();
    let result = self.storage.iter().cloned().skip( offset ).take( 3 );
    i += 1;
    let result = result.chain( self.storage.iter().cloned().skip( offset + i*9 ).take( 3 ) );
    i += 1;
    let result = result.chain( self.storage.iter().cloned().skip( offset + i*9 ).take( 3 ) );
    result
  }

  /// Get sequence of blocks in sudoku board.
  pub fn blocks( &self ) -> impl Iterator< Item = BlockIndex >
  {
    ( 0..9 ).map( move | i | ( i % 3, i / 3 ).into() )
  }

  /// Get sequence of cell values by its indices.
  pub fn select< 'a >( &'a self, indices : impl Iterator< Item = CellFlatIndex > + 'a ) -> impl Iterator< Item = CellVal > + 'a
  {
    indices.map( | i | self.storage[ usize::from( i ) ] )
  }

  /// Get sequence of cell values by its indices with mutable access.
  pub fn select_mut< 'a >( &'a mut self, indices : impl Iterator< Item = CellFlatIndex > + 'a ) -> impl Iterator< Item = &'a mut CellVal > + 'a
  {
    let storage_ptr = self.storage.as_mut_ptr();
    indices.map( move | i | unsafe { &mut *storage_ptr.add( usize::from( i ) ) } )
  }

  /// Get iterator over indices of cells in block by given block index.
  pub fn block_cells( &self, index : BlockIndex ) -> std::array::IntoIter< CellFlatIndex, 9 >
  {

    let mut indices : [ CellFlatIndex ; 9 ] = [ 0.into() ; 9 ];
    let mut i1 = 0;
    let mut i2: usize = index.first_cell().into();
    for _ in 0..3
    {
      for _ in 0..3
      {
        indices[ i1 ] = i2.into();
        i1 += 1;
        i2 += 1;
      }
      i2 += 9 - 3;
    }

    indices.into_iter()
  }

//   pub fn blocks_indices( &self ) -> Vec< impl Iterator< Item = usize > + '_ >
//   {
//     use std::sync::OnceLock;
//
//     static CELL : OnceLock< Vec< std::array::IntoIter< usize, 9 > > > = OnceLock::new();
//     let result = CELL.get_or_init
//     ( ||
//     {
//       ( 0..9 ).map( move | i | self.block_cells( ( i % 3, i / 3 ).into() ) ).collect()
//     });
//
//     result.clone()
//   }

  /// Get digits that are missing in block by its index.
  pub fn block_missing_vals( &self, index : BlockIndex ) -> HashSet< CellVal >
  {
    use std::sync::OnceLock;
    static DIGITS : OnceLock< HashSet< CellVal > > = OnceLock::new();
    let digits: &HashSet< CellVal > = DIGITS.get_or_init
    ( ||
    {
      [ 1, 2, 3, 4, 5, 6, 7, 8, 9 ].into_iter().map( | e | e.into() ).collect()
    });

    let has : HashSet< CellVal > = self.block( index ).filter( | &e | e != 0.into() ).unique().collect();
    digits.difference( &has ).cloned().collect()
  }

  /// Randomly fills empty positions in sudoku board.
  pub fn fill_missing_randomly( &mut self, hrng : Hrng ) -> &mut Self
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    for block in self.blocks()
    {
      let missing_vals = self.block_missing_vals( block );
      // println!( "for block {block:?} missing {missing_vals:?}" );
      let mut missing_vals : Vec< CellVal > = missing_vals.into_iter().if_determinism_then_sort().collect();
      missing_vals.shuffle( &mut *rng );
      let mut missing_val = missing_vals.into_iter();
      let cells = self.block_cells( block );
      cells.for_each( | cell_index |
      {
        let cell_val = &mut self.storage[ usize::from( cell_index ) ];
        if *cell_val != 0.into()
        {
          return;
        }
        *cell_val = missing_val.next().unwrap();
      });
    }
    self
  }

  /// Calculates number of errors in column and row that given cell position belongs to.
  pub fn cross_error( &self, index : CellIndex ) -> usize
  {
    let mut error : usize = 0;
    error += 9 - self.col( index.col() as usize ).filter( | &e | e != 0.into() ).unique().count();
    error += 9 - self.row( index.row() as usize ).filter( | &e | e != 0.into() ).unique().count();
    error
  }

  /// Calculates number of errors(duplicate digits) in sudoku board.
  pub fn total_error( &self ) -> usize
  {
    let mut error : usize = 0;
    for i in 0..9
    {
      error += self.cross_error( ( i, i ).into() );
    }
    error
  }

  pub fn cells_swap( &mut self, index1 : CellIndex, index2 : CellIndex )
  {
    self.storage.swap( index1.into(), index2.into() );
  }

}

/// Sets default value for board.
impl Default for Board
{
  fn default() -> Self
  {
    let storage : Vec< CellVal > =
    [
      3,1,0, 0,0,0, 0,2,0,
      0,0,6, 1,0,9, 0,0,5,
      0,0,0, 0,8,0, 0,0,0,
      0,2,0, 8,0,4, 0,5,0,
      0,0,4, 0,7,0, 0,0,0,
      0,0,0, 0,6,0, 0,0,8,
      0,6,0, 0,0,0, 9,0,0,
      0,0,9, 4,0,5, 0,0,1,
      0,0,0, 0,0,7, 0,0,0,
    ].into_iter().map( | e | e.into() ).collect();
    Board::new( storage )
  }
}

/// Create Board from value that can be converted to str.
impl< Src > From< Src > for Board
where
  Src : AsRef< str >,
{
  fn from( src : Src ) -> Self
  {
    let src = src.as_ref().trim();
    let storage: Vec< CellVal > = src
    .split( '\n' )
    .flat_map( | e | e.chars().filter( | ch | ch.is_ascii_digit() ) )
    .filter_map( | e | e.to_digit( 10 ).map( | num | num.into() ) )
    .collect()
    ;
    Self::new( storage )
  }
}

/// Output representation of sudoku board.
impl fmt::Display for Board
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    for row in self.rows()
    {
      let mut line_str = row.map( | e | e.to_string() ).collect::< String >();
      line_str.push_str( "\n" );
      write!( f, "{line_str}" )?;
    }
    write!( f, "" )
  }
}

impl fmt::Debug for Board
{
  fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
  {
    fmt::Display::fmt( self, f )
  }
}
