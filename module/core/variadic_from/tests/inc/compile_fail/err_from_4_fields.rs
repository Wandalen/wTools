//! This test ensures that `VariadicFrom` derive fails for structs with >3 fields.

use variadic_from::VariadicFrom;
use variadic_from::from;

#[ derive( VariadicFrom ) ]
struct MyStruct( i32, i32, i32, i32 );

fn main()
{
  let _x = from!( 1, 2, 3, 4 ); // This should cause a compile error
}