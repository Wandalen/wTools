fn assert_contravariant( x: ContravariantType< &dyn Fn( &'static str ) -> String > ) -> String
{
  ( x.a )( "test" )
}

#[test]
fn contravariant()
{
  let x_fn: &dyn for< 'a > Fn( &'a str ) -> String = &| s: &str |
  {
    format!( "x_fn: {s}" )
  };

  let x: ContravariantType< &dyn for< 'a > Fn( &'a str ) -> String > = ContravariantType { a: x_fn, _phantom: Default::default() };
  let value = assert_contravariant(x);

  assert_eq!( value, String::from( "x_fn: test" ) );
}
