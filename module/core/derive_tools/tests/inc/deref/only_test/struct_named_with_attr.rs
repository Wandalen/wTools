use test_tools ::a_id;

#[ test ]
fn deref_test()
{
  let got = StructNamedWithAttr { a: "hello".to_string(), b: 13 };
  let exp = 13;
  a_id!( *got, exp );
}