//! Implementation of sudoku problem for Hybrid Optimizer.

use std::collections::HashSet;
use crate::hybrid_optimizer::*;
use crate::problems::sudoku::*;

use derive_tools::{ From, InnerFrom, exposed::Display };
use deterministic_rand::{ Hrng, Rng, seq::SliceRandom };
use iter_tools::Itertools;

/// Trait that implements SA specific methods for sudoku board.
trait BoardExt
{
  /// Validate that each bloack has at least one non-fixed cell.
  fn validate_block_has_non_fixed_cells( &self, block : BlockIndex ) -> bool;
}

impl BoardExt for Board
{
  fn validate_block_has_non_fixed_cells( &self, block : BlockIndex ) -> bool
  {
    let fixed = self.block_cells( block )
    .map( | cell | self.cell( cell ) )
    .fold( 0, | acc, e | if e == 0.into() { acc + 1 } else { acc } )
    ;
    if fixed <= 1 || fixed >= 10
    {
      log::info!( "can't swap cells in block {block:?} that has {fixed} fixed cells" );
      return false;
    }

    true
  }
}

/// Get a pair of random non-fixed cells in a specified block.
pub fn cells_pair_random_in_block( initial : &Board, block : BlockIndex, hrng : Hrng ) -> Option< ( CellIndex, CellIndex ) >
{

  if !initial.validate_block_has_non_fixed_cells( block.clone() )
  {
    return None;
  }

  let cell1 = loop
  {
    let cell1 = CellIndex::random_in_block( block, hrng.clone() );
    log::trace!( "cell1 : {cell1:?}" );
    let is_fixed = initial.cell( cell1 ) != 0.into();
    if !is_fixed
    {
      break cell1;
    }
  };

  let cell2 = loop
  {
    let cell2 = CellIndex::random_in_block( block, hrng.clone() );
    log::trace!( "cell2 : {cell2:?}" );
    if cell1 == cell2
    {
      continue;
    }
    let is_fixed = initial.cell( cell2 ) != 0.into();
    if !is_fixed
    {
      break cell2;
    }
  };

  Some( ( cell1, cell2 ) )
}

/// Represents number of errors in sudoku board.
#[ derive( Default, Debug, Display, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, From, InnerFrom ) ]
pub struct SudokuCost( usize );

// xxx : derive, please
impl SudokuCost
{
  /// Converts SudokuCost struct into its inner usize value.
  pub fn unwrap( self ) -> usize
  {
    self.0
  }
}

/// Transforms SudokuCost into f64.
impl From< SudokuCost > for f64
{
  #[ inline ]
  fn from( src : SudokuCost ) -> Self
  {
    src.0 as f64
  }
}

/// Represents state of sudoku board filled with random digits and the number of the errors of the board as the cost.
#[ derive( PartialEq, Eq, Clone, Debug ) ]
pub struct SudokuPerson
{
  /// Sudoku board.
  pub board : Board,
  /// Number of errors in sudoku board.
  pub cost : SudokuCost,
}

impl Individual for SudokuPerson
{
  fn is_optimal( &self ) -> bool
  {
    if self.cost == 0.into()
    {
      true
    }
    else
    {
      false
    }
  }

  fn fitness( &self ) -> usize
  {
    self.cost.into()
  }

  fn update_fitness( &mut self, value : f64 )
  {
    self.cost = ( value as usize ).into();
  }
}

impl SudokuPerson
{
  /// Create new SudokuPerson from initial configuration of sudoku board.
  pub fn new( initial_board : &Board, hrng : Hrng ) -> Self
  {
    let mut board = initial_board.clone();
    board.fill_missing_randomly( hrng.clone() );
    let cost : SudokuCost = board.total_error().into();
    SudokuPerson { board, cost }
  }

  /// Create new SudokuPerson from board filled with values.
  pub fn with_board( board : Board ) -> Self
  {
    let cost : SudokuCost = board.total_error().into();
    SudokuPerson { board, cost }
  }

  /// Change state of the board by applying provided mutagen to current sudoku board.
  pub fn mutate( &mut self, mutagen : &SudokuMutagen )
  {
    let old_cross_error = self.board.cross_error( mutagen.cell1 )
      + self.board.cross_error( mutagen.cell2 );

    log::trace!( "cells_swap( {:?}, {:?} )", mutagen.cell1, mutagen.cell2 );
    self.board.cells_swap( mutagen.cell1, mutagen.cell2 );
    self.cost = SudokuCost( self.cost.unwrap() - old_cross_error ) ;
    self.cost = SudokuCost( self.cost.unwrap() + self.board.cross_error( mutagen.cell1 ) );
    self.cost = SudokuCost( self.cost.unwrap() + self.board.cross_error( mutagen.cell2 ) );
  }

