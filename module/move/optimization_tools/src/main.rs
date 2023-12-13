use optimization_tools::*;
use sudoku::*;
use crate::optimization::SudokuInitial;

fn main()
{
  let board = Board::default();
  println!( "{board}" );

  // use optimization::*;
  // use iter_tools::prelude::*;
  // use deterministic_rand::Seed;
  // let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new("Test", [400, 300])
  // .samples(1)
  // .build()
  // .unwrap()
  // ;

  // plotting::draw_dynamic(&mut window);

  // let seed : Seed = "seed3".into();
  // let initial = SudokuInitial::new( Board::default(), seed );
  // let ( reason, generation ) = initial.solve_with_sa();

  // let generation = generation.unwrap();

  // plotting::draw_plots();
}
