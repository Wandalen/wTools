use optimization_tools::*;
use problems::sudoku::*;
use hybrid_optimizer::*;
use test_tools::prelude::*;
use deterministic_rand::{ Seed, Hrng };

mod tools;
use tools::*;

#[ test ]
fn person_mutate()
{
  logger_init();

  //let initial = SudokuInitial::new_sa( Board::default(), Seed::default() );
  let board = Board::default();
  let hrng = Hrng::master_with_seed( Seed::default() );

  let mut person = SudokuPerson::new( &board, hrng.clone() );
  log::trace!( "{person:#?}" );
  a_id!( person.cost, 45.into() );
  a_id!( person.cost, person.board.total_error().into() );

  let mutagen = person.mutagen( &board, hrng.clone() );
  // make sure block is the same
  a_id!( BlockIndex::from( mutagen.cell1 ), BlockIndex::from( mutagen.cell2 ) );
  person.mutate(  &mutagen );
  log::trace!( "{person:#?}" );
  a_id!( person.cost, 48.into() );
  a_id!( person.cost, person.board.total_error().into() );

  let mutagen = person.mutagen( &board, hrng.clone() );
  // make sure block is the same
  a_id!( BlockIndex::from( mutagen.cell1 ), BlockIndex::from( mutagen.cell2 ) );
  person.mutate( &mutagen );
  log::trace!( "{person:#?}" );
  a_id!( person.cost, 48.into() );
  a_id!( person.cost, person.board.total_error().into() );

  // a_true!( false );
}

#[ test ]
fn initial_temperature()
{
  logger_init();
  let initial = SudokuInitial::new( Board::default() );
  let p = Problem::new( initial, BestRowsColumnsCrossover{}, RandomPairInBlockMutation{}  );
  let optimizer = HybridOptimizer::new( Config::default(), p );

  let temperature = optimizer.initial_temperature();
  a_true!( temperature.unwrap() >= 0f64 );
  a_id!( temperature.unwrap(), 1.591644851508443 );

}

/// Test SA on sudoku
///
/// # Usage
///
/// cargo test solve_with_sa --release
///
#[ ignore ]
#[ test ]
fn solve_with_sa()
{
  logger_init();
  log::set_max_level( log::LevelFilter::Warn );

  let input = r#"
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

  let initial = SudokuInitial::new( Board::from( input ) );
  let problem = Problem::new( initial, BestRowsColumnsCrossover, RandomPairInBlockMutation );
  let optimizer = HybridOptimizer::new( Config::default(), problem );

  log::set_max_level( log::LevelFilter::max() );
  let ( reason, solution ) = optimizer.optimize();

  log::trace!( "reason : {reason}" );
  a_true!( solution.is_some() );
  let solution = solution.unwrap();
  log::trace!( "{solution:#?}" );
  log::trace!( "{:#?}", solution.board );

  a_id!( solution.cost, 0.into() );
  #[ cfg( feature = "static_plot" ) ]
  plot::draw_plots();
  // a_true!( false );
}

/// Test SA on sudoku
///
/// # Usage
///
/// cargo test solve_empty_full_block --release
///
#[ ignore ]
#[ test ]
fn solve_empty_full_block()
{
  let _sudoku : &str = r#"
  402000000
  000038000
  090000018
  000000601
  000007530
  000120000
  000056100
  003940000
  206080047
  "#;

  let sudoku : &str = r#"
  350964170
  700020003
  019003524
  491758032
  507302801
  283600090
  900580317
  800017209
  170039406
  "#;
  log::set_max_level( log::LevelFilter::Warn );

  let initial = SudokuInitial::new( Board::from( sudoku ) );
  let problem = Problem::new( initial, BestRowsColumnsCrossover, RandomPairInBlockMutation );
  let optimizer = HybridOptimizer::new( Config::default(), problem );

  log::set_max_level( log::LevelFilter::max() );
  let ( reason, solution ) = optimizer.optimize();

  log::trace!( "reason : {reason}" );
  a_true!( solution.is_some() );
  let solution = solution.unwrap();
  log::trace!( "{solution:#?}" );
  println!( "{:#?}", solution.board );

  a_id!( solution.cost, 0.into() );
 }

//
// seed: "seed1"
// n_resets: 2,
// n_generation: 6850,
//
// seed: "seed2"
// n_resets: 0,
// n_generation: 1602,
//
// seed: "seed3"
// temperature: 0.3878543693250874,
// n_resets: 4,
// n_generation: 6992,
//
// 318756429
// 276149385
// 495283617
// 927834156
// 684571293
// 153962748
// 562318974
// 739425861
// 841697532
//

/// Test performance
///
/// # Usage
///
/// cargo test time_measure --release
///
#[ ignore ]
#[ test ]
fn time_measure()
{
  let input = r#"
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


  for i in 0..=3 {
    let initial = SudokuInitial::new( Board::from( input ) );

    let mut config = Config::default();
    config.hrng = Hrng::master_with_seed( Seed::new( i.to_string() ) );
    let problem = Problem::new( initial, BestRowsColumnsCrossover, RandomPairInBlockMutation );

    let optimizer = HybridOptimizer::new( config, problem );
    let ( _reason, _solution ) = optimizer.optimize();
  }
}
