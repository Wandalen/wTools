use optimization_tools::*;
use sudoku::*;

fn main()
{
  let board = Board::default();
  println!( "{board}" );

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
