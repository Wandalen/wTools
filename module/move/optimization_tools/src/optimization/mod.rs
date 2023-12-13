use crate::*;
use crate::plotting::{ PlotDescription, PlotOptions, plot };
use sudoku::{ Board, BlockIndex, CellIndex };
use deterministic_rand::Seed;
// use log::*;

pub fn sleep()
{
  std::thread::sleep( std::time::Duration::from_secs( 5 ) );
}

trait BoardExt
{

  /// Validate that each bloack has at least one non-fixed cell
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
      if fixed == 0 || fixed >= 8
      {
        return false;
      }
    }
    true
  }

}

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

#[ derive( Default, Debug, Display, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, FromInner, InnerFrom ) ]
#[ derive( Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign ) ]
pub struct SudokuCost( usize );

// xxx : derive, please
impl SudokuCost
{
  pub fn unwrap( self ) -> usize
  {
    self.0
  }
}

impl From< SudokuCost > for f64
{
  #[ inline ]
  fn from( src : SudokuCost ) -> Self
  {
    src.0 as f64
  }
}

#[ derive( Default, Debug, Display, Clone, Copy, PartialEq, PartialOrd, FromInner, InnerFrom ) ]
#[ derive( Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign ) ]
pub struct Temperature( f64 );

impl Temperature
{
  pub fn unwrap( &self ) -> f64
  {
    self.0
  }
}

impl From< f32 > for Temperature
{
  #[ inline ]
  fn from( src : f32 ) -> Self
  {
    Self( src as f64 )
  }
}

#[ derive( Debug, Display, Clone, Copy, PartialEq, PartialOrd, FromInner, InnerFrom ) ]
#[ derive( Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign ) ]
pub struct TemperatureFactor( f64 );

impl TemperatureFactor
{
  pub fn unwrap( &self ) -> f64
  {
    self.0
  }
}

impl Default for TemperatureFactor
{
  fn default() -> Self
  {
    0.001.into()
  }
}

impl From< f32 > for TemperatureFactor
{
  #[ inline ]
  fn from( src : f32 ) -> Self
  {
    Self( src as f64 )
  }
}

#[ derive( PartialEq, Eq, Clone, Copy, Debug, Display ) ]
pub enum Reason
{
  GoodEnough,
  NotFinished,
  ResetLimit,
  GenerationLimit,
}

#[ derive( PartialEq, Eq, Clone, Debug ) ]
pub struct SudokuPerson
{
  pub board : Board,
  pub cost : SudokuCost,
}

impl SudokuPerson
{

  pub fn new( initial : &SudokuInitial ) -> Self
  {
    let mut board = initial.board.clone();
    board.fill_missing_randomly( initial.hrng.clone() );
    let cost : SudokuCost = board.total_error().into();
    SudokuPerson { board, cost }
  }

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

  pub fn mutate_random( &self, initial : &SudokuInitial, hrng : Hrng ) -> Self
  {
    let mutagen = self.mutagen( initial, hrng );
    self.mutate( &initial, &mutagen.into() )
  }

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

#[ derive( PartialEq, Eq, Clone, Debug, FromInner, InnerFrom ) ]
pub struct SudokuMutagen
{
  pub cell1 : CellIndex,
  pub cell2 : CellIndex,
}

#[ derive( Clone, Debug ) ]
pub struct SudokuInitial
{
  pub board : Board,
  pub seed : Seed,
  pub hrng : Hrng,
  pub n_mutations_per_generation_limit : usize,
  pub n_resets_limit : usize,
  pub n_generations_limit : usize,
  pub temperature_decrease_factor : TemperatureFactor,
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

  pub fn initial_generation( &self ) -> SudokuGeneration
  {
    let person = SudokuPerson::new( self );
    let temperature = self.initial_temperature();
    let hrng = self.hrng.clone();
    let n_resets = 0;
    let n_generation = 0;
    SudokuGeneration { initial : self, hrng, person, temperature, n_resets, n_generation }
  }

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

  pub fn solve_with_sa( &self ) -> ( Reason, Option< SudokuGeneration > )
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
      let options = PlotOptions 
      {
        x : generation.n_generation as f32,
        y : generation.person.cost.0 as f32,
        name : String::from( "Cost change" ),
        legend : None,
        description : PlotDescription
        {
          x_label : String::from( "Step" ),
          y_label : String::from( "Cost" ),
          filename : String::from( "cost_plot" ),
          dynamic : true,
          ..Default::default()
        }
      };

      plot(options);

      // let options = PlotOptions 
      // {
      //   x : generation.n_generation as f32,
      //   y : generation.temperature.unwrap() as f32,
      //   name : String::from( "Temperature change" ),
      //   legend : None,
      //   description : PlotDescription
      //   {
      //     x_label : String::from( "Step" ),
      //     y_label : String::from( "Temperature" ),
      //     filename : String::from( "temp_plot" ),
      //     ..Default::default()
      //   }
      // };

      // plot(options);

      if generation2.is_good_enough()
      {
        return ( Reason::GoodEnough, Some( generation2 ) );
      }

      generation = generation2;
    }
  }

}

#[ derive( Clone, Debug ) ]
pub struct SudokuGeneration< 'a >
{
  initial : &'a SudokuInitial,
  hrng : Hrng,
  pub person : SudokuPerson,
  temperature : Temperature,
  n_resets : usize,
  n_generation : usize,
}

impl< 'a > SudokuGeneration< 'a >
{

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

      // plotting
      // let accept = if threshold > 1.0 { 1.0 } else { threshold };
      // let options = PlotOptions 
      // {
      //   x : self.n_generation as f32,
      //   y : accept as f32,
      //   name : String::from( "ac_probability" ),
      //   legend : None,
      //   description : PlotDescription
      //   {
      //     x_label : String::from( "Step" ),
      //     y_label : String::from( "Acceptance probability" ),
      //     filename : String::from( "probability_plot" ),
      //     plot_line : false,
      //     y_log_coords : false,
      //   }
      // };

      // plot(options);


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

  pub fn is_good_enough( &self ) -> bool
  {
    self.person.cost == 0.into()
  }

}
