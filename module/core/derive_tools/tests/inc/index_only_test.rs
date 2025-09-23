#[ allow( unused_imports ) ]
#[ allow( dead_code ) ]
#[ allow( unused_variables ) ]

use test_tools :: *;
use core ::ops ::Index as _;

// Test for TupleStruct1
#[ test ]
fn test_tuple_struct1()
{
  let instance = TupleStruct1( 123 );
  assert_eq!( instance[ 0 ], 123 );
}

// Test for NamedStruct1
#[ test ]
fn test_named_struct1()
{
  let instance = NamedStruct1 { field1: 456 };
  assert_eq!( instance[ "field1" ], 456 );
}