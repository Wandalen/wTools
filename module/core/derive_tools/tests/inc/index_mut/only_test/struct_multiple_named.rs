#[ test ]
fn index_mut() 
{
  let mut x = StructMultipleNamed
  { 
    a : vec![ 4, 17 ],
    b : vec![ 33, 55 ] 
  };
  
  x[ 0 ] = 5;
  x[ 1 ] = 18;
  let v = vec![ 5, 18 ];
  
  let exp =  ( v[ 0 ], v[ 1 ] );
  let got =  ( x[ 0 ], x[ 1 ] );

  assert_eq!( got, exp );
}


