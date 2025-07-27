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
  #[derive(Debug, PartialEq, Default, former::Former)]
  #[debug]
  pub struct StructWithVec {
    #[subform_collection( definition = former::VectorDefinition )]
    vec: Vec<String>,
  }

  let instance = StructWithVec::former().vec().add("apple".to_string()).add("banana".to_string()).end().form();

  assert_eq!(
    instance,
    StructWithVec {
      vec: vec!["apple".to_string(), "banana".to_string()]
    }
  );
  dbg!(instance);
}
