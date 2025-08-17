// MRE test for E0309 lifetime constraint error (should be FIXED)
// This test ensures we don't regress on the main type-only struct fix

use super::*;

#[ derive( Debug, PartialEq ) ]
pub struct MREProperty<T> {
  value: T,
}

// Test that should NOT have E0309 "parameter type T may not live long enough" error
#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct TypeOnlyE0309Fixed<T> where T: core::hash::Hash + core::cmp::Eq {
  pub name: String,
  pub properties: collection_tools::HashMap<T, MREProperty<T>>,
}

#[ test ]
fn test_type_only_e0309_fixed() {
  let mut map = collection_tools::HashMap::new();
  map.insert(42, MREProperty { value: 42 });
  
  let instance = TypeOnlyE0309Fixed::<i32>::former()
    .name("test".to_string())
    .properties(map)
    .form();
  
  assert_eq!(instance.name, "test");
  assert_eq!(instance.properties.len(), 1);
}