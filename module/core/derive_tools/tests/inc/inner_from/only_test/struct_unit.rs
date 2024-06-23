#[ test ]
fn inner_from()
{
  let a = StructUnit;
  let exp = ();
  let got : () = a.into();
  assert_eq!(got, exp);
}
