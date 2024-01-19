//! Contains implementation of Simmulated Annealing optimization method.
//! 

use crate::*;
#[ cfg( feature="static_plot" ) ]
use crate::plot::{ PlotDescription, PlotOptions, plot };
use rayon::iter::{ ParallelIterator, IndexedParallelIterator};
use sudoku::{ Board, BlockIndex, CellIndex };
use deterministic_rand::Seed;

mod gen_alg;
pub use gen_alg::*;
mod sim_anneal;
pub use sim_anneal::*;

/// Pause execution of SA.
pub fn sleep()
{
  std::thread::sleep( std::time::Duration::from_secs( 5 ) );
}

/// Trait that implements SA specific methods for sudoku board.
trait BoardExt
{
  /// Validate that each bloack has at least one non-fixed cell.
  fn validate_each_block_has_non_fixed_cell( &self ) -> bool;
  fn validate_block_has_non_fixed_cells( &self, block : BlockIndex ) -> bool;
}

impl BoardExt for Board
{
  fn validate_each_block_has_non_fixed_cell( &self ) -> bool
  {
    for block in self.blocks()
    {
      let fixed = self.block_cells( block )
      .map( | cell | self.cell( cell ) )
      .fold( 0, | acc, e | if e == 0.into() { acc + 1 } else { acc } )
      ;
      if fixed <= 1 || fixed >= 10
      {
        return false;
      }
    }
    true
  }

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

use derive_tools::{ FromInner, InnerFrom, Display };
use derive_tools::{ Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign };

/// Represents number of errors in sudoku board.
#[ derive( Default, Debug, Display, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, FromInner, InnerFrom ) ]
#[ derive( Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign ) ]
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

/// Represents the reasons for the termination or proceeding with the Sudoku solving.
#[ derive( PartialEq, Eq, Clone, Copy, Debug, Display ) ]
pub enum Reason
{
  /// SA process was finished with optimal result.
  GoodEnough,
  /// SA process has not yet finished.
  NotFinished,
  /// SA process finished due to reaching limit of resets.
  ResetLimit,
  /// SA process finished due to reaching limit of generations.
  GenerationLimit,
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
    
    //let mut new = self.clone();
    log::trace!( "cells_swap( {:?}, {:?} )", mutagen.cell1, mutagen.cell2 );
    self.board.cells_swap( mutagen.cell1, mutagen.cell2 );
    self.cost -= old_cross_error.into();
    self.cost += self.board.cross_error( mutagen.cell1 ).into();
    self.cost += self.board.cross_error( mutagen.cell2 ).into();
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
#[ derive( PartialEq, Eq, Clone, Debug, FromInner, InnerFrom ) ]
pub struct SudokuMutagen
{
  /// Index of cell swapped in mutation.
  pub cell1 : CellIndex,
  /// Index of cell swapped in mutation.
  pub cell2 : CellIndex,
}

/// Represents a state in the Simulated Annealing optimization process for solving Sudoku.
#[ derive( Clone, Debug ) ]
pub struct SudokuGeneration
{
  /// Initial board with fixed values.
  initial_board : Board,
  /// Current temperature in the optimization process.
  temperature : Option< Temperature >,
  /// Number of resets performed.
  n_resets : usize,
  /// Amount of generations before current genetration.
  n_generation : usize,
  pub population : Vec< SudokuPerson >
}

pub struct SudokuInitial
{
  board : Board,
}

impl SudokuInitial
{
  pub fn new( board : Board ) -> Self
  {
    Self { board }
  }
}

impl SeederOperator for SudokuInitial
{
  type Generation = SudokuGeneration;
  /// Create the initial generation for the optimization algorithm.
  fn initial_generation( &self, hrng : Hrng ) -> SudokuGeneration
  {
    let person = SudokuPerson::new( &self.board, hrng.clone() );
    let n_generation = 0;
    SudokuGeneration { initial_board: self.board.clone(), population: vec![ person ], n_resets: 0, n_generation, temperature : None }
  }
  
}

pub struct HybridOptimizer< S : SeederOperator >
{
  pub sa_config : SAConfig,
  pub ga_config : GAConfig,
  pub seed : Seed,
  pub hrng : Hrng,
  pub generation_limit : usize,
  pub seeder : S,
}

impl< S : SeederOperator > HybridOptimizer< S >
{
  pub fn new( random_seed : Seed, population_seeder : S ) -> Self
  {
    Self
    {
      sa_config : SAConfig::default(),
      ga_config : GAConfig::default(),
      seed : random_seed.clone(),
      hrng : Hrng::master_with_seed( random_seed ),
      generation_limit : 10_000,
      seeder : population_seeder
    }
  }

  pub fn with_sa_config( self, config : SAConfig ) -> Self
  {
    Self
    {
      sa_config : config,
      ..self
    }
  }

  pub fn with_ga_config( self, config : GAConfig ) -> Self
  {
    Self
    {
      ga_config : config,
      ..self
    }
  }

  pub fn optimize( &self ) -> ( Reason, Option< < S as SeederOperator >::Generation > )
  {
    let mut generation = self.seeder.initial_generation( self.hrng.clone() );
    let mut generation_number = 1;

    loop
    {
      if generation_number > self.generation_limit
      {
        return ( Reason::GenerationLimit, None );
      }

      if generation.is_good_enough()
      {
        return ( Reason::GoodEnough, Some( generation ) );
      }

      let  new_generation = generation.evolve( self.hrng.clone(), EvolutionMode::SA( &self.sa_config ) );

      generation = new_generation;
      generation_number += 1;
    }
  } 
}
