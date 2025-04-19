#[ test ]
fn component_from()
{
  let t1 = TupleStruct( 42, "Hello".to_string() );

  // Test converting to i32
  let got_i32 : i32 = ( &t1 ).into();
  let exp_i32 : i32 = 42;
  assert_eq!( got_i32, exp_i32 );

  // Test converting to String
  let got_string : String = ( &t1 ).into();
  let exp_string : String = "Hello".to_string();
  assert_eq!( got_string, exp_string );
}