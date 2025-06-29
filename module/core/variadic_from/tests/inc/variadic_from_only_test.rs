/// This file contains shared test logic for `variadic_from` manual and derive tests.

use crate::the_module; // Import the alias for the crate

#[ test ]
fn basic_test()
{
  let x : MyStruct = the_module::from!();
  assert_eq!( x.a, 0 );
  assert_eq!( x.b, 0 );

  // The `from!(T1)` case for MyStruct (two fields) is handled by manual implementation in Readme,
  // not directly by the derive macro for a two-field struct.
  let x_from_i32 : MyStruct = the_module::from!( 20 );
  assert_eq!( x_from_i32.a, 20 );
  assert_eq!( x_from_i32.b, 20 );

  let x_from_i32_i32 : MyStruct = the_module::from!( 30, 40 );
  assert_eq!( x_from_i32_i32.a, 30 );
  assert_eq!( x_from_i32_i32.b, 40 );
}

#[ test ]
fn named_field_test()
{
  let x : NamedStruct = the_module::from!( 10 );
  assert_eq!( x.field, 10 );

  let x_from_f32 : NamedStruct = the_module::from!( 30.0 );
  assert_eq!( x_from_f32.field, 30 );
}

#[ test ]
fn three_field_struct_test()
{
  let x : ThreeFieldStruct = the_module::from!();
  assert_eq!( x.x, 0 );
  assert_eq!( x.y, 0 );
  assert_eq!( x.z, 0 );

  let x_from_i32 : ThreeFieldStruct = the_module::from!( 100 );
  assert_eq!( x_from_i32.x, 100 );
  assert_eq!( x_from_i32.y, 100 );
  assert_eq!( x_from_i32.z, 100 );

  let x_from_i32_i32 : ThreeFieldStruct = the_module::from!( 100, 200 );
  assert_eq!( x_from_i32_i32.x, 100 );
  assert_eq!( x_from_i32_i32.y, 200 );
  assert_eq!( x_from_i32_i32.z, 200 );

  let x_from_i32_i32_i32 : ThreeFieldStruct = the_module::from!( 100, 200, 300 );
  assert_eq!( x_from_i32_i32_i32.x, 100 );
  assert_eq!( x_from_i32_i32_i32.y, 200 );
  assert_eq!( x_from_i32_i32_i32.z, 300 );
}