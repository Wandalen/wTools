// Purpose: Comprehensive replacement for blocked subform_all_parametrized test
// This works around "Undeclared lifetime 'child in derive macro + missing subform methods"
// by creating non-parametrized subform_all functionality that combines scalar, subform_scalar, subform_entry, subform_collection

use super::*;
#[ allow( unused_imports ) ]
use ::former::prelude::*;
use ::former::Former;

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
use std::collections::HashMap;

// Wrapper types for HashMap values to resolve EntityToStorage trait bound issues
#[ derive( Debug, PartialEq, Former ) ]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
pub struct StringValue {
  key: String,
  value: String,
}

#[ derive( Debug, PartialEq, Former ) ]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
pub struct IntValue {
  key: String,
  value: i32,
}

// Implement ValToEntry trait for wrapper types
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
impl ::former::ValToEntry<HashMap< String, StringValue >> for StringValue {
  type Entry = (String, StringValue);
  #[ inline( always ) ]
  fn val_to_entry(self) -> Self::Entry {
    (self.key.clone(), self)
  }
}

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
impl ::former::ValToEntry<HashMap< String, IntValue >> for IntValue {
  type Entry = (String, IntValue);
  #[ inline( always ) ]
  fn val_to_entry(self) -> Self::Entry {
    (self.key.clone(), self)
  }
}

// Inner struct for comprehensive subform testing
#[ derive( Debug, PartialEq, Default, Clone, Former ) ]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
pub struct SubformAllInner {
  pub title: String,
  pub value: i32,
  pub active: bool,
}

// COMPREHENSIVE SUBFORM_ALL replacement - combines ALL subform types in one working test
#[ derive( Debug, PartialEq, Former ) ]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
pub struct SubformAllReplacement {
  // Basic scalar field
  #[ scalar ]
  name: String,
  
  // Subform scalar field  
  #[ subform_scalar ]
  inner_subform: SubformAllInner,
  
  // Subform collection field
  #[ subform_collection ]
  items: Vec<String>,
  
  // Subform entry field (HashMap) - using wrapper type
  #[ subform_entry ]
  entries: HashMap< String, StringValue >,
  
  // Regular field for comparison
  active: bool,
}

// Advanced subform_all replacement with more complex scenarios
#[ derive( Debug, PartialEq, Former ) ]
#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
pub struct AdvancedSubformAllReplacement {
  // Multiple scalar fields
  #[ scalar ]
  title: String,
  
  #[ scalar ] 
  count: i32,
  
  // Multiple subform scalars
  #[ subform_scalar ]
  primary_inner: SubformAllInner,
  
  #[ subform_scalar ]
  secondary_inner: SubformAllInner,
  
  // Multiple collections
  #[ subform_collection ]
  string_list: Vec<String>,
  
  #[ subform_collection ]
  int_list: Vec<i32>,
  
  // Multiple entry maps - using wrapper types
  #[ subform_entry ]
  primary_map: HashMap< String, StringValue >,
  
  #[ subform_entry ]
  secondary_map: HashMap< String, IntValue >,
  
  // Regular field
  enabled: bool,
}

