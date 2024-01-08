use deterministic_rand::Seed;
use optimization_tools::
{ 
  sudoku::*, 
  optimization::SudokuInitial,
  params_variation::{ get_time_for_step, get_time_for_coefficient, get_time_for_input },
};

const _INPUT : &str = r#"
024007000
600000000
003680415
431005000
500000032
790000060
209710800
040093000
310004750
"#;
const INPUT1 : &str = r#"
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

fn main()
{
  let durations = get_time_for_step( | cooling_factor |
    {
      let mut initial = SudokuInitial::new( Board::from( INPUT1 ), Seed::default() );
      initial.set_temp_increase_factor( cooling_factor );
      let ( _reason, _generation ) = initial.solve_with_sa();
      
    }, 1.0, 0.2, 2 );
    println!( "Temperature increase factor {:?}", durations );

  let durations = get_time_for_coefficient( | cooling_factor |
  {
    let mut initial = SudokuInitial::new( Board::from( INPUT1 ), Seed::default() );
    initial.set_temp_decrease_factor( cooling_factor );
    let ( _reason, _generation ) = initial.solve_with_sa();
    
  }, 0.0001, 0.8, 2 );
  println!( "Temperature decrease factor {:?}", durations );

  let durations = get_time_for_input( | max |
  {
    let mut initial = SudokuInitial::new( Board::from( INPUT1 ), Seed::default() );
    initial.set_mutations_per_generation( max );
    let ( _reason, _generation ) = initial.solve_with_sa();
    
  }, vec![ 500, 1000, 2000 ], std::time::Duration::new( 1200, 0 ), 10 );

  println!( "Mutation per generation {:?}", durations );
}
