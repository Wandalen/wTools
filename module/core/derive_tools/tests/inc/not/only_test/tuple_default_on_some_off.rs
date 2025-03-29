#[ test ]
fn not()
{
  let mut x = TupleDefaultOnSomeOff( true, 0 );

  x = !x;

  assert_eq!( x.0, false );
  assert_eq!( x.1, 0 );
}
