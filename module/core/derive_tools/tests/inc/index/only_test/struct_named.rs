#[ test ]
fn index() 
{
  let x = StructNamed
  { 
    a: vec![ false, true ] 
  };
  let v = vec![ false, true ];
  let exp =  ( v[ 0 ], v[ 1 ] );
  let got =  ( x[ 0 ], x[ 1 ] );

  assert_eq!( got, exp );
}
