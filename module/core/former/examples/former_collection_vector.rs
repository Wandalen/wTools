//!
//! This example demonstrates how to employ the `Former` to configure a `Vec` using a collection setter in a structured manner.
//!

#[cfg(not(all(
  feature = "enabled",
  feature = "derive_former",
  any(feature = "use_alloc", not(feature = "no_std"))
)))]
fn main() {}
#[cfg(all(
  feature = "enabled",
  feature = "derive_former",
  any(feature = "use_alloc", not(feature = "no_std"))
))]
fn main() {
  use former::Former;

  #[derive(Debug, PartialEq, Default, Former)]
  pub struct StructWithVec {
    vec: Vec<String>,
  }

  let instance = StructWithVec::former().vec(vec!["apple".to_string(), "banana".to_string()]).form();

  assert_eq!(
    instance,
    StructWithVec {
      vec: vec!["apple".to_string(), "banana".to_string()]
    }
  );
  dbg!(instance);
}
