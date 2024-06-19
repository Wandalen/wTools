fn assert_covariant< 'a >( x: CovariantType< &'static str > ) -> CovariantType< &'a str >
{
  x
}

#[ test ]
fn covariant()
{
  let x: CovariantType< &'static str > = CovariantType { a: "boo", _phantom: Default::default(), };
  let y: CovariantType< &str > = assert_covariant( x );
  assert_eq!( y.a, "boo" );
}
