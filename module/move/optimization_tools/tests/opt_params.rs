use optimization_tools::{nelder_mead::Point, sudoku::Board, sudoku_opt_params::{OptimalParamsConfig, OptimalProblem}, *};
use optimization::*;
use test_tools::prelude::*;

mod tools;
use tools::*;

#[ ignore ]
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
  let initial = SudokuInitial::new( Board::from( easy ) );

  let opt_problem = OptimalProblem
  {
    bounds : vec![ Some( 0.0..=1.0 ), Some( 10.0..=2000.0 ), Some( 0.0..=0.5 ), Some( 0.0..=0.5 ), Some( 1.0..=1000.0 ), Some( 100.0..=5000.0 ), Some( 10.0..=5000.0 ) ],
    starting_point : Some( Point { coords : vec![ 0.999, 300.0, 0.25, 0.5, 30.0, 300.0, 1000.0 ] } ),
    simplex_size : Some( vec![ 0.0002, 20.0, 0.1, 0.2, 5.0, 200.0, 300.0 ] ),
  };

  let hybrid_problem = Problem::new( initial, BestRowsColumnsCrossover{}, RandomPairInBlockMutation{} );

  let res = sudoku_opt_params::find_hybrid_optimal_params( config, opt_problem, hybrid_problem );
  assert!( res.is_ok() );

}

#[ ignore ]
#[ test ]
fn find_opt_params_tsp()
{
  logger_init();
  log::set_max_level( log::LevelFilter::Warn );

  let config = OptimalParamsConfig::default();
  let initial = TSProblem{ graph : TSPGraph::default(), starting_node : NodeIndex( 1 ) };
  let hybrid_problem = Problem::new( initial, OrderedRouteCrossover{}, TSRouteMutation{} );

  let opt_problem = OptimalProblem
  {
    bounds : vec![ Some( 0.0..=1.0 ), Some( 10.0..=2000.0 ), Some( 0.0..=0.5 ), Some( 0.0..=0.5 ), Some( 1.0..=1000.0 ), Some( 100.0..=5000.0 ), Some( 10.0..=5000.0 ) ],
    starting_point : Some( Point { coords : vec![ 0.999, 300.0, 0.25, 0.5, 30.0, 300.0, 1000.0 ] } ),
    simplex_size : Some( vec![ 0.0002, 20.0, 0.1, 0.2, 5.0, 200.0, 300.0 ] ),
  };

  let res = sudoku_opt_params::find_hybrid_optimal_params( config, opt_problem, hybrid_problem );
  assert!( res.is_ok() );

}