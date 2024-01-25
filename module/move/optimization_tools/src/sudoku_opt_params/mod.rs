//! Funcions for calculation optimal config parameters.
//! 

use std::collections::HashMap;
use deterministic_rand::Seed;
use iter_tools::Itertools;
use crate::
{ 
  sudoku::*, 
  optimization::{ HybridOptimizer, LinearTempSchedule, SudokuInitial, BestRowsColumnsCrossover, RandomPairInBlockMutation },
  nelder_mead::{ NelderMeadOptimizer, Point, NMResult },
};

mod sudoku_sets;

/// Level of difficulty of sudoku board.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Hash ) ]
pub enum Level
{
  /// Easy level with difficulty <= 2.
  Easy,
  /// Medium, 2 < difficulty <= 2.5.
  Medium,
  /// Hard level, 2.5 < difficulty <= 3.
  Hard,
  /// Expert level with difficulty > 3.
  Expert,
}

impl Level {
  /// Iterates over sudoku difficulty levels.
  pub fn iterator() -> impl Iterator< Item = Level > 
  {
    use Level::*;
    [ Easy, Medium, Hard, Expert ].iter().copied()
  }
}

/// Calculate optimal params for SA optimization.
pub fn get_sa_optimal_params()
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
      .starting_point( Point::new( vec![ 0.999, 1.0, 2000.0 ] ) )
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
          let temp_schedule = LinearTempSchedule
          {
            constant : 0.0.into(),
            coefficient : case.coords[ 0 ].into(),
            reset_increase_value : case.coords[ 1 ].into(),
          };

          let mut optimizer = HybridOptimizer::new( Seed::default(), initial, BestRowsColumnsCrossover{}, RandomPairInBlockMutation{} )
          .set_sa_temp_schedule( Box::new( temp_schedule ) )
          .set_sa_max_mutations_per_generation( case.coords[ 2 ] as usize )
          ;

          let mut results: Vec< std::time::Duration > = Vec::new();
          for _ in 0..3
          {
            let now = std::time::Instant::now();
            let ( _reason, _generation ) = optimizer.optimize();
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
      let mut optimizer = HybridOptimizer::new( Seed::default(), initial, BestRowsColumnsCrossover{}, RandomPairInBlockMutation{} );

      let now = std::time::Instant::now();
      let ( _reason, _generation ) = optimizer.optimize();
      let elapsed = now.elapsed();

      // optimized
      let optimized_params = level_average.get( &level ).unwrap();
      let initial = SudokuInitial::new( board.clone() );
      let temp_schedule = LinearTempSchedule
      {
        constant : 0.0.into(),
        coefficient : optimized_params.0.into(),
        reset_increase_value : optimized_params.1.into(),
      };

      let mut optimizer = HybridOptimizer::new( Seed::default(), initial, BestRowsColumnsCrossover{}, RandomPairInBlockMutation{} )
      .set_sa_temp_schedule( Box::new( temp_schedule ) )
      .set_sa_max_mutations_per_generation( optimized_params.2 as usize )
      ;
      
      let now = std::time::Instant::now();
      let ( _reason, _generation ) = optimizer.optimize();
      let opt_elapsed = now.elapsed();
      let res = elapsed.as_millis() as i128 - opt_elapsed.as_millis() as i128;
      results.push( res );
    }
    level_improvement.insert( level, results );
  }
}

/// Calculate optimal params for GA optimization.
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
      .starting_point( Point::new( vec![ 0.25, 0.5 ] ) )
      .unwrap()
      .simplex_size( vec![ 0.1, 0.2 ] )
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

          let mut optimizer = HybridOptimizer::new( Seed::default(), initial, BestRowsColumnsCrossover{}, RandomPairInBlockMutation{}  )
          .set_ga_elite_selection_rate( case.coords[ 0 ] )
          .set_ga_mutation_rate( case.coords[ 1 ] )
          ;
          
          let mut results: Vec< std::time::Duration > = Vec::new();
          for _ in 0..3
          {
            let now = std::time::Instant::now();
            let ( _reason, _generation ) = optimizer.optimize();
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

/// Calculate optimal params for hybrid optimization.
pub fn hybrid_optimal_params() -> Vec< ( Level, Vec< NMResult > ) >
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
      .starting_point( Point::new( vec![ 0.999, 1.0, 2000.0, 0.25, 0.5 ] ) )
      .unwrap()
      .simplex_size( vec![ 0.002, 0.2, 200.0, 0.1, 0.2 ] )
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
          let temp_schedule = LinearTempSchedule
          {
            constant : 0.0.into(),
            coefficient : case.coords[ 0 ].into(),
            reset_increase_value : case.coords[ 1 ].into(),
          };

          let mut optimizer = HybridOptimizer::new( Seed::default(), initial, BestRowsColumnsCrossover{}, RandomPairInBlockMutation{}  )
          .set_sa_temp_schedule( Box::new( temp_schedule ) )
          .set_sa_max_mutations_per_generation( case.coords[ 2 ] as usize )
          .set_ga_elite_selection_rate( case.coords[ 3 ] )
          .set_ga_mutation_rate( case.coords[ 4 ] )
          ;
          
          let mut results: Vec< std::time::Duration > = Vec::new();
          for _ in 0..3
          {
            let now = std::time::Instant::now();
            let ( _reason, _generation ) = optimizer.optimize();
            let elapsed = now.elapsed();
            results.push( elapsed );
          }
          let size = results.len() as u128;
          let average = results
          .into_iter()
          .fold( 0, | acc, elem | acc + elem.as_millis() / size )
          ;
          
          log::info!
              (
                "point : {:?}, duration in ms : {}",
                case,
                average
              );
          average as f64
        }, 
      );
      let results = level_results.get_mut( &level ).unwrap();
      results.push( res );
    }

    log::info!( "results: {:?}", level_results );
  }
  level_results.into_iter().map( | ( l, res ) | ( *l, res ) ).collect_vec()
}