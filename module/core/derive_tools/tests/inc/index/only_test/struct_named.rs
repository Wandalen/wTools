
#[ test ]
fn index()
{
  let x = StructNamed { a : true};
  let exp = true;
  let got = x[0];

  assert_eq!(got, exp);
}

