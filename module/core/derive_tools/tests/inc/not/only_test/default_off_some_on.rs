#[ test ]
fn not()
{
  let mut x = DefaultOffSomeOn { a : true, b: 0 };

  x = !x;

  assert_eq!( x.a, true );
  assert_eq!( x.b, 255 );
}
