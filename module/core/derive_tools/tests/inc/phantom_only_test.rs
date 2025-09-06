#[ allow( unused_imports ) ]
#[ allow( dead_code ) ]
use test_tools::*;

use crate::inc::phantom_tests::struct_named::NamedStruct1 as NamedStruct1Derive;
use crate::inc::phantom_tests::struct_named::NamedStruct2 as NamedStruct2Derive;
use crate::inc::phantom_tests::struct_named_manual::NamedStruct1 as NamedStruct1Manual;
use crate::inc::phantom_tests::struct_named_manual::NamedStruct2 as NamedStruct2Manual;

// Test for NamedStruct1
#[ test ]
fn test_named_struct1()
{
  let _ = NamedStruct1Derive { field1 : 123 };
  let _: PhantomData< i32 > = PhantomData;
  let _ = NamedStruct1Manual { field1 : 123 };
  let _: PhantomData< i32 > = PhantomData;
}

// Test for NamedStruct2
#[ test ]
fn test_named_struct2()
{
  let _ = NamedStruct2Derive { field1 : 123, field2 : true };
  let _: PhantomData< ( i32, bool ) > = PhantomData;
  let _ = NamedStruct2Manual { field1 : 123, field2 : true };
  let _: PhantomData< ( i32, bool ) > = PhantomData;
}