#[test]
fn index() {
  let x = StructTuple(7, 12);
  let exp = (7, 12);
  let got = (x[0], x[1]);

  assert_eq!(got, exp);
}
