use deterministic_rand::Seed;
use iter_tools::Itertools;
use optimization_tools::
{ 
  sudoku::*, 
  optimization::SudokuInitial,
  nelder_mead::NelderMeadOptimizer,
};

fn main()
{
  let dir = std::env::current_dir().unwrap();

  for level in [ "easy", "medium", "hard", "expert", "master" ]
  {
    let mut file = std::fs::File::open(format!("{}/src/resources/{}.txt", dir.to_string_lossy(), level ) ).unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    let boards = contents.split( "\n\n" ).collect_vec();

    let mut diff_coeffs = Vec::new();
    for board_str in &boards
    {
      diff_coeffs.push( Board::from( board_str ).calculate_difficulty() );
    }

    println!("{} : {:?}", level, diff_coeffs);
  }

  //   let mut diff_params = Vec::new();
  //   for board_str in &boards
  //   {
  //     let difficulty = calculate_difficulty( Board::from( board_str ) );
  
  //     let params_var = ParamsFitChecker {
  //       proc : | case : ParamsCase |
  //       {
  //         let mut initial = SudokuInitial::new( Board::from( board_str ), Seed::default() );
  //         initial.set_temp_decrease_factor( case.temp_decrease );
  //         initial.set_temp_increase_factor( case.temp_increase );
  //         initial.set_mutations_per_generation( case.gen_number );
  //         let ( _reason, _generation ) = initial.solve_with_sa();
          
  //       },
  //       lower_bound_case : ParamsCase::new( 0.0002, 1.0, 1000 ),
  //       upper_bound_case : ParamsCase::new( 0.0003, 1.5, 1500 ),
  //       number_of_iterations : 3,
    
  //     };
  //     let min_case = params_var.get_min_points();
  
  //     diff_params.push( ( difficulty, min_case ) );
  //   }
  
  //   println!( "{:?}", diff_params );
  // }

  let mut file = std::fs::File::open(format!("{}/src/resources/easy.txt", dir.to_string_lossy())).unwrap();
  let mut contents = String::new();
  std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
  let boards = contents.split( "\n\n" ).collect_vec();

  let mut optimizer = NelderMeadOptimizer::default();
  optimizer.improvement_threshold = 100.0;

  let res = optimizer.optimize
  (
    | case : Vec< f64 > |
    {
      let mut initial = SudokuInitial::new( Board::from( boards[ 0 ] ), Seed::default() );
      initial.set_temp_decrease_factor( case[ 0 ] );
      initial.set_temp_increase_factor( case[ 1 ] );
      initial.set_mutations_per_generation( case[ 2 ] as usize );
      
      let mut results: Vec< std::time::Duration > = Vec::new();
      for _ in 0..3
      {
        let now = std::time::Instant::now();
        let ( _reason, _generation ) = initial.solve_with_sa();
        let elapsed = now.elapsed();
        results.push( elapsed );
      }
      let size = results.len() as u128;
      let average = results
      .into_iter()
      .fold( 0, | acc, elem | acc + elem.as_millis() / size )
      ;
      average as f64
    }, 
    vec![ 0.0001, 1.0, 1000.0 ],
    vec![ 0.00005, 0.1, 200.0 ],
  );

  println!( "{:?} : {:?}", res.0, res.1 );

  // let mut diff_params = Vec::new();
  // for board_str in boards
  // {
  //   let difficulty = calculate_difficulty( Board::from( board_str ) );

  //   let params_var = ParamsFitChecker {
  //     proc : | case : ParamsCase |
  //     {
  //       let mut initial = SudokuInitial::new( Board::from( board_str ), Seed::default() );
  //       initial.set_temp_decrease_factor( case.temp_decrease );
  //       initial.set_temp_increase_factor( case.temp_increase );
  //       initial.set_mutations_per_generation( case.gen_number );
  //       let ( _reason, _generation ) = initial.solve_with_sa();
        
  //     },
  //     lower_bound_case : ParamsCase::new( 0.0002, 1.0, 1000 ),
  //     upper_bound_case : ParamsCase::new( 0.0003, 1.5, 1500 ),
  //     number_of_iterations : 3,
  
  //   };
  //   let min_case = params_var.get_min_points();

  //   diff_params.push( ( difficulty, min_case ) );
  // }

  // println!( "{:?}", diff_params );

}
