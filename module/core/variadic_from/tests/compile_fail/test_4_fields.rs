// tests/compile_fail/test_4_fields.rs

#[ allow( dead_code ) ]
#[ derive( variadic_from::VariadicFrom ) ]
struct Test4FieldsNamed
{
  a : i32,
  b : i32,
  c : i32,
  d : i32,
}