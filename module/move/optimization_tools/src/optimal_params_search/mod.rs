//! Funcions for calculation optimal config parameters.
//! 
pub mod results_serialize;
pub mod nelder_mead;
pub mod sim_annealing;
use std::ops::RangeBounds;
use iter_tools::Itertools;

use crate::hybrid_optimizer::*;

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

#[ derive( Debug, Clone ) ]
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
      improvement_threshold : 0.005,
      max_no_improvement_steps : 10,
      max_iterations : 100,
    }
  }
} 

#[ derive( Debug, Clone ) ]
pub struct OptimalProblem< R : RangeBounds< f64 > >
{
  pub params_names : Vec< Option< String > >,
  pub bounds : Vec< Option< R > >,
  pub starting_point : Vec< Option< f64 > >,
  pub simplex_size : Vec< Option< f64 > >,
}

impl< 'a, R : RangeBounds< f64 > > OptimalProblem< R >
{
  pub fn new() -> Self
  {
    Self
    {
      params_names : Vec::new(),
      bounds : Vec::new(),
      starting_point : Vec::new(),
      simplex_size : Vec::new(),
    }
  }

  pub fn add
  ( 
    mut self,
    name : Option< String >, 
    bounds : Option< R >, 
    start_value : Option< f64 >, 
    simplex_size : Option< f64 >, 
  ) -> Result< Self, Error >
  {
    if let Some( ref name ) = name
    {
      if self.params_names.iter().cloned().filter_map( | n | n ).contains( name )
      {
        return Err( Error::NameError );
      }
    }

    if let Some( start_value ) = start_value
    {
      if let Some( ref bounds ) = bounds
      {
        if !bounds.contains( &start_value )
        {
          return Err( Error::OutOfBoundsError );
        }
      }
    }

    self.params_names.push( name );
    self.bounds.push( bounds );
    self.simplex_size.push( simplex_size );
    self.starting_point.push( start_value );

    Ok( self )
  }
}

/// Calculate optimal params for hybrid optimization.
pub fn find_hybrid_optimal_params< R, S, C, M >
( 
  config : OptimalParamsConfig, 
  problem : OptimalProblem< R >, 
  hybrid_problem : Problem< S, C, M > 
) -> Result< nelder_mead::Solution, nelder_mead::Error >
where  R : RangeBounds< f64 > + Sync,
  S : InitialProblem + Sync + Clone, 
  C : CrossoverOperator::< Person = < S as InitialProblem>::Person > + Clone + Sync,
  M : MutationOperator::< Person = < S as InitialProblem >::Person > + Sync,
  M : MutationOperator::< Problem = S > + Sync + Clone,
  TournamentSelection: SelectionOperator<<S as InitialProblem>::Person>
{
  let seeder = hybrid_problem.seeder.clone();
  let ga_crossover_operator = hybrid_problem.ga_crossover_operator.clone();
  let mutation_operator = hybrid_problem.mutation_operator.clone();

  let objective_function = | case : nelder_mead::Point |
  {
    log::info!
    (
      "temp_decrease_coefficient : {:.4?}, max_mutations_per_dynasty: {}, mutation_rate: {:.2}, crossover_rate: {:.2};",
      case.coords[ 0 ], case.coords[ 1 ] as usize, case.coords[ 2 ], case.coords[ 3 ]
    );

    log::info!
    (
      "max_stale_iterations : {:?}, population_size: {}, dynasties_limit: {};",
      case.coords[ 4 ] as usize, case.coords[ 5 ] as usize, case.coords[ 6 ] as usize
    );

    let temp_schedule = LinearTempSchedule
    {
      constant : 0.0.into(),
      coefficient : case.coords[ 0 ].into(),
      reset_increase_value : 1.0.into(),
    };

    let h_problem = Problem
    {
      seeder : seeder.clone(),
      sa_temperature_schedule : Box::new( temp_schedule ),
      ga_crossover_operator : ga_crossover_operator.clone(),
      ga_selection_operator : Box::new( TournamentSelection::default() ),
      mutation_operator : mutation_operator.clone(),
    };

    let props = crate::hybrid_optimizer::PopulationModificationProportions::new()
    .set_crossover_rate( case.coords[ 3 ] )
    .set_mutation_rate( case.coords[ 2 ] )
    ;

    let optimizer = HybridOptimizer::new( Config::default(), h_problem )
    .set_sa_max_mutations_per_dynasty( case.coords[ 1 ] as usize )
    .set_population_proportions( props )
    .set_max_stale_iterations( case.coords[ 4 ] as usize )
    .set_population_size( case.coords[ 5 ] as usize )
    .set_dynasties_limit( case.coords[ 6 ] as usize )
    ;
    let ( _reason, _solution ) = optimizer.optimize();
  };

  let res = optimize_by_time( config, problem, objective_function );
    
  log::info!( "result: {:?}", res );

  res
}

pub fn optimize_by_time< F, R >( config : OptimalParamsConfig, problem : OptimalProblem< R >, objective_function : F ) -> Result< nelder_mead::Solution, nelder_mead::Error >
where F : Fn( nelder_mead::Point ) + Sync, R : RangeBounds< f64 > + Sync
{
  let objective_function = | case : nelder_mead::Point |
  {

    let now = std::time::Instant::now();
    objective_function( case );
    let elapsed = now.elapsed();
    
    log::info!
      (
        "execution duration: {:?}",
        elapsed
      );
    elapsed.as_secs_f64()
  }; 

  let mut bounds = Vec::new();
  for bound in problem.bounds
  {
    if let Some( bound ) = bound
    {
      bounds.push( bound );
    }
  }
  
  let mut optimizer = sim_annealing::Optimizer
  {
    bounds : bounds,
    objective_function : objective_function,
    max_iterations : 50,
  };
  // let mut optimizer = nelder_mead::Optimizer::new( objective_function );
  // optimizer.bounds = problem.bounds;
  // optimizer.set_starting_point( problem.starting_point.clone() );
  // optimizer.set_simplex_size( problem.simplex_size );

  // optimizer.improvement_threshold = config.improvement_threshold;
  // optimizer.max_iterations = config.max_iterations;
  // optimizer.max_no_improvement_steps = config.max_no_improvement_steps;

  optimizer.optimize()
}

/// Possible error when building NMOptimizer.
#[ derive( thiserror::Error, Debug ) ]
pub enum Error {
  #[ error( "parameter with similar name exists" ) ]
  NameError,

  #[ error( "starting value is out of bounds" ) ]
  OutOfBoundsError,
}
