#![ allow( unused_imports ) ]
#![ allow( dead_code ) ]

use test_tools::prelude::*;

// Test for UnitStruct
#[ test ]
fn test_unit_struct()
{
  let instance = UnitStruct::new();
  // No fields to assert, just ensure it compiles and can be constructed
}

// Test for TupleStruct1
#[ test ]
fn test_tuple_struct1()
{
  let instance = TupleStruct1::new( 123 );
  assert_eq!( instance.0, 123 );
}

// Test for TupleStruct2
#[ test ]
fn test_tuple_struct2()
{
  let instance = TupleStruct2::new( 123, 456 );
  assert_eq!( instance.0, 123 );
  assert_eq!( instance.1, 456 );
}

// Test for NamedStruct1
#[ test ]
fn test_named_struct1()
{
  let instance = NamedStruct1::new( 789 );
  assert_eq!( instance.field1, 789 );
}

// Test for NamedStruct2
#[ test ]
fn test_named_struct2()
{
  let instance = NamedStruct2::new( 10, 20 );
  assert_eq!( instance.field1, 10 );
  assert_eq!( instance.field2, 20 );
}