use core::ops::Deref;
use derive_tools::Deref;

#[ derive( Deref ) ]
struct MyTuple( i32 );

#[ test ]
fn basic_tuple_deref_minimal()
{
  let x = MyTuple( 10 );
  assert_eq!( *x, 10 );
}