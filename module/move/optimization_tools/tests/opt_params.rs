use optimization_tools::{ sudoku::Board, hybrid_opt_params::OptimalParamsConfig, * };
use optimization::*;

mod tools;
use tools::*;

#[ ignore ]
#[ test ]
fn find_opt_params_sudoku() -> Result< (), hybrid_opt_params::Error > 
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
  let initial = SudokuInitial::new( Board::from( easy ) );

  let hybrid_problem = Problem::new( initial, BestRowsColumnsCrossover{}, RandomPairInBlockMutation{} );

  let res = hybrid_opt_params::find_hybrid_optimal_params( config, optimization::starting_params_for_hybrid()?, hybrid_problem );
  assert!( res.is_ok() );
  Ok( () )
}

#[ ignore ]
#[ test ]
fn find_opt_params_tsp() -> Result< (), hybrid_opt_params::Error > 
{
  logger_init();
  log::set_max_level( log::LevelFilter::Warn );

  let config = OptimalParamsConfig::default();
  let initial = TSProblem{ graph : TSPGraph::default(), starting_node : NodeIndex( 1 ) };
  let hybrid_problem = Problem::new( initial, OrderedRouteCrossover{}, TSRouteMutation{} );

  let res = hybrid_opt_params::find_hybrid_optimal_params( config, optimization::starting_params_for_hybrid()?, hybrid_problem );
  assert!( res.is_ok() );
  Ok( () )
}