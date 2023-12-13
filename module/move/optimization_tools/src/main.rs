use optimization_tools::*;
use sudoku::*;

fn main()
{
  let board = Board::default();
  println!( "{board}" );

  // use optimization::*;
  // use iter_tools::prelude::*;

  // let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new( "Dynamic cost", [ 400, 300 ] )
  // .samples( 1 )
  // .build()
  // .unwrap()
  // ;

  // let handle = std::thread::spawn
  // ( move || {
  //   let seed : deterministic_rand::Seed = "seed3".into();
  //   let initial = crate::optimization::SudokuInitial::new( Board::default(), seed );
  //   let ( reason, generation ) = initial.solve_with_sa();
  //   let generation = generation.unwrap();
  // });
  // dynamic_plotting::plot_dynamically( &mut window, &String::from( "Cost change" ) );

  // handle.join();

  // plotting::draw_plots();
}
