#[ test ]
fn not()
{
  let mut x = NameCollisions { a : true, b : 0 };

  x = !x;

  assert_eq!( x.a, false );
  assert_eq!( x.b, 255 );
}
