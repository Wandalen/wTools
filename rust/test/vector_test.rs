
use wtools::vector;
use wtools::vector::left_index;

//

#[test]
fn append_vectors_once_trivial()
{
  println!( "empty vector" );
  let mut dst: Vec<u8> = vec![];
  let src: Vec<Vec<u8>> = vec![];
  let got = vector::append_vectors_once( &mut dst, &src, None::<fn(_)> );
  let exp: Vec<u8> = vec![];
  assert_eq!( *got, exp );

  println!( "dst - filled, src - empty" );
  let mut dst: Vec<u8> = vec![ 1, 2 ];
  let src: Vec<Vec<u8>> = vec![];
  let got = vector::append_vectors_once( &mut dst, &src, None::<fn(_)> );
  assert_eq!( *got, vec![ 1, 2 ] );

  println!( "dst - filled, src - filled, all is unical elements" );
  let mut dst: Vec<u8> = vec![ 1, 2 ];
  let src: Vec<Vec<u8>> = vec![ vec![ 3, 4 ], vec![ 5, 6 ] ];
  let got = vector::append_vectors_once( &mut dst, &src, None::<fn(_)> );
  assert_eq!( *got, vec![ 1, 2, 3, 4, 5, 6 ] );

  println!( "dst - filled, src - filled, some is duplicated" );
  let mut dst: Vec<u8> = vec![ 1, 2 ];
  let src: Vec<Vec<u8>> = vec![ vec![ 1, 3 ], vec![ 4, 2 ] ];
  let got = vector::append_vectors_once( &mut dst, &src, None::<fn(_)> );
  assert_eq!( *got, vec![ 1, 2, 3, 4 ] );

  println!( "dst - filled, src - filled, all is duplicated" );
  let mut dst: Vec<u8> = vec![ 1, 2 ];
  let src: Vec<Vec<u8>> = vec![ vec![ 1, 1 ], vec![ 2, 2 ] ];
  let got = vector::append_vectors_once( &mut dst, &src, None::<fn(_)> );
  assert_eq!( *got, vec![ 1, 2 ] );

  println!( "dst - filled, src - filled, all is duplicated" );
  let mut dst: Vec<u8> = vec![ 1, 2 ];
  let src: Vec<Vec<u8>> = vec![ vec![ 1, 2 ], vec![ 2, 1 ] ];
  let got = vector::append_vectors_once( &mut dst, &src, None::<fn(_)> );
  assert_eq!( *got, vec![ 1, 2 ] );
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
  append_vectors_once_trivial
  left_index_trivial
  left_index_with_equalizer
*/
