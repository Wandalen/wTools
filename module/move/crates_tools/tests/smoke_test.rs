#![allow(missing_docs)]

#[test]
fn local_smoke_test() {
  println!("Local smoke test passed");
}

#[ignore = "smoke test for published version"]
#[test]
fn published_smoke_test() {
  println!("Published smoke test passed");
}
