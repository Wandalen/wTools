#![ allow( unused_imports ) ]
#![ allow( dead_code ) ]

use test_tools::prelude::*;
use core::ops::IndexMut as _;
use core::ops::Index as _;

// Test for TupleStruct1
#[ test ]
fn test_tuple_struct1()
{
  let mut instance = TupleStruct1( 123 );
  assert_eq!( instance[ 0 ], 123 );
  instance[ 0 ] = 456;
  assert_eq!( instance[ 0 ], 456 );
}

// Test for NamedStruct1
#[ test ]
fn test_named_struct1()
{
  let mut instance = NamedStruct1 { field1 : 789 };
  assert_eq!( instance[ "field1" ], 789 );
  instance[ "field1" ] = 101;
  assert_eq!( instance[ "field1" ], 101 );
}