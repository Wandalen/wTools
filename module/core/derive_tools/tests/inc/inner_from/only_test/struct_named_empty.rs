#[ test ]
fn inner_from()
{
  let a = StructNamedEmpty{};
  let exp = ();
  let got : () = a.into();
  assert_eq!(got, exp);
}
