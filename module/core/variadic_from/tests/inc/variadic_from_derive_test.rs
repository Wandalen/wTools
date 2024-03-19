#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn from_named_fields()
{
  use the_module::prelude::*;

  #[ derive( Debug, PartialEq, the_module::VariadicFrom ) ]
  struct StructNamedFields
  {
    a : i32,
    b : i32,
    c : i32,
    d : i32,
  }

  include!( "./only_test/variadic_from_named.rs" );
}

//

#[ test ]
fn from_tuple()
{
  use the_module::prelude::*;

  #[ derive( Debug, PartialEq, the_module::VariadicFrom ) ]
  struct StructTuple( i32, i32, i32, i32 );

  include!( "./only_test/variadic_from_tuple.rs" );
}

//

#[ test ]
fn sample()
{
  use the_module::exposed::*;

  #[ derive( Debug, PartialEq, the_module::VariadicFrom ) ]
  struct MyStruct
  {
    a : i32,
    b : i32,
  }

  let got : MyStruct = from!();
  let exp = MyStruct { a : 0, b : 0 };
  a_id!( got, exp );

  let got : MyStruct = from!( 13 );
  let exp = MyStruct { a : 13, b : 13 };
  a_id!( got, exp );

}

// qqq : add to examples and to readme