  /// Create random mutagen and apply it current board.
  pub fn mutate_random( &self, initial_board : &Board, hrng : Hrng ) -> Self
  {
    let mutagen = self.mutagen( initial_board, hrng );
    let mut p = self.clone();
    p.mutate( &mutagen.into() );
    p
  }

  /// Create new SudokuMutagen as random cells pair in random sudoku block in current board.
  pub fn mutagen( &self, initial : &Board, hrng : Hrng ) -> SudokuMutagen
  {
    let mutagen;
    loop
    {
      let rng_ref = hrng.rng_ref();
      let mut rng = rng_ref.lock().unwrap();
      let block : BlockIndex = rng.gen();
      drop( rng );
      if let Some( m ) = cells_pair_random_in_block( &initial, block, hrng.clone() )
      {
        mutagen = m;
        break;
      }
    }
    mutagen.into()
  }
}

/// Represents single change(mutation) which contains indeces of two swapped cells. It is used to generate new state of the board for sudoku solving process.
#[ derive( PartialEq, Eq, Clone, Debug, From, InnerFrom ) ]
pub struct SudokuMutagen
{
  /// Index of cell swapped in mutation.
  pub cell1 : CellIndex,
  /// Index of cell swapped in mutation.
  pub cell2 : CellIndex,
}

/// Initial sudoku.
#[ derive( Debug, Clone ) ]
pub struct SudokuInitial
{
  /// Initial sudoku board with empty fields.
  board : Board,
}

impl SudokuInitial
{
  /// Create new instance of initial sudoku.
  pub fn new( board : Board ) -> Self
  {
    Self { board }
  }
}

impl InitialProblem for SudokuInitial
{
  type Person = SudokuPerson;

  fn get_random_person( &self, hrng : Hrng ) -> SudokuPerson
  {
    SudokuPerson::new( &self.board, hrng.clone() )
  }

  fn evaluate( &self, person : &SudokuPerson ) -> f64
  {
    person.board.total_error() as f64
  }
}

/// Mutation that randomly swaps two values in sudoku board, excluding values set in initial board.
#[ derive( Debug, Clone ) ]
pub struct RandomPairInBlockMutation;

impl MutationOperator for RandomPairInBlockMutation
{
  type Person = SudokuPerson;
  type Problem = SudokuInitial;

  fn mutate( &self, hrng : Hrng, person : &mut Self::Person, context : &Self::Problem )
    {
        let mutagen : SudokuMutagen =
        loop
        {
          let rng_ref = hrng.rng_ref();
          let mut rng = rng_ref.lock().unwrap();
          let block : BlockIndex = rng.gen();
          drop( rng );
          if let Some( m ) = cells_pair_random_in_block( &context.board, block, hrng.clone() )
          {
            break m;
          }
        }.into();
      let old_cross_error = person.board.cross_error( mutagen.cell1 )
        + person.board.cross_error( mutagen.cell2 );

      log::trace!( "cells_swap( {:?}, {:?} )", mutagen.cell1, mutagen.cell2 );
      person.board.cells_swap( mutagen.cell1, mutagen.cell2 );
      person.cost = SudokuCost( person.cost.unwrap() - old_cross_error );
      person.cost = SudokuCost( person.cost.unwrap() + person.board.cross_error( mutagen.cell1 ) );
      person.cost = SudokuCost( person.cost.unwrap() + person.board.cross_error( mutagen.cell2 ) );
    }

}

/// Crossover is performed by combining blocks from parents' boards, split in several randomly chosen crossover points.
#[ derive( Debug, Clone ) ]
pub struct MultiplePointsBlockCrossover;

impl CrossoverOperator for MultiplePointsBlockCrossover
{
  type Person = SudokuPerson;
  fn crossover( &self, hrng : Hrng, parent1 : &Self::Person, parent2 : &Self::Person ) -> Self::Person
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();

    let possible_values = [ 1, 2, 3, 4, 5, 6, 7, 8 ];
    let first_parent_blocks_number = possible_values.choose( &mut *rng ).unwrap();
    let mut first_parent_blocks : HashSet< BlockIndex > = HashSet::new();

    while first_parent_blocks.len() != *first_parent_blocks_number
    {
      first_parent_blocks.insert( rng.gen() );
    }

