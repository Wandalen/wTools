#[ test ]
fn inner_from()
{
  let a = EnumTupleEmpty::A();
  let exp = ();
  let got : () = a.into();
  assert_eq!(got, exp);
}
