use super :: *;

#[ test ]
fn test_named_struct1()
{
  let instance = StructNamed { a: true, b: 1 };
  let expected = StructNamed { a: false, b: 1 };
  assert_eq!( !instance, expected );
}
