
use wtools::vector;

//

#[test]
fn append_vectors_once_trivial()
{
  println!( "empty vector::" );
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

/*
  append_vectors_once_trivial
*/
