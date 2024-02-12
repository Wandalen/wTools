//! Funcions for calculation optimal config parameters.
//! 

use std::ops::RangeBounds;
use crate::
{ 
  optimization::{ 
    Config, 
    CrossoverOperator, 
    HybridOptimizer, 
    InitialProblem, 
    LinearTempSchedule, 
    MutationOperator, 
    Problem, 
    SelectionOperator, 
    TournamentSelection 
  },
  nelder_mead,
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

pub struct OptimalProblem< R : RangeBounds< f64 > >
{
  pub bounds : Vec< Option< R > >,
  pub starting_point : Option< nelder_mead::Point >,
  pub simplex_size : Option< Vec< f64 > >,
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
      "temp_decrease_coefficient : {:?}, max_mutations_per_dynasty: {}, mutation_rate: {}, crossover_rate: {};",
      case.coords[ 0 ], case.coords[ 1 ], case.coords[ 2 ], case.coords[ 3 ]
    );

    log::info!
    (
      "max_stale_iterations : {:?}, population_size: {}, dynasties_limit: {};",
      case.coords[ 4 ], case.coords[ 5 ], case.coords[ 6 ]
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

    let props = crate::optimization::PopulationModificationProportions::new()
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
        "execution duration in ms : {:?}",
        elapsed
      );
    elapsed.as_secs_f64()
  }; 

  let mut optimizer = nelder_mead::Optimizer::new( objective_function );
  optimizer.bounds = problem.bounds;
  if let Some( start_point ) = problem.starting_point
  {
    optimizer.start_point = start_point;
  }

  if let Some( simplex_size ) = problem.simplex_size
  {
    optimizer.set_simplex_size( simplex_size );
  }

  optimizer.improvement_threshold = config.improvement_threshold;
  optimizer.max_iterations = config.max_iterations;
  optimizer.max_no_improvement_steps = config.max_no_improvement_steps;

  optimizer.optimize()
}

