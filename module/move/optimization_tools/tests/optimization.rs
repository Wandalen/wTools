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

  let person = SudokuPerson::new( &initial );
  log::trace!( "{person:#?}" );
  a_id!( person.cost, 45.into() );
  a_id!( person.cost, person.board.total_error().into() );

  let mutagen = person.mutagen( &initial, initial.hrng.clone() );
  // make sure block is the same
  a_id!( BlockIndex::from( mutagen.cell1 ), BlockIndex::from( mutagen.cell2 ) );
  let person2 = person.mutate( &initial, &mutagen );
  log::trace!( "{person2:#?}" );
  a_id!( person2.cost, 48.into() );
  a_id!( person2.cost, person2.board.total_error().into() );

  let mutagen = person2.mutagen( &initial, initial.hrng.clone() );
  // make sure block is the same
  a_id!( BlockIndex::from( mutagen.cell1 ), BlockIndex::from( mutagen.cell2 ) );
  let person3 = person2.mutate( &initial, &mutagen );
  log::trace!( "{person3:#?}" );
  a_id!( person3.cost, 48.into() );
  a_id!( person3.cost, person3.board.total_error().into() );

  // a_true!( false );
}

#[ test ]
fn initial_temperature()
{
  logger_init();

  let initial = SudokuInitial::new( Board::default(), Seed::default() );

  let temperature = initial.initial_temperature();
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
  let initial = SudokuInitial::new( Board::default(), seed );

  log::set_max_level( log::LevelFilter::max() );
  let ( reason, generation ) = initial.solve_with_sa();

  log::trace!( "reason : {reason}" );
  a_true!( generation.is_some() );
  let generation = generation.unwrap();
  log::trace!( "{generation:#?}" );
  log::trace!( "{:#?}", generation.person.board );

  a_id!( generation.person.cost, 0.into() );

  #[ cfg( feature = "static_plot" ) ]
  plotting::draw_plots();
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
