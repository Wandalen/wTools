use optimization_tools::*;
use problems::sudoku::*;
use test_tools::prelude::*;
use deterministic_rand::Hrng;

// #[ macro_export ]
// macro_rules! cells_container
// {
//   ( $( $Tokens : tt )+ ) =>
//   {{
//     [ $( $Tokens )+ ].into_iter().map( | e | e.into() ).collect()
//   }}
// }

// zzz : move to iter_tools, maybe
fn each_into< T, IntoIter, IntoCellVal >( src : IntoIter ) -> impl Iterator< Item = T >
where
  IntoIter : IntoIterator< Item = IntoCellVal >,
  IntoCellVal : Into< T >,
{
  src.into_iter().map( | e | e.into() )
}

#[ test ]
fn from_string()
{
  let src = "
310 000 020
006 109 005
000 080 000

020 804 050
004 070 000
000 060 008

060 000 900
009 405 001
000 007 000
";
  let got : Board = src.into();
  let storage : Vec< CellVal > = each_into
  ([
    3,1,0, 0,0,0, 0,2,0,
    0,0,6, 1,0,9, 0,0,5,
    0,0,0, 0,8,0, 0,0,0,
    0,2,0, 8,0,4, 0,5,0,
    0,0,4, 0,7,0, 0,0,0,
    0,0,0, 0,6,0, 0,0,8,
    0,6,0, 0,0,0, 9,0,0,
    0,0,9, 4,0,5, 0,0,1,
    0,0,0, 0,0,7, 0,0,0,
  ]).collect();
  let exp = Board::new( storage );
  a_id!( got, exp );
}

#[ test ]
fn cell()
{

  let board = Board::default();
  let mut cells = board.cells();

  assert_eq!( ( CellIndex::from( ( 0, 0 ) ), CellVal::from( 3 ) ), cells.next().unwrap() );
  assert_eq!( ( CellIndex::from( ( 1, 0 ) ), CellVal::from( 1 ) ), cells.next().unwrap() );
  assert_eq!( ( CellIndex::from( ( 2, 0 ) ), CellVal::from( 0 ) ), cells.next().unwrap() );

  cells.next();
  cells.next();
  cells.next();

  cells.next();
  cells.next();
  cells.next();

  cells.next();
  cells.next();
  cells.next();

  assert_eq!( ( CellIndex::from( ( 3, 1 ) ), CellVal::from( 1 ) ), cells.next().unwrap() );
  assert_eq!( ( CellIndex::from( ( 4, 1 ) ), CellVal::from( 0 ) ), cells.next().unwrap() );
  assert_eq!( ( CellIndex::from( ( 5, 1 ) ), CellVal::from( 9 ) ), cells.next().unwrap() );

}

#[ test ]
fn col()
{
  let board = Board::default();

  let exp : Vec< CellVal > = each_into([ 3, 0, 0, 0, 0, 0, 0, 0, 0 ]).collect();
  let got = board.col( 0 ).collect::< Vec< _ > >();
  a_id!( got, exp );

  let exp : Vec< CellVal > = each_into([ 0, 5, 0, 0, 0, 8, 0, 1, 0 ]).collect();
  let got = board.col( 8 ).collect::< Vec< _ > >();
  a_id!( got, exp );

  a_id!( board.cols().count(), 9 );

}

#[ test ]
fn row()
{
  let board = Board::default();

  let exp : Vec< CellVal > = each_into([ 3, 1, 0, 0, 0, 0, 0, 2, 0 ]).collect();
  let got = board.row( 0 ).collect::< Vec< _ > >();
  a_id!( got, exp );

  let exp : Vec< CellVal > = each_into([ 0, 0, 0, 0, 0, 7, 0, 0, 0 ]).collect();
  let got = board.row( 8 ).collect::< Vec< _ > >();
  a_id!( got, exp );

  a_id!( board.rows().count(), 9 );

}

#[ test ]
fn block()
{
  let board = Board::default();

  let got = board.block( ( 0, 0 ).into() ).collect::< Vec< _ > >();
  let exp : Vec< CellVal > = each_into([ 3, 1, 0, 0, 0, 6, 0, 0, 0 ]).collect();
  a_id!( got, exp );

  let got = board.block( ( 1, 0 ).into() ).collect::< Vec< _ > >();
  let exp : Vec< CellVal > = each_into([ 0, 0, 0, 1, 0, 9, 0, 8, 0 ]).collect();
  a_id!( got, exp );

  let got = board.block( ( 2, 2 ).into() ).collect::< Vec< _ > >();
  let exp : Vec< CellVal > = each_into([ 9, 0, 0, 0, 0, 1, 0, 0, 0 ]).collect();
  a_id!( got, exp );

  a_id!( board.blocks().count(), 9 );

}

