use optimization_tools::{sudoku_opt_params::{OptimalParamsConfig, OptimalProblem}, *};
use optimization::*;
use test_tools::prelude::*;

mod tools;
use tools::*;

#[ test ]
fn find_opt_params_sudoku()
{
  let easy = r#"
  080924060
  920060105
  360080029
  408209600
  106003802
  002806390
  840690070
  009705208
  075040036
  "#;

  logger_init();
  log::set_max_level( log::LevelFilter::Warn );

  let config = OptimalParamsConfig::default();

  let obj_function = | case : Point |
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

    let initial = SudokuInitial::new( Board::from( easy ) );
    let temp_schedule = LinearTempSchedule
    {
      constant : 0.0.into(),
      coefficient : case.coords[ 0 ].into(),
      reset_increase_value : 1.0.into(),
    };

    let props = crate::optimization::PopulationModificationProportions::new()
    .set_crossover_rate( case.coords[ 3 ] )
    .set_mutation_rate( case.coords[ 2 ] )
    ;

    let optimizer = HybridOptimizer::new( Seed::default(), initial )
    .set_crossover_operator( BestRowsColumnsCrossover{} )
    .set_mutation_operator( RandomPairInBlockMutation{} )
    .set_sa_temp_schedule( Box::new( temp_schedule ) )
    .set_sa_max_mutations_per_dynasty( case.coords[ 1 ] as usize )
    .set_population_proportions( props )
    .set_max_stale_iterations( case.coords[ 4 ] as usize )
    .set_population_size( case.coords[ 5 ] as usize )
    .set_dynasties_limit( case.coords[ 6 ] as usize )
    ;
    
    let mut results: Vec< std::time::Duration > = Vec::new();
    for _ in 0..3
    {
      let now = std::time::Instant::now();
      let ( _reason, _solution ) = optimizer.optimize();
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
        "execution duration in ms : {}",
        average
      );
    average as f64
  }; 

  let opt_problem = OptimalProblem
  {
    bounds : [ 0.0..=1.0, 10.0..=2000.0, 0.0..=0.5, 0.0..=0.5, 1.0..=1000.0, 100.0..=5000.0, 10.0..=5000.0 ],
    starting_point : Point { coords : vec![ 0.999, 300.0, 0.25, 0.5, 30.0, 300.0, 1000.0 ] },
    simplex_size : vec![ 0.0002, 20.0, 0.1, 0.2, 5.0, 200.0, 300.0 ],
    obj_function,
  };

  let res = sudoku_opt_params::hybrid_optimal_params( config, opt_problem );
  assert!( res.len() != 0 );

}

#[ test ]
fn find_opt_params_tsp()
{
  logger_init();
  log::set_max_level( log::LevelFilter::Warn );

  let config = OptimalParamsConfig::default();

  let obj_function = | case : Point |
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

    let initial = TSProblem{ graph : TSPGraph::default(), starting_node : NodeIndex( 1 ) };
    let temp_schedule = LinearTempSchedule
    {
      constant : 0.0.into(),
      coefficient : case.coords[ 0 ].into(),
      reset_increase_value : 1.0.into(),
    };

    let props = crate::optimization::PopulationModificationProportions::new()
    .set_crossover_rate( case.coords[ 3 ] )
    .set_mutation_rate( case.coords[ 2 ] )
    ;

    let optimizer = HybridOptimizer::new( Seed::default(), initial )
    .set_crossover_operator( OrderedRouteCrossover{} )
    .set_mutation_operator( TSRouteMutation{} )
    .set_sa_temp_schedule( Box::new( temp_schedule ) )
    .set_sa_max_mutations_per_dynasty( case.coords[ 1 ] as usize )
    .set_population_proportions( props )
    .set_max_stale_iterations( case.coords[ 4 ] as usize )
    .set_population_size( case.coords[ 5 ] as usize )
    .set_dynasties_limit( case.coords[ 6 ] as usize )
    ;
    
    let mut results: Vec< std::time::Duration > = Vec::new();
    for _ in 0..3
    {
      let now = std::time::Instant::now();
      let ( _reason, _solution ) = optimizer.optimize();
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
        "execution duration in ms : {}",
        average
      );
    average as f64
  }; 

  let opt_problem = OptimalProblem
  {
    bounds : [ 0.0..=1.0, 10.0..=2000.0, 0.0..=0.5, 0.0..=0.5, 1.0..=1000.0, 100.0..=5000.0, 10.0..=5000.0 ],
    starting_point : Point { coords : vec![ 0.999, 300.0, 0.25, 0.5, 30.0, 300.0, 1000.0 ] },
    simplex_size : vec![ 0.0002, 20.0, 0.1, 0.2, 5.0, 200.0, 300.0 ],
    obj_function,
  };

  let res = sudoku_opt_params::hybrid_optimal_params( config, opt_problem );
  assert!( res.len() != 0 );

}