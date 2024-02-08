//! Funcions for calculation optimal config parameters.
//! 

use std::ops::RangeBounds;
use deterministic_rand::Seed;
use iter_tools::Itertools;
use crate::
{ 
  sudoku::*, 
  optimization::{ HybridOptimizer, LinearTempSchedule, SudokuInitial, BestRowsColumnsCrossover, RandomPairInBlockMutation, PopulationModificationProportions },
  nelder_mead::{ NelderMeadOptimizer, Point, Solution },
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

pub struct OptimalParamsConfig
{
  improvement_threshold : f64,
  max_no_improvement_steps : usize,
  max_iterations : usize,
}

impl Default for OptimalParamsConfig
{
  fn default() -> Self 
  {
    Self 
    {
      improvement_threshold : 10.0,
      max_no_improvement_steps : 5,
      max_iterations : 25,
    }
  }
} 

pub struct OptimalProblem< F : Fn( Point ) -> f64, R : RangeBounds< f64 > >
{
  pub bounds : Vec< Option< R > >,
  pub starting_point : Option< Point >,
  pub simplex_size : Vec< f64 >,
  pub obj_function : F
}

/// Calculate optimal params for hybrid optimization.
pub fn hybrid_optimal_params< F, R >( config : OptimalParamsConfig, problem : OptimalProblem< F, R > ) -> Vec< Solution >
where F : Fn( Point ) -> f64, R : RangeBounds< f64 >
{
  let mut results = Vec::new();

  let mut optimizer: NelderMeadOptimizer<std::ops::Range< f64 >> = NelderMeadOptimizer::default();
  // optimizer.set_bounds( problem.bounds )
  // .starting_point( problem.starting_point )
  // .simplex_size( problem.simplex_size )
  // ;
  let res = optimizer.optimize
  (
    problem.obj_function
  );
  results.push( res );
    
  log::info!( "results: {:?}", results );
  results.into_iter().filter_map( | res | res.ok() ).collect_vec()
}
