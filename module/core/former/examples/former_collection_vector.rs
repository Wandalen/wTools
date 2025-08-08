
//! This example demonstrates how to employ the `Former` to configure a `Vec` using a collection setter in a structured manner.

#![allow(missing_docs)]

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
  #[ cfg( feature = "enabled" ) ]
  use former_meta::Former;
  // use former as the_module; // Commented out - unused import

  #[ derive( Default, Debug, PartialEq, Former ) ]
  pub struct Struct1 {
    #[ subform_collection( definition = former::VectorDefinition ) ]
    vec_1: Vec<String>,
  }

  let instance = Struct1::former().vec_1().add("apple".to_string()).add("banana".to_string()).end().form();

  assert_eq!(
    instance,
    Struct1 {
      vec_1: vec!["apple".to_string(), "banana".to_string()],
    }
  );
  dbg!(instance);
}
