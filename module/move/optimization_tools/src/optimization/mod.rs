//! Contains implementation of hybrid optimization using Simulated Annealing and Genetic optimization methods.
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
  /// Population of individuals in current generation.
  pub population : Vec< SudokuPerson >
}

/// Initial sudoku.
#[ derive( Debug ) ]
pub struct SudokuInitial
{
  /// Initial sudoku board with empty fields.
  board : Board,
}

/// Mode which represents algorithm used for optimization of current generation.
#[ derive( Debug ) ]
pub enum EvolutionMode< 'a >
{
  /// Simulated annealing optimization method.
  SA( &'a SAConfig ),
  /// Genetic optimization method.
  GA( &'a GAConfig ),
}

impl SudokuInitial
{
  /// Create new instance of initial sudoku.
  pub fn new( board : Board ) -> Self
  {
    Self { board }
  }
}

impl SeederOperator for SudokuInitial
{
  type Generation = SudokuGeneration;
  
  fn initial_generation( &self, hrng : Hrng, size : usize ) -> SudokuGeneration
  {
    let mut population = Vec::new();
    for _ in 0..size
    {
      population.push( SudokuPerson::new( &self.board, hrng.clone() ) );
    }
    SudokuGeneration { initial_board: self.board.clone(), population, n_resets: 0, temperature : None }
  }
  
}

/// Represents hybrid optimization method with both Simulated Annealing and Genetic Algorithm.
#[ derive( Debug ) ]
pub struct HybridOptimizer< S : SeederOperator >
{
  /// Configuration for SA optimization.
  pub sa_config : SAConfig,
  /// Configuration for GA optimization.
  pub ga_config : GAConfig,
  /// Hierarchical random numbers generator.
  pub hrng : Hrng,
  /// Struct responsible for creation of initial generation.
  pub seeder : S,
}

impl< S : SeederOperator > HybridOptimizer< S >
{
  /// Create new instance of HybridOptimizer with default config for SA and GA.
  pub fn new( random_seed : Seed, population_seeder : S ) -> Self
  {
    Self
    {
      sa_config : SAConfig::default(),
      ga_config : GAConfig::default(),
      hrng : Hrng::master_with_seed( random_seed ),
      seeder : population_seeder
    }
  }

  /// Create new instance of HybridOptimizer with provided SA config.
  pub fn with_sa_config( self, config : SAConfig ) -> Self
  {
    Self
    {
      sa_config : config,
      ..self
    }
  }

  /// Create new instance of HybridOptimizer with provided GA config.
  pub fn with_ga_config( self, config : GAConfig ) -> Self
  {
    Self
    {
      ga_config : config,
      ..self
    }
  }

  /// Perform hybrid SA/GA optimization.
  pub fn optimize( &self, strategy : &HybridStrategy ) -> ( Reason, Option< < S as SeederOperator >::Generation > )
  {
    let mut generation = self.seeder.initial_generation( self.hrng.clone(), strategy.population_size );
    let mut generation_number = 1;

    loop
    {
      if generation_number > strategy.generation_limit
      {
        return ( Reason::GenerationLimit, None );
      }

      if generation.is_good_enough()
      {
        return ( Reason::GoodEnough, Some( generation ) );
      }

      let mode;
      let mut iterations = generation_number;
      let mut cycle = 1usize;
      while let Some( res ) = cycle.checked_sub( strategy.sa_generations_number + strategy.ga_generations_number )
      {
        
        cycle += 1;
        iterations = res;
      }
      if cycle > strategy.number_of_cycles
      {
        if let StrategyMode::GA = strategy.finalize_with
        {
          mode = EvolutionMode::GA( &self.ga_config );
        }
        else 
        {
          mode = EvolutionMode::SA( &self.sa_config );
        }
      }
      else 
      {
        match strategy.start_with
        {
          StrategyMode::GA if iterations > strategy.ga_generations_number && strategy.sa_generations_number > 0 =>
          {
            mode = EvolutionMode::SA( &self.sa_config );
          },
          StrategyMode::GA => mode = EvolutionMode::GA( &self.ga_config ),
          StrategyMode::SA if iterations > strategy.sa_generations_number && strategy.ga_generations_number > 0 =>
          {
            mode = EvolutionMode::GA( &self.ga_config );
          }
          StrategyMode::SA => mode = EvolutionMode::SA( &self.sa_config ),
        }
      }

      let new_generation = generation.evolve( self.hrng.clone(), mode );

      generation = new_generation;
      generation_number += 1;
    }
  } 
}

/// Strategy for combination of SA and GA optimization. Performs cyclic optimization with iteration of SA and GA methods in order defined by srart_with field.
#[ derive( Debug ) ]
pub struct HybridStrategy
{
  /// Starting method of optimization.
  pub start_with : StrategyMode,
  /// Finishing method of optimization.
  pub finalize_with : StrategyMode,
  /// Number of cycles of optimization with GA and SA algorithms.
  pub number_of_cycles : usize,
  /// Number of generations optimized by SA algorithm in each cycle of optimization.
  pub sa_generations_number : usize,
  /// Number of generations optimized by GA algorithm in each cycle of optimization.
  pub ga_generations_number : usize,
  /// Percent of population selected for next cycle of optimization.
  pub population_percent : f64,
  /// Max number of generations, termination condition.
  pub generation_limit : usize,
  /// Number of Individuals in initial generation of solutions.
  pub population_size : usize,
}

impl Default for HybridStrategy
{
  fn default() -> Self 
  {
    Self
    {
      sa_generations_number : 1000,
      ga_generations_number : 1000,
      number_of_cycles : 1,
      finalize_with : StrategyMode::SA,
      population_percent : 1.0,
      start_with : StrategyMode::GA,
      generation_limit : 10_000_000,
      population_size : 1000,
    }
  }
}

/// Methods of optimization.
#[ derive( Debug ) ]
pub enum StrategyMode
{
  SA,
  GA,
}
