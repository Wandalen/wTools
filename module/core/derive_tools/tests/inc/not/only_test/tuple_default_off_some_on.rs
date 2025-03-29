#[ test ]
fn not()
{
  let mut x = TupleDefaultOffSomeOn( true, 0 );

  x = !x;

  assert_eq!( x.0, true );
  assert_eq!( x.1, 255 );
}
