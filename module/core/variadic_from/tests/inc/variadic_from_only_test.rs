/// This file contains shared test logic for `variadic_from` manual and derive tests.

#[ test ]
fn basic_test()
{
  let x = MyStruct( 10 );
  assert_eq!( x.0, 10 );

  let x_from_i32 : MyStruct = 20.into();
  assert_eq!( x_from_i32.0, 20 );

  let x_from_f32 : MyStruct = 30.0.into();
  assert_eq!( x_from_f32.0, 30 );
}

#[ test ]
fn named_field_test()
{
  let x = NamedStruct { field : 10 };
  assert_eq!( x.field, 10 );

  let x_from_i32 : NamedStruct = 20.into();
  assert_eq!( x_from_i32.field, 20 );

  let x_from_f32 : NamedStruct = 30.0.into();
  assert_eq!( x_from_f32.field, 30 );
}