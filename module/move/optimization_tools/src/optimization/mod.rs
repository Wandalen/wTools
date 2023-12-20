use crate::*;
#[ cfg( feature="static_plot" ) ]
use crate::plot::{ PlotDescription, PlotOptions, plot };
use sudoku::{ Board, BlockIndex, CellIndex };
use deterministic_rand::Seed;
// use log::*;

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
      if fixed == 0 || fixed >= 9
      {
        return false;
      }
    }
    true
  }

}

/// Get a pair of random non-fixed cells in a specified block.
pub fn cells_pair_random_in_block( initial : &Board, block : BlockIndex, hrng : Hrng ) -> ( CellIndex, CellIndex )
{

  debug_assert!( initial.validate_each_block_has_non_fixed_cell() );

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

  ( cell1, cell2 )
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

/// Represents temperature of SA process.
#[ derive( Default, Debug, Display, Clone, Copy, PartialEq, PartialOrd, FromInner, InnerFrom ) ]
#[ derive( Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign ) ]
pub struct Temperature( f64 );

impl Temperature
{
  /// Returns inner value of Temperature struct.
  pub fn unwrap( &self ) -> f64
  {
    self.0
  }
}

/// Transforms Temperature value into f64.
impl From< f32 > for Temperature
{
  #[ inline ]
  fn from( src : f32 ) -> Self
  {
    Self( src as f64 )
  }
}

/// Struct that represents coefficient to change temperature value.
#[ derive( Debug, Display, Clone, Copy, PartialEq, PartialOrd, FromInner, InnerFrom ) ]
#[ derive( Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign ) ]
pub struct TemperatureFactor( f64 );

impl TemperatureFactor
{
  /// Returns inner value of TemperatureFactor struct.
  pub fn unwrap( &self ) -> f64
  {
    self.0
  }
}

/// Default value of TemperatureFactor struct.
impl Default for TemperatureFactor
{
  fn default() -> Self
  {
    0.001.into()
  }
}

/// Transforms f32 value into TemperatureFactor.
impl From< f32 > for TemperatureFactor
{
  #[ inline ]
  fn from( src : f32 ) -> Self
  {
    Self( src as f64 )
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
  pub fn new( initial : &SudokuInitial ) -> Self
  {
    let mut board = initial.board.clone();
    board.fill_missing_randomly( initial.hrng.clone() );
    let cost : SudokuCost = board.total_error().into();
    SudokuPerson { board, cost }
  }

  /// Change state of the board by applying provided mutagen to current sudoku board.
  pub fn mutate( &self, _initial : &SudokuInitial, mutagen : &SudokuMutagen ) -> Self
  {
    let mut new = self.clone();
    log::trace!( "cells_swap( {:?}, {:?} )", mutagen.cell1, mutagen.cell2 );
    new.board.cells_swap( mutagen.cell1, mutagen.cell2 );
    new.cost -= self.board.cross_error( mutagen.cell1 ).into();
    new.cost -= self.board.cross_error( mutagen.cell2 ).into();
    new.cost += new.board.cross_error( mutagen.cell1 ).into();
    new.cost += new.board.cross_error( mutagen.cell2 ).into();
    new
  }

  /// Create random mutagen and apply it current board.
  pub fn mutate_random( &self, initial : &SudokuInitial, hrng : Hrng ) -> Self
  {
    let mutagen = self.mutagen( initial, hrng );
    self.mutate( &initial, &mutagen.into() )
  }

  /// Create new SudokuMutagen as random cells pair in random sudoku block in current board.
  pub fn mutagen( &self, initial : &SudokuInitial, hrng : Hrng ) -> SudokuMutagen
  {
    let rng_ref = hrng.rng_ref();
    let mut rng = rng_ref.lock().unwrap();
    let block : BlockIndex = rng.gen();
    drop( rng );
    let mutagen = cells_pair_random_in_block( &initial.board, block, hrng );
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

/// Represents initial configuration of SA optimization process for sudoku solving.
#[ derive( Clone, Debug ) ]
pub struct SudokuInitial
{
  /// Initial state of sudoku board with fixed values.
  pub board : Board,
  /// Seed for random numbers generator.
  pub seed : Seed,
  /// Random numbers generator used for creating new state of SA.
  pub hrng : Hrng,
  /// Max amount of mutations in generation.
  pub n_mutations_per_generation_limit : usize,
  /// Max allowed number of resets.
  pub n_resets_limit : usize,
  /// Max number of generations created during SA process.
  pub n_generations_limit : usize,
  /// Coefficient for lowering SA temperature.
  pub temperature_decrease_factor : TemperatureFactor,
  /// Coefficient for increasing SA temperature during reset.
  pub temperature_increase_factor : TemperatureFactor,
}

// impl Default for SudokuInitial
// {
//   fn default() -> Self
//   {
//     let board = Default::new();
//     let seed = Default::new();
//     let hrng = Hrng::master_with_seed( seed.clone() );
//     let temperature_decrease_factor = Default::new();
//   }
// }

impl SudokuInitial
{
  /// Create new initial state for SA.
  pub fn new( board : Board, seed : Seed ) -> Self
  {
    let hrng = Hrng::master_with_seed( seed.clone() );
    let temperature_decrease_factor = Default::default();
    let temperature_increase_factor = 1.0f64.into(); // xxx
    let n_mutations_per_generation_limit = 2_000; // xxx
    let n_resets_limit = 1_000; // xxx
    let n_generations_limit = 1_000_000;
    Self
    {
      board,
      seed,
      hrng,
      n_mutations_per_generation_limit,
      n_resets_limit,
      n_generations_limit,
      temperature_decrease_factor,
      temperature_increase_factor,
    }
  }

  /// Create the initial generation for the simulated annealing algorithm.
  pub fn initial_generation< 'initial >( &'initial self ) -> SudokuGeneration < 'initial >
  {
    let person = SudokuPerson::new( self );
    let temperature = self.initial_temperature();
    let hrng = self.hrng.clone();
    let n_resets = 0;
    let n_generation = 0;
    SudokuGeneration { initial : self, hrng, person, temperature, n_resets, n_generation }
  }

  /// Calculate the initial temperature for the optimization process.
  pub fn initial_temperature( &self ) -> Temperature
  {
    use statrs::statistics::Statistics;
    let state = SudokuPerson::new( self );
    const N : usize = 16;
    let mut costs : [ f64 ; N ] = [ 0.0 ; N ];
    for i in 0..N
    {
      let state2 = state.mutate_random( self, self.hrng.clone() );
      costs[ i ] = state2.cost.into();
    }
    costs[..].std_dev().into()
  }

  /// Main loop for solving sudoku with simulated annealing. Returns reason that inidicates why loop exited and solved sudoku if optimization was successful.
  pub fn solve_with_sa( &self ) -> ( Reason, Option< SudokuGeneration < '_ > > )
  {
    let mut generation = self.initial_generation();
    // let mut n_generation : usize = 0;

    // xxx : optimize, make sure it use not more than 2 enitties of generation
    loop
    {
      // n_generation += 1;
      if generation.n_generation > self.n_generations_limit
      {
        return ( Reason::GenerationLimit, None );
      }

      log::trace!( "\n= n_generation : {}\n", generation.n_generation );

      // log::trace!( "\n= n_generation : {n_generation}\n" );
      // println!( "max_level : {}", log::max_level() );


      let ( reason, generation2 ) = generation.mutate( generation.hrng.clone() );
      if generation2.is_none()
      {
        return ( reason, None );
      }
      let generation2 = generation2.unwrap();

      //plotting
      // #[ cfg( feature="static_plot" ) ]
      // {
      //   let options = PlotOptions 
      //   {
      //     x : generation.n_generation as f32,
      //     y : generation.person.cost.0 as f32,
      //     name : String::from( "Cost change" ),
      //     description : PlotDescription
      //     {
      //       x_label : String::from( "Step" ),
      //       y_label : String::from( "Cost" ),
      //       filename : String::from( "cost_plot" ),
      //       ..Default::default()
      //     }
      //   };
      //   plot( options );

      // }

      // #[ cfg( feature="dynamic_plot" ) ]
      // {
      //   let options = PlotOptions 
      //   {
      //     x : generation.n_generation as f32,
      //     y : generation.person.cost.0 as f32,
      //     name : String::from( "Cost change" ),
      //     description : PlotDescription
      //     {
      //       x_label : String::from( "Step" ),
      //       y_label : String::from( "Cost" ),
      //       filename : String::from( "cost_plot" ),
      //       ..Default::default()
      //     }
      //   };
      //   plot_dynamic::dyn_plot( options );
      // }

      // #[ cfg( feature="static_plot" ) ]
      // {
      //   let options = PlotOptions 
      //   {
      //     x : generation.n_generation as f32,
      //     y : generation.temperature.unwrap() as f32,
      //     name : String::from( "Temperature change" ),
      //     description : PlotDescription
      //     {
      //       x_label : String::from( "Step" ),
      //       y_label : String::from( "Temperature" ),
      //       filename : String::from( "temp_plot" ),
      //       ..Default::default()
      //     }
      //   };

      //   plot( options );
      // }

      if generation2.is_good_enough()
      {
        return ( Reason::GoodEnough, Some( generation2 ) );
      }

      generation = generation2;
    }
  }

}

/// Represents a state in the Simulated Annealing optimization process for solving Sudoku.
#[ derive( Clone, Debug ) ]
pub struct SudokuGeneration< 'a >
{
  /// Initial configuration of the Sudoku puzzle.
  initial : &'a SudokuInitial,
  /// Random number generator for generating new state.
  hrng : Hrng,
  /// Current state of sudoku board.
  pub person : SudokuPerson,
  /// Current temperature in the optimization process.
  temperature : Temperature,
  /// Number of resets performed.
  n_resets : usize,
  /// Amount of generations before current genetration.
  n_generation : usize,
}

impl< 'a > SudokuGeneration< 'a >
{
  /// Performs single iteration of optimization process, returns a tuple containing the reason to stop or continue optimization process and the new Sudoku generation if successful.
  pub fn mutate( &self, hrng : Hrng ) -> ( Reason, Option< Self > )
  {
    let initial = self.initial;
    let mut temperature = self.temperature;
    let mut n_mutations : usize = 0;
    let mut n_resets : usize = self.n_resets;

    let person = loop
    {

      if n_mutations > initial.n_mutations_per_generation_limit
      {
        n_resets += 1;
        if n_resets >= initial.n_resets_limit
        {
          return ( Reason::ResetLimit, None );
        }
        let temperature2 = ( temperature.unwrap() + initial.temperature_increase_factor.unwrap() ).into();
        log::trace!( " 🔄 reset temperature {temperature} -> {temperature2}" );
        sleep();
        temperature = temperature2;
        n_mutations = 0;
      }

      let mutagen = self.person.mutagen( initial, hrng.clone() );
      let person = self.person.mutate( initial, &mutagen );

      let rng_ref = hrng.rng_ref();
      let mut rng = rng_ref.lock().unwrap();

      let cost_difference = 0.5 + person.cost.unwrap() as f64 - self.person.cost.unwrap() as f64;
      let threshold = ( - cost_difference / temperature.unwrap() ).exp();

      log::trace!
      (
        "cost : {} -> {} | cost_difference : {cost_difference} | temperature : {temperature}",
        self.person.cost,
        person.cost,
      );
      let rand : f64 = rng.gen();
      let vital = rand < threshold;

      //plotting
      // #[ cfg( feature="static_plot" ) ]
      // {
      //   let accept = if threshold > 1.0 { 1.0 } else { threshold };
      //   let options = PlotOptions 
      //   {
      //     x : self.n_generation as f32,
      //     y : accept as f32,
      //     name : String::from( "Treshold" ),
      //     description : PlotDescription
      //     {
      //       x_label : String::from( "Step" ),
      //       y_label : String::from( "Acceptance probability" ),
      //       filename : String::from( "ac_prob_plot" ),
      //       plot_line : false,
      //       y_log_coords : false,
      //       ..Default::default()
      //     }
      //   };
      //   plot( options );
      // }

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
      }
      else
      {
        log::trace!( " ❌ non-vital | rand( {rand} ) > threshold( {threshold} )" );
      }


      // info!( target = threshold ); xxx

      if vital
      {
        break person;
      }

      n_mutations += 1;
    };

    temperature = Temperature::from( temperature.unwrap() * ( 1.0f64 - self.initial.temperature_decrease_factor.unwrap() ) );
    let n_generation = self.n_generation + 1;

    let generation = SudokuGeneration { initial, hrng, person, temperature, n_resets, n_generation };

    ( Reason::NotFinished, Some( generation ) )
  }

  /// Checks if the current state is considered good enough as a solution.
  pub fn is_good_enough( &self ) -> bool
  {
    self.person.cost == 0.into()
  }

}
