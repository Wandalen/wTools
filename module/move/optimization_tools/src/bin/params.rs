use std::collections::HashMap;

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
  let mut boards = HashMap::new();

  for level in [ "easy", "medium", "hard", "expert" ]
  {
    let mut file = std::fs::File::open( format!( "{}/src/resources/{}.txt", dir.to_string_lossy(), level ) ).unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string( &mut file, &mut contents ).unwrap();
    let boards_str = contents.split( "\n\n" ).collect_vec();
    boards.insert( level, Vec::new() );

    let mut diff_coeffs = Vec::new();
    for board_str in boards_str
    {
      let board = Board::from( board_str );
      diff_coeffs.push( board.calculate_difficulty() );
      // let s = SudokuInitial::new( board.clone(), Seed::default() );
      // println!( "{:?}", board );
      // s.solve_with_sa();
      boards.get_mut( level ).unwrap().push( board );
    }

    println!( "{} : {:?}", level, diff_coeffs );
  }

  let mut optimizer = NelderMeadOptimizer::default();
  optimizer.improvement_threshold = 50.0;
  optimizer.max_no_improvement_steps = 5;
  optimizer.max_iterations = 25;

  let mut level_results = HashMap::new();
  for ( level, level_boards ) in boards
  {
    level_results.insert( level, Vec::new() );
    for board in level_boards
    {
      let res = optimizer.optimize
      (
        | case : Vec< f64 > |
        {
          let mut initial = SudokuInitial::new( board.clone(), Seed::default() );
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
      //println!( "{}: {:?} : {:?}", level, res.0, res.1 );
      let results = level_results.get_mut( level ).unwrap();
      results.push( res );
    }
  }

  let mut level_average = HashMap::new();
  for ( level, results ) in level_results
  {
    let size = results.len() as f64;
    level_average.insert
    ( 
      level,  
      results.iter().fold
      ( 
        ( 0.0, 0.0, 0.0 ), 
        | acc, elem | 
        ( 
          acc.0 + elem.0[ 0 ] / size, 
          acc.1 + elem.0[ 1 ] / size, 
          acc.2 + elem.0[ 2 ] / size, 
        )
      ),
    );
  }
  println!( "Average: {:?}", level_average );

  //check improvement

  for level in [ "easy", "medium", "hard", "expert" ]
  {
    let mut file = std::fs::File::open( format!( "{}/src/resources/{}-check.txt", dir.to_string_lossy(), level ) ).unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string( &mut file, &mut contents ).unwrap();
    let boards_str = contents.split( "\n\n" ).collect_vec();
    println!( "{}", level );

    for board_str in boards_str
    {
      let board = Board::from( board_str );
      println!( "Difficulty: {}", board.calculate_difficulty() );
      // initial
      let mut initial = SudokuInitial::new( board.clone(), Seed::default() );
      
      let now = std::time::Instant::now();
      let ( _reason, _generation ) = initial.solve_with_sa();
      let elapsed = now.elapsed();

      println!( "Without optimization: {:?}", elapsed );
      //optimized
      initial = SudokuInitial::new( board.clone(), Seed::default() );
      let optimized_params = level_average.get( &level ).unwrap();
      initial.set_temp_decrease_factor( optimized_params.0 );
      initial.set_temp_increase_factor( optimized_params.1 );
      initial.set_mutations_per_generation( optimized_params.2 as usize );
      
      let now = std::time::Instant::now();
      let ( _reason, _generation ) = initial.solve_with_sa();
      let elapsed = now.elapsed();

      println!( "Optimized: {:?}", elapsed );
    }

    
  }
}
