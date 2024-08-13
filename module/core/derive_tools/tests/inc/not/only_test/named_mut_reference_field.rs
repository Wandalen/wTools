#[ test ]
fn not()
{
  let mut value = true;
  let mut x = NamedMutReferenceField { a : &mut value, b : 0 };

  x = !x;

  assert_eq!( *x.a, false );
  assert_eq!( x.b, 255 );
}
