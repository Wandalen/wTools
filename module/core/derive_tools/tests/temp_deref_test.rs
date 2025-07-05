use core::ops::Deref;

#[ automatically_derived ]
impl core::ops::Deref for MyTuple
{
  type Target = i32;
  #[ inline ]
  fn deref( &self ) -> &i32
  {
    &self.0
  }
}

struct MyTuple(i32); // Define MyTuple here for the test

#[ test ]
fn temp_basic_tuple_deref()
{
  let x = MyTuple( 10 );
  assert_eq!( *x, 10 );
}