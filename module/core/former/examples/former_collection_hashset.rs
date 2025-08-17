
//! Example demonstrating `HashSet` construction using Former with subforming capabilities.

#![allow(missing_docs)]

//
// This example demonstrates the use of the `Former` to build a `collection_tools::HashSet` through subforming.
//

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
  use collection_tools::{HashSet, hset};

  #[ derive( Debug, PartialEq, former::Former ) ]
  pub struct StructWithSet {
    #[ subform_collection( definition = former::HashSetDefinition ) ]
    set: HashSet<&'static str>,
  }

  let instance = StructWithSet::former().set().add("apple").add("banana").end().form();

  assert_eq!(
    instance,
    StructWithSet {
      set: hset!["apple", "banana"]
    }
  );
  dbg!(instance);
}
