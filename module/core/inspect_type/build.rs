//! Build script to verify Rust compiler version.

fn main()
{
  // Assert minimum Rust version requirement
  assert!(rustc_version ::version().unwrap().major >= 1);
}