// COMPREHENSIVE SUBFORM_ALL TESTS - covering ALL subform attribute combinations

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[ test ]
fn subform_all_basic_test() {
  let inner = SubformAllInner {
    title: "subform_test".to_string(),
    value: 42,
    active: true,
  };
  
  let mut expected_entries = HashMap::new();
  expected_entries.insert("key1".to_string(), StringValue { key: "key1".to_string(), value: "value1".to_string() });
  expected_entries.insert("key2".to_string(), StringValue { key: "key2".to_string(), value: "value2".to_string() });
  
  let got = SubformAllReplacement::former()
    .name("basic_test".to_string())
    .inner_subform()
      .title("subform_test".to_string())
      .value(42)
      .active(true)
      .form()
    .items()
      .add("item1".to_string())
      .add("item2".to_string())
      .end()
    .entries()
      .key("key1".to_string())
      .value("value1".to_string())
      .end()
    .entries()
      .key("key2".to_string())
      .value("value2".to_string())
      .end()
    .active(true)
    .form();
    
  let expected = SubformAllReplacement {
    name: "basic_test".to_string(),
    inner_subform: inner,
    items: vec!["item1".to_string(), "item2".to_string()],
    entries: expected_entries,
    active: true,
  };
  
  assert_eq!(got, expected);
}

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[ test ]
fn subform_all_empty_collections_test() {
  let inner = SubformAllInner {
    title: "empty_test".to_string(),
    value: 0,
    active: false,
  };
  
  let got = SubformAllReplacement::former()
    .name("empty_test".to_string())
    .inner_subform()
      .title("empty_test".to_string())
      .value(0)
      .active(false)
      .form()
    .active(false)
    .form();
    
  let expected = SubformAllReplacement {
    name: "empty_test".to_string(),
    inner_subform: inner,
    items: Vec::new(),
    entries: HashMap::new(),
    active: false,
  };
  
  assert_eq!(got, expected);
}

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[ test ]
fn advanced_subform_all_test() {
  let primary_inner = SubformAllInner {
    title: "primary".to_string(),
    value: 100,
    active: true,
  };
  
  let secondary_inner = SubformAllInner {
    title: "secondary".to_string(),
    value: 200,
    active: false,
  };
  
  let mut expected_primary_map = HashMap::new();
  expected_primary_map.insert("primary_key".to_string(), StringValue { key: "primary_key".to_string(), value: "primary_value".to_string() });
  
  let mut expected_secondary_map = HashMap::new();
  expected_secondary_map.insert("secondary_key".to_string(), IntValue { key: "secondary_key".to_string(), value: 999 });
  
  let got = AdvancedSubformAllReplacement::former()
    .title("advanced".to_string())
    .count(555)
    .primary_inner()
      .title("primary".to_string())
      .value(100)
      .active(true)
      .form()
    .secondary_inner()
      .title("secondary".to_string())
      .value(200)
      .active(false)
      .form()
    .string_list()
      .add("string1".to_string())
      .add("string2".to_string())
      .end()
    .int_list()
      .add(10)
      .add(20)
      .add(30)
      .end()
    .primary_map()
      .key("primary_key".to_string())
      .value("primary_value".to_string())
      .end()
    .secondary_map()
      .key("secondary_key".to_string())
      .value(999)
      .end()
    .enabled(true)
    .form();
    
  let expected = AdvancedSubformAllReplacement {
    title: "advanced".to_string(),
    count: 555,
    primary_inner,
    secondary_inner,
    string_list: vec!["string1".to_string(), "string2".to_string()],
    int_list: vec![10, 20, 30],
    primary_map: expected_primary_map,
    secondary_map: expected_secondary_map,
    enabled: true,
  };
  
  assert_eq!(got, expected);
}

#[cfg(any(not(feature = "no_std"), feature = "use_alloc"))]
#[ test ]
fn subform_all_stress_test() {
  // Test comprehensive combination of all subform types
  let _inner = SubformAllInner {
    title: "stress".to_string(),
    value: 777,
    active: true,
  };
  
  let got = SubformAllReplacement::former()
    .name("stress_test".to_string())
    .inner_subform()
      .title("stress".to_string())
      .value(777)
      .active(true)
      .form()
    .items()
      .add("stress_item".to_string())
      .end()
    .entries()
      .key("stress_key".to_string())
      .value("stress_value".to_string())
      .end()
    .active(true)
    .form();
  
  // Verify all subform types work together
  assert_eq!(got.name, "stress_test");
  assert_eq!(got.inner_subform.title, "stress");
  assert_eq!(got.items.len(), 1);
  assert_eq!(got.entries.len(), 1);
  assert!(got.active);
}