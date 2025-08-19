#[ allow( unused_imports ) ]
#[ allow( dead_code ) ]
#[ allow( unused_variables ) ]

use test_tools::*;

// Test for UnitStruct
#[ test ]
fn test_unit_struct()
{
  let instance = UnitStruct;
  let not_instance = !instance;
  // For unit structs, Not usually returns Self, so no change in value
  let _ = not_instance;
}

// Test for TupleStruct1
#[ test ]
fn test_tuple_struct1()
{
  let instance = TupleStruct1( true );
  let not_instance = !instance;
  assert_eq!( not_instance.0, false );

  let instance = TupleStruct1( false );
  let not_instance = !instance;
  assert_eq!( not_instance.0, true );
}

// Test for NamedStruct1
#[ test ]
fn test_named_struct1()
{
  let instance = NamedStruct1 { field1 : true };
  let not_instance = !instance;
  assert_eq!( not_instance.field1, false );

  let instance = NamedStruct1 { field1 : false };
  let not_instance = !instance;
  assert_eq!( not_instance.field1, true );
}