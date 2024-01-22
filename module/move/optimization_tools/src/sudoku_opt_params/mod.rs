use std::collections::HashMap;

use deterministic_rand::Seed;
use iter_tools::Itertools;
use crate::
{ 
  sudoku::*, 
  optimization::{SudokuInitial, HybridOptimizer, SAConfig, GAConfig, HybridStrategy, StrategyMode},
  nelder_mead::{NelderMeadOptimizer, Point},
};

mod sudoku_sets;

/// Level of difficulty of sudoku board.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Hash ) ]
pub enum Level
{
  Easy,
  Medium,
  Hard,
  Expert,
}

impl Level {
  pub fn iterator() -> impl Iterator< Item = Level > 
  {
    use Level::*;
    [ Easy, Medium, Hard, Expert ].iter().copied()
  }
}

pub fn get_optimal_params()
{
  let mut boards = HashMap::new();
  let mut control_boards = HashMap::new();

  for ( index, level ) in Level::iterator().enumerate()
  {
    boards.insert( level, sudoku_sets::TRAINING[ index ].iter().map( | str | Board::from( str ) ).collect_vec() );
    control_boards.insert( level, sudoku_sets::CONTROL[ index ].iter().map( | str | Board::from( str ) ).collect_vec() );
  }

  for ( _level, level_boards ) in &boards
  {
    let mut diff_coeffs = Vec::new();
    for board in level_boards
    {
      diff_coeffs.push( board.calculate_difficulty() );
    }
  }

  let mut level_average = HashMap::new();
  
  for ( level, level_boards ) in &boards
  {
    let mut level_results = HashMap::new();
    level_results.insert( level, Vec::new() );

    for board in level_boards
    {  
      let optimizer = NelderMeadOptimizer::new()
      .starting_point( Point::new( vec![ 0.001, 1.0, 2000.0 ] ) )
      .unwrap()
      .simplex_size( vec![ 0.002, 0.2, 200.0 ] )
      .unwrap()
      .set_improvement_threshold( 10.0 )
      .set_max_no_improvement_steps( 5 )
      .set_max_iterations( 25 )
      ;
      let res = optimizer.optimize
      (
        | case : Point |
        {
          
          let initial = SudokuInitial::new( board.clone() );
          let mut sa_config = SAConfig::default();
          sa_config.set_temp_decrease_factor( case.coords[ 0 ] );
          sa_config.set_temp_increase_factor( case.coords[ 1 ] );
          sa_config.set_mutations_per_generation( case.coords[ 2 ] as usize );
          let optimizer = HybridOptimizer::new( Seed::default(), initial )
          .with_sa_config( sa_config )
          ;

          let strategy = HybridStrategy
          {
            start_with : StrategyMode::SA,
            finalize_with : StrategyMode::SA,
            number_of_cycles : 1,
            ga_generations_number : 0,
            sa_generations_number : 1000,
            population_percent : 1.0,
            generation_limit : 100_000_000,
            population_size : 1,
          };

          let mut results: Vec< std::time::Duration > = Vec::new();
          for _ in 0..3
          {
            let now = std::time::Instant::now();
            let ( _reason, _generation ) = optimizer.optimize( &strategy );
            let elapsed = now.elapsed();
            results.push( elapsed );
          }
          let size = results.len() as u128;
          let average = results
          .into_iter()
          .fold( 0, | acc, elem | acc + elem.as_millis() / size )
          ;
          average as f64
        }, 
      );
      //println!( "{}: {:?} : {:?}", level, res.0, res.1 );
      let results = level_results.get_mut( &level ).unwrap();
      results.push( res );
    }

    for ( level, results ) in level_results
    {
      let size = results.len() as f64;
      level_average.insert
      ( 
        level,  
        results.iter().fold
        ( 
          ( 0.0, 0.0, 0.0 ), 
          | acc, elem | 
          ( 
            acc.0 + elem.point.coords[ 0 ] / size, 
            acc.1 + elem.point.coords[ 1 ] / size, 
            acc.2 + elem.point.coords[ 2 ] / size, 
          )
        ),
      );
    }
    // println!( "Average: {:?}", level_average );
  }

  //check improvement
  let mut level_improvement = HashMap::new();
  for level in Level::iterator()
  {
    let mut results = Vec::new();
    for board in control_boards.get( &level ).unwrap()
    {
      // initial
      let initial = SudokuInitial::new( board.clone() );
      let optimizer = HybridOptimizer::new( Seed::default(), initial );
      let strategy = HybridStrategy
      {
        start_with : StrategyMode::SA,
        finalize_with : StrategyMode::SA,
        number_of_cycles : 1,
        ga_generations_number : 0,
        sa_generations_number : 1000,
        population_percent : 1.0,
        generation_limit : 100_000_000,
        population_size : 1,
      };
      let now = std::time::Instant::now();
      let ( _reason, _generation ) = optimizer.optimize( &strategy );
      let elapsed = now.elapsed();

      // optimized
      let optimized_params = level_average.get( &level ).unwrap();
      let initial = SudokuInitial::new( board.clone() );
      let mut sa_config = SAConfig::default();
      sa_config.set_temp_decrease_factor( optimized_params.0 );
      sa_config.set_temp_increase_factor( optimized_params.1 );
      sa_config.set_mutations_per_generation( optimized_params.2 as usize );
      let optimizer = HybridOptimizer::new( Seed::default(), initial )
      .with_sa_config( sa_config )
      ;
      
      let now = std::time::Instant::now();
      let ( _reason, _generation ) = optimizer.optimize( &strategy );
      let opt_elapsed = now.elapsed();
      let res = elapsed.as_millis() as i128 - opt_elapsed.as_millis() as i128;
      results.push( res );
    }
    level_improvement.insert( level, results );
  }
}

