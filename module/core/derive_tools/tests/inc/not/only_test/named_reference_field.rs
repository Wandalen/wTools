#[ test ]
fn not()
{
  let value = true;
  let mut x = NamedReferenceField { a : &value, b : 0 };

  x = !x;

  assert_eq!( *x.a, true );
  assert_eq!( x.b, 255 );
}
