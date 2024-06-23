#[ test ]
fn inner_from()
{
  let a = EnumUnit::A;
  let exp = ();
  let got : () = a.into();
  assert_eq!(got, exp);
}
