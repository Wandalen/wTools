use super :: *;


/// Tests that `as_mut` works for a named struct.
#[ test ]
fn basic()
{
  let mut src = StructNamed { field1: 13 };
  assert_eq!( src.as_mut(), &mut 13 );
  *src.as_mut() = 5;
  assert_eq!( src.as_mut(), &mut 5 );
}