use super :: *;

#[ test ]
fn test_named_struct1()
{
  let instance = NamedStruct1 { field1: 1 };
  let expected = NamedStruct1 { field1: 1 };
  assert_eq!( instance, expected );
}

#[ test ]
fn test_named_struct2()
{
  let instance = NamedStruct2 { field1: 1, field2: true };
  let expected = NamedStruct2 { field1: 1, field2: true };
  assert_eq!( instance, expected );
}