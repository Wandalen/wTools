
// use wtools::vector;
use wtools::vector::{ left_index, append_vectors_once };

//

#[test]
fn append_vectors_once()
{
  println!( "empty vector" );
  let mut appender: append_vectors_once<u8> = append_vectors_once::default();
  let got = appender.call();
  let exp: Vec<u8> = vec![];
  assert_eq!( got, exp );

  println!( "dst - filled, src - empty" );
  let mut appender: append_vectors_once<u8> = append_vectors_once::default();
  appender.dst( vec![ 1, 2 ] );
  let got = appender.call();
  assert_eq!( got, vec![ 1, 2 ] );

  println!( "dst - filled, src - filled, all is unical elements" );
  let mut appender: append_vectors_once<u8> = append_vectors_once::default();
  appender.dst( vec![ 1, 2 ] );
  appender.src( vec![ vec![ 3, 4 ], vec![ 5, 6 ] ] );
  let got = appender.call();
  assert_eq!( *got, vec![ 1, 2, 3, 4, 5, 6 ] );

  println!( "dst - filled, src - filled, some is duplicated" );
  let mut appender: append_vectors_once<u8> = append_vectors_once::default();
  appender.dst( vec![ 1, 2 ] );
  appender.src( vec![ vec![ 1, 3 ], vec![ 2, 4 ] ] );
  let got = appender.call();
  assert_eq!( got, vec![ 1, 2, 3, 4 ] );

  println!( "dst - filled, src - filled, all is duplicated" );
  let mut appender: append_vectors_once<u8> = append_vectors_once::default();
  appender.dst( vec![ 1, 2 ] );
  appender.src( vec![ vec![ 1, 1 ], vec![ 2, 2 ] ] );
  let got = appender.call();
  assert_eq!( got, vec![ 1, 2 ] );

  println!( "dst - filled, src - filled, all is duplicated" );
  let mut appender: append_vectors_once<u8> = append_vectors_once::default();
  appender.dst( vec![ 1, 2 ] );
  appender.src( vec![ vec![ 1, 2 ], vec![ 2, 1 ] ] );
  let got = appender.call();
  assert_eq!( got, vec![ 1, 2 ] );
}

//

#[test]
fn left_index()
{
  println!( "empty vector" );
  let mut src : left_index<u8> = left_index::default();
  src.ins( 1 );
  let got = src.call();
  assert_eq!( got, None );

  println!( "filled vector, not matches" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 1, 2, 3 ] );
  src.ins( 4 );
  let got = src.call();
  assert_eq!( got, None );

  println!( "filled vector, matches one" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 1, 2, 3 ] );
  src.ins( 2 );
  let got = src.call();
  assert_eq!( got, Some( 1 ) );

  println!( "filled vector, matches several" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 1, 2, 2 ] );
  src.ins( 2 );
  let got = src.call();
  assert_eq!( got, Some( 1 ) );

  println!( "filled vector, matches all" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 2, 2, 2 ] );
  src.ins( 2 );
  let got = src.call();
  assert_eq!( got, Some( 0 ) );
}

//

#[test]
fn left_index_with_equalizer()
{
  fn equalizer( src1 : &u8, src2 : &u8 ) -> bool
  {
    *src1 == *src2
  }

  /* */

  println!( "empty vector" );
  let mut src : left_index<u8> = left_index::default();
  src.ins( 1 );
  src.on_equalize( equalizer );
  let got = src.call();
  assert_eq!( got, None );

  println!( "filled vector, not matches" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 1, 2, 3 ] );
  src.ins( 4 );
  src.on_equalize( equalizer );
  let got = src.call();
  assert_eq!( got, None );

  println!( "filled vector, matches one" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 1, 2, 3 ] );
  src.ins( 2 );
  src.on_equalize( equalizer );
  let got = src.call();
  assert_eq!( got, Some( 1 ) );

  println!( "filled vector, matches several" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 1, 2, 2 ] );
  src.ins( 2 );
  src.on_equalize( equalizer );
  let got = src.call();
  assert_eq!( got, Some( 1 ) );

  println!( "filled vector, matches all" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 2, 2, 2 ] );
  src.ins( 2 );
  src.on_equalize( equalizer );
  let got = src.call();
  assert_eq!( got, Some( 0 ) );
}

//

#[test]
fn left_index_with_evaluator()
{
  fn evaluator1( src : &u8 ) -> u8
  {
    *src
  }

  /* */

  println!( "empty vector" );
  let mut src : left_index<u8> = left_index::default();
  src.ins( 1 );
  src.on_evaluate1( evaluator1 );
  let got = src.call();
  assert_eq!( got, None );

  println!( "filled vector, not matches" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 1, 2, 3 ] );
  src.ins( 4 );
  src.on_evaluate1( evaluator1 );
  let got = src.call();
  assert_eq!( got, None );

  println!( "filled vector, matches one" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 1, 2, 3 ] );
  src.ins( 2 );
  src.on_evaluate1( evaluator1 );
  let got = src.call();
  assert_eq!( got, Some( 1 ) );

  println!( "filled vector, matches several" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 1, 2, 2 ] );
  src.ins( 2 );
  src.on_evaluate1( evaluator1 );
  let got = src.call();
  assert_eq!( got, Some( 1 ) );

  println!( "filled vector, matches all" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 2, 2, 2 ] );
  src.ins( 2 );
  src.on_evaluate1( evaluator1 );
  let got = src.call();
  assert_eq!( got, Some( 0 ) );
}

//

#[test]
fn left_index_with_two_evaluators()
{
  fn evaluator1( src : &u8 ) -> u8
  {
    *src
  }
  fn evaluator2( src : &u8 ) -> u8
  {
    *src
  }

  /* */

  println!( "empty vector" );
  let mut src : left_index<u8> = left_index::default();
  src.ins( 1 );
  src.on_evaluate1( evaluator1 );
  src.on_evaluate1( evaluator2 );
  let got = src.call();
  assert_eq!( got, None );

  println!( "filled vector, not matches" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 1, 2, 3 ] );
  src.ins( 4 );
  src.on_evaluate1( evaluator1 );
  src.on_evaluate1( evaluator2 );
  let got = src.call();
  assert_eq!( got, None );

  println!( "filled vector, matches one" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 1, 2, 3 ] );
  src.ins( 2 );
  src.on_evaluate1( evaluator1 );
  src.on_evaluate1( evaluator2 );
  let got = src.call();
  assert_eq!( got, Some( 1 ) );

  println!( "filled vector, matches several" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 1, 2, 2 ] );
  src.ins( 2 );
  src.on_evaluate1( evaluator1 );
  src.on_evaluate1( evaluator2 );
  let got = src.call();
  assert_eq!( got, Some( 1 ) );

  println!( "filled vector, matches all" );
  let mut src : left_index<u8> = left_index::default();
  src.src( vec![ 2, 2, 2 ] );
  src.ins( 2 );
  src.on_evaluate1( evaluator1 );
  src.on_evaluate1( evaluator2 );
  let got = src.call();
  assert_eq!( got, Some( 0 ) );

}

/*
  append_vectors_once
  left_index
  left_index_with_equalizer
  left_index_with_evaluator
  left_index_with_two_evaluators
*/
