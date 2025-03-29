#[ test ]
fn index() 
{
  let x = StructTuple( vec![ 2, 44, 81 ] );
  let exp = ( 2, 44 );
  let got = ( x[ 0 ], x[ 1 ] );

  assert_eq!( got, exp );
}
