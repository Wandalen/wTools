#[ test ]
fn inner_from()
{
  let a = StructTupleEmpty();
  let exp = ();
  let got : () = a.into();
  assert_eq!(got, exp);
}
