use deterministic_rand::Seed;
use iter_tools::Itertools;
use optimization_tools::
{ 
  sudoku::*, 
  optimization::SudokuInitial,
  params_variation::{ calculate_difficulty, ParamsFitChecker, ParamsCase },
};

fn main()
{
  let dir = std::env::current_dir().unwrap();

  let mut file = std::fs::File::open(format!("{}/src/resources/hard.txt", dir.to_string_lossy())).unwrap();
  let mut contents = String::new();
  std::io::Read::read_to_string(&mut file, &mut contents).unwrap();

  let boards = contents.split( "\n\n" ).collect_vec();

  let mut diff_params = Vec::new();
  for board_str in boards
  {
    let difficulty = calculate_difficulty( Board::from( board_str ) );

    let params_var = ParamsFitChecker {
      proc : | case : ParamsCase |
      {
        let mut initial = SudokuInitial::new( Board::from( board_str ), Seed::default() );
        initial.set_temp_decrease_factor( case.temp_decrease );
        initial.set_temp_increase_factor( case.temp_increase );
        initial.set_mutations_per_generation( case.gen_number );
        let ( _reason, _generation ) = initial.solve_with_sa();
        
      },
      lower_bound_case : ParamsCase::new( 0.0001, 1.0, 1000 ),
      upper_bound_case : ParamsCase::new( 0.0002, 1.5, 1500 ),
      number_of_iterations : 3,
  
    };
    let min_case = params_var.get_min_points();

    diff_params.push( ( difficulty, min_case ) );
  }

  println!( "{:?}", diff_params );

}
