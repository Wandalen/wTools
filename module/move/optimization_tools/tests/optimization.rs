use optimization_tools::*;
use sudoku::*;
use optimization::*;
use test_tools::prelude::*;
use deterministic_rand::Seed;

mod tools;
use tools::*;

#[ test ]
fn person_mutate()
{
  logger_init();

  let initial = SudokuInitial::new( Board::default(), Seed::default() );

  let mut person = SudokuPerson::new( initial.board.fill_missing_randomly(initial.config.hrng.clone()) );
  log::trace!( "{person:#?}" );
  a_id!( person.cost, 45.into() );
  a_id!( person.cost, person.board.total_error().into() );

  let mutagen = person.mutagen( &initial.board, initial.config.hrng.clone() );
  // make sure block is the same
  a_id!( BlockIndex::from( mutagen.cell1 ), BlockIndex::from( mutagen.cell2 ) );
  person.mutate(  &mutagen );
  log::trace!( "{person:#?}" );
  a_id!( person.cost, 48.into() );
  a_id!( person.cost, person.board.total_error().into() );

  let mutagen = person.mutagen( &initial.board, initial.config.hrng.clone() );
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
  // logger_init();

  let initial = SudokuInitial::new( Board::default(), Seed::default() );

  let temperature = SudokuInitial::initial_temperature(&initial.board, initial.config.hrng.clone() );
  a_true!( temperature.unwrap() >= 0f64 );
  a_id!( temperature, 1.591644851508443.into() );

  // a_true!( false );
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
  let mut initial = SudokuInitial::new( Board::default(), seed );

  log::set_max_level( log::LevelFilter::max() );
  let ( reason, generation ) = initial.solve_with_sa();

  log::trace!( "reason : {reason}" );
  a_true!( generation.is_some() );
  let generation = generation.unwrap();
  log::trace!( "{generation:#?}" );
  log::trace!( "{:#?}", generation.person.board );

  a_id!( generation.person.cost, 0.into() );
  #[ cfg( feature = "static_plot" ) ]
  plot::draw_plots();
  // a_true!( false );
}

/// Test SA on sudoku
///
/// # Usage
///
/// cargo test solve_one_empty_block --release --features rapidity_6
///
#[ cfg( feature = "rapidity_6" ) ]
#[ test ]
fn solve_one_empty_block()
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
  // let seed = Seed::random();
  let mut initial = SudokuInitial::new( Board::from(sudoku), seed );

  log::set_max_level( log::LevelFilter::max() );
  let ( reason, generation ) = initial.solve_with_sa();

  log::trace!( "reason : {reason}" );
  a_true!( generation.is_some() );
  let generation = generation.unwrap();
  log::trace!( "{generation:#?}" );
  log::trace!( "{:#?}", generation.person.board );

  a_id!( generation.person.cost, 0.into() );

  // a_true!( false );
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
  for i in 0..=9 {
    let mut initial = SudokuInitial::new( Board::default(), Seed::new( i.to_string() ) );

    let ( _reason, _generation ) = initial.solve_with_sa();
  }

}
