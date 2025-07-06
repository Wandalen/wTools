// tests/compile_fail/test_from_macro_too_many_args.rs

#[ allow( dead_code ) ]
fn test_from_macro_too_many_args()
{
  let _ = variadic_from::from!( 1, 2, 3, 4 );
}