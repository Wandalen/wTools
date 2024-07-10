#[ test ]
fn index_mut() 
{
  let mut x = StructMultipleTuple( false, vec![ 2, 44, 81 ] );
  
  x[ 0 ] = 18;
  x[ 1 ] = 99;
  
  let exp = ( 18, 99 );
  let got = ( x[ 0 ], x[ 1 ] );

  assert_eq!( got, exp );
}

