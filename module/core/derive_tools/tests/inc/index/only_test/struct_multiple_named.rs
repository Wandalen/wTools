#[ test ]
fn index() 
{
  let x = StructMultipleNamed
  { 
    a : vec![ 12, 22 ], 
    b : vec![ 33, 55 ] 
  };
  let v = vec![ 33, 55 ];
  let exp =  ( v[ 0 ], v[ 1 ] );
  let got =  ( x[ 0 ], x[ 1 ] );
  
  assert_eq!( got, exp );
}
