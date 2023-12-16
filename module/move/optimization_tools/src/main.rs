use optimization_tools::*;
use sudoku::*;

const INPUT : &str = r#"
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

fn main()
{
  // let board = Board::default();
  let seed : Seed = "seed1".into();
  let board = Board::from( INPUT );
  let initial = optimization::SudokuInitial::new( board, seed );

  let ( reason, generation ) = initial.solve_with_sa();

  log::trace!( "reason : {reason}" );
  assert!( generation.is_some() );
  let generation = generation.unwrap();
  log::trace!( "{generation:#?}" );
  log::trace!( "{:#?}", generation.person.board );

  // println!( "{board}" );
}
