//! Contains implementation of hybrid optimization using Simulated Annealing and Genetic optimization methods.
//! 

use crate::*;
#[ cfg( feature="static_plot" ) ]
use crate::plot::{ PlotDescription, PlotOptions, plot };
use rand::seq::IteratorRandom;
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

/// Initial sudoku.
#[ derive( Debug ) ]
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

impl SeederOperator for SudokuInitial
{
  type Person = SudokuPerson;
  type Context = Board;
  
  fn initial_generation( &self, hrng : Hrng, size : usize ) -> Vec< SudokuPerson >
  {
    let mut population = Vec::new();
    for _ in 0..size
    {
      population.push( SudokuPerson::new( &self.board, hrng.clone() ) );
    }
    population
  }

  fn initial_temperature( &self, hrng : Hrng ) -> Temperature
  {
    use statrs::statistics::Statistics;
    let state = SudokuPerson::new( &self.board, hrng.clone() );
    const N : usize = 16;
    let mut costs : [ f64 ; N ] = [ 0.0 ; N ];
    for i in 0..N
    {
      let state2 = state.mutate_random( &self.board, hrng.clone() );
      costs[ i ] = state2.cost.into();
    }
    costs[..].std_dev().into()
  }

  fn context( &self ) -> &Board
  {
    &self.board
  }
  
}

/// Represents hybrid optimization method with both Simulated Annealing and Genetic Algorithm.
#[ derive( Debug ) ]
pub struct HybridOptimizer< S : SeederOperator, C, M >
{

  /// Max amount of mutations in generation.
  pub sa_mutations_per_generation_limit : usize,

  /// Max allowed number of resets.
  pub reset_limit : usize,

  /// Temperature update operator.
  pub sa_temperature_schedule : Box< dyn TemperatureSchedule >,

  /// Number of fittest individuals that will be cloned to new population.
  pub ga_elite_selection_rate : f64,

  /// Probabilistic measure of a individual mutation likelihood.
  pub mutation_rate : f64,

  /// Recalculate fitness on every iteration.
  pub fitness_recalculation : bool,

  /// Max number of iteration without improvement in population.
  pub ga_max_stale_iterations : usize,

  /// Crossover genetic operator, which defines how new Individuals are produced by combiniting traits of Individuals from current generation.
  pub ga_crossover_operator : C,

  /// Selection genetic operator, which defines how Individuals from current generation are selected to be breeders of new generation.
  pub ga_selection_operator : Box< dyn SelectionOperator< < S as SeederOperator >::Person > >,

  /// Hierarchical random numbers generator.
  pub hrng : Hrng,

  /// Struct responsible for creation of initial generation.
  pub seeder : S,

  /// Percent of population selected for next cycle of optimization.
  //pub population_percent : f64,

  /// Max number of generations, termination condition.
  pub generation_limit : usize,

  /// Number of Individuals in initial generation of solutions.
  pub population_size : usize,

  pub mutation_operator : M,

  pub temperature : Temperature,
}

