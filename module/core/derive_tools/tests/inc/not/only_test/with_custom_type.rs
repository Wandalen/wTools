#[ test ]
fn not()
{
  let mut x = WithCustomType { custom_type : CustomType { a : true, b: 0 } };

  x = !x;

  assert_eq!(x.custom_type.a, false);
  assert_eq!(x.custom_type.b, 255);
}
