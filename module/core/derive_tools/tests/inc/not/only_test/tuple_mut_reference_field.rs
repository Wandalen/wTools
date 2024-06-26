#[ test ]
fn not()
{
  let mut value = true;
  let mut x = TupleMutReferenceField( &mut value, 0 );

  x = !x;

  assert_eq!( *x.0, false );
  assert_eq!( x.1, 255 );
}
