//! Performs solving of sudoku puzzle using Simmulated Annealing algorithm.
//! 

use optimization_tools::{ *, optimization::{ HybridOptimizer, BestRowsColumnsCrossover, RandomPairInBlockMutation } };
use sudoku::*;

const INPUT : &str = r#"
024007000
600000000
003680415
431005000
500000032
790000060
209710800
040093000
310004750
"#;

fn main()
{
  let _ = env_logger::builder()
  .filter_level( log::LevelFilter::max() )
  .try_init();

  let seed : Seed = "seed1".into();
  let board = Board::from( INPUT );
  println!("{board}");
  let initial = optimization::SudokuInitial::new( board );
  let mut optimizer = HybridOptimizer::new( seed, initial )
  .set_crossover_operator( BestRowsColumnsCrossover{} )
  .set_mutation_operator( RandomPairInBlockMutation{} )
  ;

  let ( reason, solution ) = optimizer.optimize( );

  log::trace!( "reason : {reason}" );
  assert!( solution.is_some() );
  let solution = solution.unwrap();
  log::trace!( "{solution:#?}" );
  log::trace!( "{:#?}", solution.board );

  // let mut dp = plot_dynamic::init_dyn_plotter( String::from( "Cost change" ), 800, 400 );

  // let handle = std::thread::spawn
  // ( move || 
  //   {
  //     let seed : deterministic_rand::Seed = "seed3".into();
  //     let initial = crate::optimization::SudokuInitial::new( Board::default(), seed );
  //     let ( _reason, generation ) = initial.solve_with_sa();
  //     let _generation = generation.unwrap();
  //   }
  // );

  // dp.plot_dynamically();
  
  // _ = handle.join();
  
}