pub fn ga_optimal_params()
{
  let mut boards = HashMap::new();
  boards.insert( Level::Easy, sudoku_sets::TRAINING[ 0 ].iter().map( | str | Board::from( str ) ).collect_vec() );
  let mut level_results = HashMap::new();
  
  for ( level, level_boards ) in &boards
  {
    level_results.insert( level, Vec::new() );

    for board in level_boards
    {  
      let optimizer = NelderMeadOptimizer::new()
      .starting_point( Point::new( vec![ 0.25, 0.25, 0.5 ] ) )
      .unwrap()
      .simplex_size( vec![ 0.1, 0.1, 0.2 ] )
      .unwrap()
      .set_improvement_threshold( 10.0 )
      .set_max_no_improvement_steps( 5 )
      .set_max_iterations( 25 )
      ;
      let res = optimizer.optimize
      (
        | case : Point |
        {
          let initial = SudokuInitial::new( board.clone() );
          let ga_config = GAConfig::default()
          .set_elite_selection_rate( case.coords[ 0 ] )
          .set_random_selection_rate( case.coords[ 1 ] )
          .set_mutation_rate( case.coords[ 2 ] )
          ;
          let optimizer = HybridOptimizer::new( Seed::default(), initial )
          .with_ga_config( ga_config )
          ;

          let strategy = HybridStrategy
          {
            start_with : StrategyMode::GA,
            finalize_with : StrategyMode::GA,
            number_of_cycles : 1,
            ga_generations_number : 1000,
            sa_generations_number : 0,
            population_percent : 1.0,
            generation_limit : 100_000_000,
            population_size : 10000,
          };
          
          let mut results: Vec< std::time::Duration > = Vec::new();
          for _ in 0..3
          {
            let now = std::time::Instant::now();
            let ( _reason, _generation ) = optimizer.optimize( &strategy );
            let elapsed = now.elapsed();
            results.push( elapsed );
          }
          let size = results.len() as u128;
          let average = results
          .into_iter()
          .fold( 0, | acc, elem | acc + elem.as_millis() / size )
          ;
          average as f64
        }, 
      );
      let results = level_results.get_mut( &level ).unwrap();
      results.push( res );
    }
    println!( "results: {:?}", level_results );
  }
}
