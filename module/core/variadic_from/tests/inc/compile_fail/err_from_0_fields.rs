//! This test ensures that `VariadicFrom` derive fails for structs with 0 fields.

use variadic_from::VariadicFrom;
use variadic_from::from;

#[ derive( VariadicFrom ) ]
struct MyStruct;

fn main()
{
  let _x = from!( 1 ); // This should cause a compile error
}