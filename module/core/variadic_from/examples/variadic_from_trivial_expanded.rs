//! This example demonstrates the use of the `variadic_from` macro to implement flexible
//! constructors for a struct, allowing it to be instantiated from different numbers of
//! arguments or tuples. It also showcases how to derive common traits like `Debug`,
//! `PartialEq`, `Default`, and `VariadicFrom` for the struct.

#[ cfg( not( all(feature = "enabled", feature = "type_variadic_from" ) ) ) ]
fn main(){}
#[ cfg( all( feature = "enabled", feature = "type_variadic_from" ) )]
fn main()
{
  use variadic_from::exposed::*;

  // Define a struct `MyStruct` with fields `a` and `b`.
  // The struct derives common traits like `Debug`, `PartialEq`, `Default`
  // `VariadicFrom` defined manually.
  #[ derive( Debug, PartialEq, Default ) ]
  struct MyStruct
  {
    a : i32,
    b : i32,
  }

  // Implement the `From1` trait for `MyStruct`, which allows constructing a `MyStruct` instance
  // from a single `i32` value by assigning it to both `a` and `b` fields.
  impl From1< i32 > for MyStruct
  {
    fn from1( a : i32 ) -> Self { Self { a, b : a } }
  }

  // == begin of generated

  impl From2< i32, i32 > for MyStruct
  {
    fn from2( a : i32, b : i32 ) -> Self { Self{ a : a, b : b } }
  }

  impl From< ( i32, i32 ) > for MyStruct
  {
    #[ inline( always ) ]
    fn from( ( a, b ) : ( i32, i32 ) ) -> Self
    {
      Self::from2( a, b )
    }
  }

  // == end of generated

  let got : MyStruct = from!();
  let exp = MyStruct { a : 0, b : 0 };
  assert_eq!( got, exp );

  let got : MyStruct = from!( 13 );
  let exp = MyStruct { a : 13, b : 13 };
  assert_eq!( got, exp );

  let got : MyStruct = from!( 13, 14 );
  let exp = MyStruct { a : 13, b : 14 };
  assert_eq!( got, exp );

  dbg!( exp );
  //> MyStruct {
  //>   a : 13,
  //>   b : 14,
  //> }

}