#[ test ]
fn select()
{
  let board = Board::default();

  let indices = board.block_cells( ( 0, 0 ).into() );
  let got : Vec< CellVal > = board.select( indices ).collect();
  let exp : Vec< CellVal > = each_into([ 3, 1, 0, 0, 0, 6, 0, 0, 0 ]).collect();
  a_id!( got, exp );

  let indices = board.block_cells( ( 1, 0 ).into() );
  let got : Vec< CellVal > = board.select( indices ).collect();
  let exp : Vec< CellVal > = each_into([ 0, 0, 0, 1, 0, 9, 0, 8, 0 ]).collect();
  a_id!( got, exp );

  let indices = board.block_cells( ( 2, 2 ).into() );
  let got : Vec< CellVal > = board.select( indices ).collect();
  let exp : Vec< CellVal > = each_into([ 9, 0, 0, 0, 0, 1, 0, 0, 0 ]).collect();
  a_id!( got, exp );

}

#[ test ]
fn select_mut()
{
  let mut board = Board::default();

  let indices = board.block_cells( ( 0, 0 ).into() );
  board.select_mut( indices ).for_each( | e | *e = CellVal::from( e.unwrap() + 1 ) );
  let indices = board.block_cells( ( 0, 0 ).into() );
  let got : Vec< CellVal > = board.select( indices ).collect();
  let exp : Vec< CellVal > = each_into([ 4, 2, 1, 1, 1, 7, 1, 1, 1 ]).collect();

  a_id!( got, exp );

}

#[ test ]
fn cross_error()
{
  let board = Board::default();

  let exp = 14;
  let got = board.cross_error( ( 0, 0 ).into() );
  a_id!( got, exp );

  let exp = 12;
  let got = board.cross_error( ( 1, 0 ).into() );
  a_id!( got, exp );

  let exp = 14;
  let got = board.cross_error( ( 8, 8 ).into() );
  a_id!( got, exp );

}

#[ test ]
fn total_error()
{
  let board = Board::default();

  let exp = 116;
  let got = board.total_error();
  a_id!( got, exp );

}

#[ test ]
fn cells_swap()
{

  let storage : Vec< CellVal > = each_into
  ([
    0,1,0, 0,0,0, 0,2,0,
    0,0,6, 1,0,9, 0,0,5,
    0,0,0, 0,8,0, 0,0,0,
    0,2,0, 8,0,4, 0,5,0,
    0,0,4, 0,7,0, 0,0,0,
    0,0,0, 0,6,0, 0,0,8,
    0,6,0, 0,0,0, 9,0,0,
    0,0,9, 4,0,5, 0,0,1,
    0,0,0, 0,0,7, 0,0,3,
  ]).collect();
  let exp = Board::new( storage );
  let mut got = Board::default();
  got.cells_swap( ( 0, 0 ).into(), ( 8, 8 ).into() );
  a_id!( got, exp );

  let storage : Vec< CellVal > = each_into
  ([
    3,1,0, 0,0,0, 0,2,0,
    0,0,6, 1,0,9, 0,0,2,
    0,0,0, 0,8,0, 0,0,0,
    0,5,0, 8,0,4, 0,5,0,
    0,0,4, 0,7,0, 0,0,0,
    0,0,0, 0,6,0, 0,0,8,
    0,6,0, 0,0,0, 9,0,0,
    0,0,9, 4,0,5, 0,0,1,
    0,0,0, 0,0,7, 0,0,0,
  ]).collect();
  let exp = Board::new( storage );
  let mut got = Board::default();
  got.cells_swap( ( 1, 3 ).into(), ( 8, 1 ).into() );
  dbg!( &got );
  dbg!( &exp );
  a_id!( got, exp );

}

#[ test ]
fn block_missing_vals()
{

  let board = Board::default();
  let got = board.block_missing_vals( ( 0, 0 ).into() );
  let exp = hset!( 2, 4, 5, 7, 8, 9 );
  a_id!( got, exp );

}

#[ test ]
fn fill_missing_randomly()
{

  let hrng = Hrng::master_with_seed( "seed1".into() );
  let mut board = Board::default();
  println!( "{board}" );
  let full_board = board.fill_missing_randomly( hrng );
  for cell in full_board.cells()
  {
    // println!( "cell : {cell:?}" );
    assert!( cell.1 != 0.into() );
  }
  for block in full_board.blocks()
  {
    let missing = full_board.block_missing_vals( block );
    assert!( missing.len() == 0 );
  }
  println!( "{full_board} with hash {}", hash( &full_board ) );
  println!( "total_error : {}", full_board.total_error() );

  let hrng = Hrng::master_with_seed( "seed1".into() );
  let mut board2 = Board::default();
  println!( "{board2}" );
  let full_board2 = board2.fill_missing_randomly( hrng );
  println!( "{full_board2} with hash {}", hash( &full_board2 ) );
  assert_eq!( hash( &full_board ), hash( &full_board2 ) );

  // assert!( false );
}

fn hash< T : std::hash::Hash >( t : &T ) -> u64
{
  use std::hash::Hasher;
  use std::collections::hash_map::DefaultHasher;
  let mut hasher = DefaultHasher::new();
  t.hash( &mut hasher );
  hasher.finish()
}

// 310 000 020
// 006 109 005
// 000 080 000
// 020 804 050
// 004 070 000
// 000 060 008
// 060 000 900
// 009 405 001
// 000 007 000
