use std::collections::HashMap;

use deterministic_rand::Seed;
use iter_tools::Itertools;
use crate::
{ 
  sudoku::*, 
  optimization::SudokuInitial,
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
      .simplex_size( vec![ 0.002, 0.2, 200.0 ] )
      .set_improvement_threshold( 10.0 )
      .set_max_no_improvement_steps( 5 )
      .set_max_iterations( 25 )
      ;
      let res = optimizer.optimize
      (
        | case : Point |
        {
          let mut initial = SudokuInitial::new( board.clone(), Seed::default() );
          initial.set_temp_decrease_factor( case.coords[ 0 ] );
          initial.set_temp_increase_factor( case.coords[ 1 ] );
          initial.set_mutations_per_generation( case.coords[ 2 ] as usize );
          
          let mut results: Vec< std::time::Duration > = Vec::new();
          for _ in 0..3
          {
            let now = std::time::Instant::now();
            let ( _reason, _generation ) = initial.solve_with_sa();
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
            acc.0 + elem.0.coords[ 0 ] / size, 
            acc.1 + elem.0.coords[ 1 ] / size, 
            acc.2 + elem.0.coords[ 2 ] / size, 
          )
        ),
      );
    }
    // println!( "Average: {:?}", level_average );
  }

  //check improvement

  for level in Level::iterator()
  {
    for board in control_boards.get( &level ).unwrap()
    {
      // initial
      let mut initial = SudokuInitial::new( board.clone(), Seed::default() );
      let now = std::time::Instant::now();
      let ( _reason, _generation ) = initial.solve_with_sa();
      let elapsed = now.elapsed();

      // optimized
      initial = SudokuInitial::new( board.clone(), Seed::default() );
      let optimized_params = level_average.get( &level ).unwrap();
      initial.set_temp_decrease_factor( 0.0023 );
      initial.set_temp_increase_factor( optimized_params.1 );
      initial.set_mutations_per_generation( optimized_params.2 as usize );
      
      let now = std::time::Instant::now();
      let ( _reason, _generation ) = initial.solve_with_sa();
      let elapsed = now.elapsed();
    }
  }
}
