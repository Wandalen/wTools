#[ test ]
fn not()
{
  let value = true;
  let mut x = TupleReferenceField( &value, 0 );

  x = !x;

  assert_eq!( *x.0, true );
  assert_eq!( x.1, 255 );
}
