#[ test ]
fn not()
{
  let mut value = true;
  let mut x = NamedDefaultOnMutReferenceOff { a : &mut value, b : 0 };

  x = !x;

  assert_eq!( *x.a, true );
  assert_eq!( x.b, 255 );
}