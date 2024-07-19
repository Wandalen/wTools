#[ test ]
fn not()
{
  let mut x = TupleDefaultOff( true, 0 );

  x = !x;

  assert_eq!( x.0, true );
  assert_eq!( x.1, 0 );
}
