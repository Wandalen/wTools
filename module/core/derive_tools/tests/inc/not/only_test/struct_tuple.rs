#[ test ]
fn not()
{
  let mut x = StructTuple( true, 0 );

  x = !x;

  assert_eq!( x.0, false );
  assert_eq!( x.1, 255 );
}
