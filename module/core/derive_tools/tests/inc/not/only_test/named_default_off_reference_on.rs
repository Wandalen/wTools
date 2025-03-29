#[ test ]
fn not()
{
  let value = true;
  let mut x = NamedDefaultOffReferenceOn { a : &value, b : 0 };

  x = !x;

  assert_eq!( *x.a, true );
  assert_eq!( x.b, 0 );
}