impl< S : SeederOperator, C : CrossoverOperator::< Person = < S as SeederOperator>::Person >, M > HybridOptimizer< S, C, M >
where M : MutationOperator::< Person = < S as SeederOperator>::Person > + Sync,
  M : MutationOperator::< Context = < S as SeederOperator>::Context > + Sync
{
  /// Create new instance of HybridOptimizer with default config for SA and GA.
  pub fn new( random_seed : Seed, population_seeder : S, crossover_op : C, mutation_op : M ) -> Self
  where gen_alg::TournamentSelection : gen_alg::SelectionOperator< < S as gen_alg::SeederOperator >::Person >
  {
    let selection_operator = Box::new( TournamentSelection
    {
      size : 2,
      selection_pressure : 0.85,
    } );

    let hrng = Hrng::master_with_seed( random_seed );
    let start_temp = population_seeder.initial_temperature( hrng.clone() );
    Self
    {
      sa_temperature_schedule : Box::new( LinearTempSchedule
      {
        coefficient : ( 1.0 - TemperatureFactor::default().unwrap() ).into(),
        constant : 0f64.into(),
        reset_increase_value : 1f64.into()
      } ),
      ga_max_stale_iterations : 20,
      sa_mutations_per_generation_limit : 2_000,
      reset_limit : 1_000,
      ga_elite_selection_rate : 0.25,
      fitness_recalculation : false,
      mutation_rate : 0.5,
      ga_crossover_operator : crossover_op,
      ga_selection_operator : selection_operator as Box<dyn SelectionOperator< < S as SeederOperator >::Person > >,
      hrng,
      seeder : population_seeder,
      generation_limit : 10_000,
      population_size : 10_000,
      temperature : start_temp,
      mutation_operator : mutation_op,
    }
  }

  /// Set temperature schedule for SA.
  pub fn set_sa_temp_schedule( mut self, schedule : Box< dyn TemperatureSchedule > ) -> Self
  {
    self.sa_temperature_schedule = schedule;
    self
  }

  /// Set max amount of mutations per one generation.
  pub fn set_sa_max_mutations_per_generation( mut self, number : usize ) -> Self
  {
    self.sa_mutations_per_generation_limit = number;
    self
  }

  /// Set mutation rate for GA.
  pub fn set_ga_mutation_rate( mut self, rate : f64 ) -> Self
  {
    self.mutation_rate = rate;
    self
  }

  /// Set percent of most fit Individuals that will be cloned to next generation.
  pub fn set_ga_elite_selection_rate( mut self, rate : f64 ) -> Self
  {
    self.ga_elite_selection_rate = rate;
    self
  }

  /// Perform hybrid SA/GA optimization.
  pub fn optimize( &mut self ) -> ( Reason, Option< < S as SeederOperator >::Person > )
  {
    let mut generation = self.seeder.initial_generation( self.hrng.clone(), self.population_size );
    let mut generation_number = 1;
    let mut stale_generations = 0;
    let mut prev_fitness = generation[ 0 ].fitness();

    loop
    {
      if generation_number > self.generation_limit
      {
        return ( Reason::GenerationLimit, None );
      }

      let mut new_generation = Vec::new();
      generation.sort_by( | p1, p2 | p1.fitness().cmp( &p2.fitness() ) );

      if self.population_has_solution( &generation )
      {
        return ( Reason::GoodEnough, Some( generation[ 0 ].clone() ) );
      }
      
      if generation[ 0 ].fitness() != prev_fitness
      {
        stale_generations = 0;
        prev_fitness = generation[ 0 ].fitness();
      }
      else
      {
        stale_generations += 1;
      }

      if stale_generations > self.ga_max_stale_iterations
      {
        self.temperature = self.sa_temperature_schedule.reset_temperature( self.temperature );
      }

      for i in 0..generation.len()
      {
        new_generation.push( self.evolve( generation[ i ].clone(), &generation ) );
        if new_generation.last().unwrap().is_optimal()
        {
          break;
        }
      }
      self.temperature = self.sa_temperature_schedule.calculate_next_temp( self.temperature );

      generation = new_generation;
      generation_number += 1;
    }
  }

  fn population_has_solution( &self, population : &Vec< < S as SeederOperator >::Person > ) -> bool
  {
    for person in population
    {
      if person.is_optimal()
      {
        return true;
      }
    }
    false
  }

  fn evolve
  ( 
    &self, 
    person : < S as SeederOperator >::Person, 
    population : &Vec< < S as SeederOperator >::Person >,
  ) -> < S as SeederOperator >::Person
  {
    let mut child =
    if population.iter().position( | p | *p == person ).unwrap() <= ( population.len() as f64 * self.ga_elite_selection_rate ) as usize
    {
      person.clone()
    }
    else 
    {
      let parent1 = self.ga_selection_operator.select( self.hrng.clone(), &population );
      let parent2 = self.ga_selection_operator.select( self.hrng.clone(), &population );
      self.ga_crossover_operator.crossover( self.hrng.clone(), parent1, parent2 )
    };
 
    let rng_ref = self.hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();
    let rand : f64 = rng.gen();
    drop( rng );

    if rand < self.mutation_rate
    {

    let mut n_mutations : usize = 0;
    let mut expected_number_of_mutations = 4;

    loop
    {
      if n_mutations > self.sa_mutations_per_generation_limit
      {
        {
          return person.clone();
        }
      }
  
      let hrng = self.hrng.clone();
      let mutation_op = &self.mutation_operator;
      let temperature = self.temperature;
      let mutation_context = self.seeder.context();

      let candidates = rayon::iter::repeat( () )
      .take( expected_number_of_mutations )
      .enumerate()
      .map( | ( i, _ ) | hrng.child( i ) )
      .flat_map( | hrng | 
        {
          let mut candidate = child.clone();
          mutation_op.mutate( hrng.clone(), &mut candidate, mutation_context );
      
          let rng_ref = hrng.rng_ref();
          let mut rng = rng_ref.lock().unwrap();
      
          let cost_difference = 0.5 + candidate.fitness() as f64 - child.fitness() as f64;
          let threshold = ( - cost_difference / temperature.unwrap() ).exp();
      
          log::trace!
          (
            "cost : {}  | cost_difference : {cost_difference} | temperature : {}",
            person.fitness(),
            temperature,
          );
          let rand : f64 = rng.gen();
          let vital = rand < threshold;  
          if vital
          {
            let emoji = if cost_difference > 0.0
            {
              "🔼"
            }
            else if cost_difference < 0.0
            {
              "✔️"
            }
            else
            {
              "🔘"
            };
            log::trace!( " {emoji} vital | rand( {rand} ) < threshold( {threshold} )" );
            if cost_difference == 0.0
            {
              // sleep();
            }
            Some( candidate )
          }
          else
          {
            log::trace!( " ❌ non-vital | rand( {rand} ) > threshold( {threshold} )" );
            None
          }
            
        } )
        .collect::< Vec< _ > >()
        ;

        if candidates.len() > 0
        {
          let rng_ref = self.hrng.rng_ref();
          let mut rng = rng_ref.lock().unwrap();
          
          if let Some( index ) = ( 0..candidates.len() - 1 ).choose( &mut *rng )
          {
            child = candidates[ index ].clone();
          }
          else 
          {
            child = candidates[ 0 ].clone();
          }
          break;
        }

      n_mutations += expected_number_of_mutations;
      if expected_number_of_mutations < 32
      {
        expected_number_of_mutations += 4;
      }
    }
  }

    if self.fitness_recalculation
    {
      child.update_fitness();
    }

    child
  }
}
