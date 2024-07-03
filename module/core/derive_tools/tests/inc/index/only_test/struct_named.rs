#[test]
fn index() {
  let x = StructNamed { a: true, b: false };
  let exp = (true, false);
  let got = (x[0], x[1]);

  dbg!(exp, got);

  assert_eq!(got, exp);
}
