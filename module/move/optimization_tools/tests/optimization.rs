use optimization_tools::*;
use sudoku::*;
use optimization::*;
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
  let hrng = Hrng::master_with_seed( Seed::default() );
  let initial = SudokuInitial::new( Board::default() );

  let generation = initial.initial_generation( hrng.clone(), 1 );
  let temperature = generation.initial_temperature( hrng.clone() );
  a_true!( temperature.unwrap() >= 0f64 );
  a_id!( temperature.unwrap(), 1.02469507659596 );

}

/// Test SA on sudoku
///
/// # Usage
///
/// cargo test solve_with_sa --release --features rapidity_6
///
#[ cfg( feature = "rapidity_6" ) ]
#[ test ]
fn solve_with_sa()
{
  logger_init();
  log::set_max_level( log::LevelFilter::Warn );

  // let seed : Seed = "seed1".into();
  // let seed : Seed = "seed2".into();
  let seed : Seed = "seed3".into();
  // let seed = Seed::random();
  let initial = SudokuInitial::new( Board::default() );
  let optimizer = HybridOptimizer::new( seed, initial );

  let strategy = HybridStrategy
  {
    start_with : StrategyMode::SA,
    finalize_with : StrategyMode::SA,
    number_of_cycles : 1,
    ga_generations_number : 0,
    sa_generations_number : 1000,
    population_percent : 1.0,
    generation_limit : 100_000_000,
    population_size : 1,
  };

  log::set_max_level( log::LevelFilter::max() );
  let ( reason, generation ) = optimizer.optimize( &strategy );

  log::trace!( "reason : {reason}" );
  a_true!( generation.is_some() );
  let generation = generation.unwrap();
  log::trace!( "{generation:#?}" );
  log::trace!( "{:#?}", generation.population[ 0 ].board );

  a_id!( generation.population[ 0 ].cost, 0.into() );
  #[ cfg( feature = "static_plot" ) ]
  plot::draw_plots();
  // a_true!( false );
}

/// Test SA on sudoku
///
/// # Usage
///
/// cargo test solve_empty_full_block --release --features rapidity_6
///
#[ cfg( feature = "rapidity_6" ) ]
#[ test ]
fn solve_empty_full_block()
{
  let sudoku : &str = r#"
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
  log::set_max_level( log::LevelFilter::Warn );

  let seed : Seed = "seed3".into();
  let mut initial = SudokuInitial::new( Board::from( sudoku ) );
  let optimizer = HybridOptimizer::new( seed, initial );

  let strategy = HybridStrategy
  {
    start_with : StrategyMode::SA,
    finalize_with : StrategyMode::SA,
    number_of_cycles : 1,
    ga_generations_number : 0,
    sa_generations_number : 1000,
    population_percent : 1.0,
    generation_limit : 100_000_000,
    population_size : 1,
  };
  log::set_max_level( log::LevelFilter::max() );
  let ( reason, generation ) = optimizer.optimize( &strategy );

  log::trace!( "reason : {reason}" );
  a_true!( generation.is_some() );
  let generation = generation.unwrap();
  log::trace!( "{generation:#?}" );
  println!( "{:#?}", generation.population[ 0 ].board );

  a_id!( generation.population[ 0 ].cost, 0.into() );

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

  let seed : Seed = "seed3".into();
  initial = SudokuInitial::new( Board::from( sudoku ) );
  let optimizer = HybridOptimizer::new( seed, initial );

  let strategy = HybridStrategy
  {
    start_with : StrategyMode::SA,
    finalize_with : StrategyMode::SA,
    number_of_cycles : 1,
    ga_generations_number : 0,
    sa_generations_number : 1000,
    population_percent : 1.0,
    generation_limit : 100_000_000,
    population_size : 1,
  };
  log::set_max_level( log::LevelFilter::max() );
  let ( reason, generation ) = optimizer.optimize( &strategy );

  log::trace!( "reason : {reason}" );
  a_true!( generation.is_some() );
  let generation = generation.unwrap();
  log::trace!( "{generation:#?}" );
  println!( "{:#?}", generation.population[ 0 ].board );

  a_id!( generation.population[ 0 ].cost, 0.into() );
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
/// cargo test time_measure --release --features rapidity_6
///
#[ cfg( feature = "rapidity_6" ) ]
#[ test ]
fn time_measure()
{
  let strategy = HybridStrategy
  {
    start_with : StrategyMode::SA,
    finalize_with : StrategyMode::SA,
    number_of_cycles : 1,
    ga_generations_number : 0,
    sa_generations_number : 1000,
    population_percent : 1.0,
    generation_limit : 100_000_000,
    population_size : 1,
  };

  for i in 0..=9 {
    let initial = SudokuInitial::new( Board::default() );
    let optimizer = HybridOptimizer::new( Seed::new( i.to_string() ), initial );
    let ( _reason, _generation ) = optimizer.optimize( &strategy );
  }

}