    let mut child_storage: Vec< CellVal > = vec![ 0.into(); 81 ];

    for i in parent1.board.blocks()
    {
      if first_parent_blocks.contains( &i )
      {
        let parent_block = parent1.board.block( i ).collect_vec();
        let cells = parent1.board.block_cells( i );
        for ( index, cell_index ) in cells.enumerate()
        {
          child_storage[ usize::from( cell_index ) ] = parent_block[ index ];
        }
      }
      else
      {
        let parent_block = parent2.board.block( i ).collect_vec();
        let cells = parent2.board.block_cells( i );
        for ( index, cell_index ) in cells.enumerate()
        {
          child_storage[ usize::from( cell_index ) ] = parent_block[ index ];
        }
      }
    }

    let child = SudokuPerson::with_board( Board::new( child_storage ) );
    child
  }
}

/// Crossover performed by selecting blocks with best rows or columns from two Individuals.
#[ derive( Debug, Clone ) ]
pub struct BestRowsColumnsCrossover;

impl CrossoverOperator for BestRowsColumnsCrossover
{
  type Person = < SudokuInitial as InitialProblem >::Person;

  fn crossover( &self, _hrng : Hrng, parent1 : &Self::Person, parent2 : &Self::Person ) -> Self::Person
  {
    let mut rows_costs = vec![ Vec::new(); 2 ];
    let mut columns_costs = vec![ Vec::new(); 2 ];
    for ( index, parent ) in [ parent1, parent2 ].iter().enumerate()
    {
      rows_costs[ index ] = parent.board
      .rows()
      .map( | row | row.collect::< HashSet< _ > >().len() )
      .collect_vec()
      .chunks( 3 )
      .map( | costs | 27 - costs.iter().fold( 0, | acc, cost | acc + cost ) )
      .collect_vec()
      ;

      columns_costs[ index ] = parent.board
      .cols()
      .map( | row | row.collect::< HashSet< _ > >().len() )
      .collect_vec()
      .chunks( 3 )
      .map( | costs | 27 - costs.iter().fold( 0, | acc, cost | acc + cost ) )
      .collect_vec()
      ;
    }

    let mut child1_storage = vec![ CellVal::from( 0 ); 81 ];
    for i in 0..3
    {
      if rows_costs[ 0 ][ i ] < rows_costs[ 1 ][ i ]
      {
        for j in 0..3
        {
          let parent_block = parent1.board.block( BlockIndex::from( ( j as u8, i as u8 ) ) ).collect_vec();
          let cells = parent1.board.block_cells( BlockIndex::from( ( j as u8, i as u8 ) ) );
          for ( index, cell_index ) in cells.enumerate()
          {
            child1_storage[ usize::from( cell_index ) ] = parent_block[ index ];
          }
        }
      }
      else
      {
        for j in 0..3
        {
          let parent_block = parent2.board.block( BlockIndex::from( ( j as u8, i as u8 ) ) ).collect_vec();
          let cells = parent2.board.block_cells( BlockIndex::from( ( j as u8, i as u8 ) ) );
          for ( index, cell_index ) in cells.enumerate()
          {
            child1_storage[ usize::from( cell_index ) ] = parent_block[ index ];
          }
        }
      }
    }

    let mut child2_storage = vec![ CellVal::from( 0 ); 81 ];
    for i in 0..3
    {
      for j in 0..3
      {
        if columns_costs[ 0 ][ j ] < columns_costs[ 1 ][ j ]
        {
          let parent_block = parent1.board.block( BlockIndex::from( ( j as u8, i as u8 ) ) ).collect_vec();
          let cells = parent1.board.block_cells( BlockIndex::from( ( j as u8, i as u8 ) ) );
          for ( index, cell_index ) in cells.enumerate()
          {
            child2_storage[ usize::from( cell_index ) ] = parent_block[ index ];
          }
        }
        else
        {
          let parent_block = parent2.board.block( BlockIndex::from( ( j as u8, i as u8 ) ) ).collect_vec();
          let cells = parent2.board.block_cells( BlockIndex::from( ( j as u8, i as u8 ) ) );
          for ( index, cell_index ) in cells.enumerate()
          {
            child2_storage[ usize::from( cell_index ) ] = parent_block[ index ];
          }
        }
      }
    }

    let min_board = [ Board::new( child1_storage ), Board::new( child2_storage ) ]
    .into_iter()
    .min_by( | b1, b2 | b1.total_error().cmp( &b2.total_error() ) )
    .unwrap()
    ;

    SudokuPerson::with_board( min_board )
  }
}
