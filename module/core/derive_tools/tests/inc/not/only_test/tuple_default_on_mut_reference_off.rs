#[ test ]
fn not()
{
  let mut value = true;
  let mut x = TupleDefaultOnMutReferenceOff( &mut value, 0 );

  x = !x;

  assert_eq!( *x.0, true );
  assert_eq!( x.1, 255 );
}