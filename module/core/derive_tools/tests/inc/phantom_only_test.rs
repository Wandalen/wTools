#[ allow( unused_imports ) ]
#[ allow( dead_code ) ]
use test_tools::prelude::*;

use crate::inc::phantom_tests::struct_named::NamedStruct1 as NamedStruct1Derive;
use crate::inc::phantom_tests::struct_named::NamedStruct2 as NamedStruct2Derive;
use crate::inc::phantom_tests::struct_named_manual::NamedStruct1 as NamedStruct1Manual;
use crate::inc::phantom_tests::struct_named_manual::NamedStruct2 as NamedStruct2Manual;

// Test for NamedStruct1
#[ test ]
fn test_named_struct1()
{
  let _instance = NamedStruct1Derive { field1 : 123 };
  let _phantom_data : PhantomData< i32 > = PhantomData;
  let _instance_manual = NamedStruct1Manual { field1 : 123 };
  let _phantom_data_manual : PhantomData< i32 > = PhantomData;
}

// Test for NamedStruct2
#[ test ]
fn test_named_struct2()
{
  let _instance = NamedStruct2Derive { field1 : 123, field2 : true };
  let _phantom_data : PhantomData< ( i32, bool ) > = PhantomData;
  let _instance_manual = NamedStruct2Manual { field1 : 123, field2 : true };
  let _phantom_data_manual : PhantomData< ( i32, bool ) > = PhantomData;
}