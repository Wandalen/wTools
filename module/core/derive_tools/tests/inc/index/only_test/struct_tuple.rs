#[test ]
fn index()
{
  let x = StructTuple(7);
  let exp = 7;
  let got = x[0];

  assert_eq!(got, exp);

}

