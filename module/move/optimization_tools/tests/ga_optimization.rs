use iter_tools::Itertools;
use optimization_tools::*;
use hybrid_optimizer::CrossoverOperator;
use problems::sudoku::*;
use deterministic_rand::{ Seed, Hrng };

mod tools;
use tools::*;

#[ test ]
fn crossover()
{
  logger_init();

  let board = Board::default();
  let hrng = Hrng::master_with_seed( Seed::default() );

  let parent1 = SudokuPerson::new( &board, hrng.clone() );
  log::trace!( "parent 1{parent1:#?}" );
  
  let parent2 = SudokuPerson::new( &board, hrng.clone() );
  log::trace!( "parent 2{parent2:#?}" );

  let operator = MultiplePointsBlockCrossover;

  let child = operator.crossover( hrng.clone(), &parent1, &parent2 );
  log::trace!( "child {child:#?}" );
  let mut is_child = true;
  let mut has_first_parent_blocks = false;
  let mut has_second_parent_blocks = false;

  for i in child.board.blocks()
  {
    if child.board.block( i ).collect_vec() != parent1.board.block( i ).collect_vec() 
      && child.board.block( i ).collect_vec() != parent2.board.block( i ).collect_vec()
    {
      is_child = false;
    }

    if child.board.block( i ).collect_vec() == parent1.board.block( i ).collect_vec() 
    {
      has_first_parent_blocks = true;
    }

    if child.board.block( i ).collect_vec() == parent2.board.block( i ).collect_vec() 
    {
      has_second_parent_blocks = true;
    }
  }
  assert!( is_child && has_first_parent_blocks && has_second_parent_blocks );
}

/// Test GA on sudoku
///
/// # Usage
///
/// cargo test solve_with_ga --release
///
#[ ignore ]
#[ test ]
fn solve_with_ga()
{
  use test_tools::prelude::*;
  use hybrid_optimizer::{ Config, HybridOptimizer, Problem };
  let sudoku : &str = r#"
  801920000
  040850726
  056073090
  598004100
  700000530
  002600400
  900300680
  683190050
  000000013
  "#;

  logger_init();
  log::set_max_level( log::LevelFilter::Warn );

  let initial = SudokuInitial::new( Board::from( sudoku ) );
  let problem = Problem::new( initial, BestRowsColumnsCrossover, RandomPairInBlockMutation );

  let optimizer = HybridOptimizer::new( Config::default(), problem );

  let ( reason, solution ) = optimizer.optimize();

  log::trace!( "reason : {reason}" );
  a_true!( solution.is_some() );
  let solution = solution.unwrap();
  log::trace!( "{solution:#?}" );
  log::trace!( "{:#?}", solution.board );

  a_id!( solution.cost, 0.into() );

}

