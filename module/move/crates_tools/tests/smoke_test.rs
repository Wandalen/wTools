#![allow(missing_docs)]

#[test]
fn local_smoke_test() {
  println!("Local smoke test passed");
}

#[ignore]
#[test]
fn published_smoke_test() {
  println!("Published smoke test passed");
}
