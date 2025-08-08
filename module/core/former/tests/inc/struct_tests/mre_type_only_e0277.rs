// MRE test for E0277 trait bound error in type-only struct FormerBegin
// This test ensures the trait bounds are properly propagated in FormerBegin implementations

use super::*;

#[ derive( Debug, PartialEq ) ]
pub struct TypeProperty<T> {
  value: T,
}

// Minimal reproducible example of E0277 trait bound error
#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct TypeOnlyMRE<T> where T: core::hash::Hash + core::cmp::Eq {
  pub name: String,
  pub data: collection_tools::HashMap<T, TypeProperty<T>>,
}

#[ test ]
fn test_type_only_mre() {
  let instance = TypeOnlyMRE::<i32>::former()
    .name("test".to_string())
    .data(collection_tools::HashMap::new())
    .form();
  assert_eq!(instance.name, "test");
}