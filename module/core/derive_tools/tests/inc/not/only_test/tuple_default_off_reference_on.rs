#[ test ]
fn not()
{
  let value = true;
  let mut x = TupleDefaultOffReferenceOn( &value, 0 );

  x = !x;

  assert_eq!( *x.0, true );
  assert_eq!( x.1, 0 );
}
